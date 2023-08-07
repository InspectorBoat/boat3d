#![allow(incomplete_features)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(unused_parens)]
#![allow(unused_unsafe)]
#![allow(unused_imports)]
#![allow(unreachable_code)]
#![allow(unused_mut)]
#![feature(slice_as_chunks)]
#![feature(pointer_byte_offsets)]
#![feature(adt_const_params)]
#![feature(new_uninit)]
#![feature(raw_ref_op)]
#![feature(result_option_inspect)]
#![feature(int_roundings)]
#![feature(portable_simd)]
#![feature(slice_from_ptr_range)]
#![feature(let_chains)]

mod block;
mod world;
mod gl_util;
mod render;
mod entity;

use std::{collections::HashMap, ptr, os::raw::c_void, hint::{black_box, unreachable_unchecked}, time::SystemTime, mem, slice, cell::RefCell, fs::File};
use std::env;
use cgmath::Vector3;
use cgmath_culling::{BoundingBox, Intersection};
use gl::{types, FramebufferParameteri};
use glfw::{Context, Window, Action, Key};
use jni::{JNIEnv, objects::{JClass, JByteBuffer, ReleaseMode, JPrimitiveArray, JObject}, sys::{jlong, jint, jchar, jcharArray, jshort, jobject}};
use world::{world::World, section::Section, blockpos::BlockPos};

// Loads a 16-tall chunk column
#[no_mangle]
pub extern "system" fn Java_net_boat3d_NativeWorld_nativeLoadChunk<'local>(
    mut env: JNIEnv<'local>,
    class: JClass<'local>,
    world: jlong,
    section_x: jint,
    section_z: jint
) { unsafe {
    let world = &mut *(world as *mut World);
    for section_y in 0..16 {
        let mut section: Box<Section> = Box::new_zeroed().assume_init();
        section.set_pos(Vector3 {
            x: section_x,
            y: section_y,
            z: section_z,
        });
        world.add_section(section);
    }
    println!("LOAD section_x:{section_x} section_z:{section_z}, total: {}", world.sections.len());
} }
// Unloads a 16-tall chunk column
#[no_mangle]
pub extern "system" fn Java_net_boat3d_NativeWorld_nativeUnloadChunk<'local>(
    mut env: JNIEnv<'local>,
    class: JClass<'local>,
    world: jlong,
    section_x: jint,
    section_z: jint
) { unsafe {
    let world = &mut *(world as *mut World);
    for section_y in 0..16 {
        world.remove_section(Vector3 { x: section_x, y: section_y, z: section_z });
    }
    println!("UNLOAD section_x:{section_x} section_z:{section_z}, total: {}", world.sections.len());
} }

// Sets a chunk section
#[no_mangle]
pub extern "system" fn Java_net_boat3d_NativeWorld_nativeSetChunkSection<'local>(
    mut env: JNIEnv<'local>,
    class: JClass<'local>,
    world: jlong,
    section_x: jint,
    section_y: jint,
    section_z: jint,
    block_data: jcharArray
) { unsafe {
    let world = &mut *(world as *mut World);
    let section = world.sections.get_mut(&Vector3 { x: section_x, y: section_y, z: section_z }).unwrap();
    let block_data = mem::transmute::<jcharArray, JPrimitiveArray<jshort>>(block_data);
    let block_data = env.get_array_elements_critical(&block_data, ReleaseMode::NoCopyBack);
    
    if let Ok(block_data) = block_data {
        let raw_block_data = &*(block_data.as_ptr() as *const [i16; 4096]);
        for i in 0..4096 {
            let y = i >> 8;
            let z = i >> 4 & 0xf;
            let x = i & 0xf;
            section.blocks[BlockPos::new(x, y, z).index] = if raw_block_data[i] == 0 { 0 } else { 1 };
        }
    }
    println!("SET section_x:{section_x} section_y:{section_y} section_z:{section_z}, total: {}", world.sections.len());
} }

// Updates a single block
#[no_mangle]
pub extern "system" fn Java_net_boat3d_NativeWorld_nativeUpdateBlock<'local>(
    mut env: JNIEnv<'local>,
    class: JClass<'local>,
    world: jlong,
    x: jint,
    y: jint,
    z: jint,
    block: jchar
) { unsafe {
    let world = &mut *(world as *mut World);
    if let Some(chunk) = world.sections.get_mut(&Vector3 { x: x / 16, y: y / 16, z: z / 16 }) {
        let pos = BlockPos::new(x & 0xf, y & 0xf, z & 0xf);
        let block = if block == 0 { 0 } else { 1 };
        if chunk.blocks[pos.index] != block {
            chunk.blocks[pos.index] = block;
        }
    }
    println!("SETBLOCK x:{x} y:{y} z:{z}, total: {}", world.sections.len());
} }

// Creates a new world
#[no_mangle]
pub extern "system" fn Java_net_boat3d_NativeWorld_nativeCreateWorld<'local>(
    mut env: JNIEnv<'local>,
    class: JClass<'local>,
    worldWrapper: JObject
) -> jlong { unsafe {
    let mut world: Box<World> = Box::from(World::new());

    // let geometry_staging_byte_buffer = env.new_direct_byte_buffer((&raw mut *world.geometry_staging_buffer.buffer) as *mut u8, 262144);
    // env.set_field(&worldWrapper, "geometryStagingBuffer", "Ljava/nio/ByteBuffer;", (&geometry_staging_byte_buffer.unwrap()).into());

    // let light_staging_buffer = env.new_direct_byte_buffer((&raw mut *world.light_staging_buffer.buffer) as *mut u8, 262144);
    // env.set_field(&worldWrapper, "lightStagingBuffer", "Ljava/nio/ByteBuffer;", (&light_staging_buffer.unwrap()).into());

    return Box::into_raw(world) as i64;
} }

// Deletes a world
#[no_mangle]
pub extern "system" fn Java_net_boat3d_NativeWorld_nativeDeleteWorld<'local>(
    mut env: JNIEnv<'local>,
    class: JClass<'local>,
    world: jlong
) { unsafe {
    let world = Box::from_raw(world as *mut World);
    mem::drop(world);
} }

#[no_mangle]
pub extern "system" fn Java_net_boat3d_NativeWorld_nativeMeshChunkSection<'local>(
    mut env: JNIEnv<'local>,
    class: JClass<'local>,
    world: jlong,
    chunkX: jint,
    chunkY: jint,
    chunkZ: jint
) { unsafe {
    let world = &mut *(world as *mut World);
    world.mesh_all();
} }

#[no_mangle]
pub extern "system" fn Java_net_boat3d_NativeWorld_nativeGenerateDrawCommands<'local>(
    mut env: JNIEnv<'local>,
    class: JClass<'local>,
    world: jlong
) { unsafe {
    let world = &*(world as *const World);
} }