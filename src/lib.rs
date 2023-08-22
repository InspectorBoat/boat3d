#![allow(incomplete_features)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(unused_parens)]
#![allow(unused_unsafe)]
#![allow(unused_imports)]
#![allow(unreachable_code)]
#![allow(unused_mut)]
#![allow(non_upper_case_globals)]
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
#![feature(inline_const)]
mod block;
mod world;
mod gl_util;
mod render;
mod entity;
mod cull;

use std::{collections::HashMap, ptr, os::raw::c_void, hint::{black_box, unreachable_unchecked}, time::SystemTime, mem, slice, cell::RefCell, fs::File, str::FromStr};
use std::env;
use cgmath::{Vector3, Euler, Deg, Rad, conv};
use gl::{types, FramebufferParameteri};
use gl_util::{gl_wrapper::{self, STORAGE, PointerStorage}, gl_helper::{WindowStatus, log_if_error}};
use glfw::{Context, Window, Action, Key, Glfw, ffi::glfwGetCurrentContext};
use jni::{JNIEnv, objects::{JClass, JByteBuffer, ReleaseMode, JPrimitiveArray, JObject, JString, JValueOwned, JFieldID}, sys::{jlong, jint, jchar, jcharArray, jshort, jobject, jclass, jdouble, jbyte}, strings::JNIString, signature::ReturnType, descriptors::Desc};
use world::{world::World, section::Section, blockpos::BlockPos, camera::Camera};
use jni::errors::Result;

// loads a 16-tall chunk column
#[no_mangle]
pub extern "system" fn Java_net_boat3d_NativeWorld_nativeLoadChunk<'local>(
    mut env: JNIEnv<'local>,
    class: JClass<'local>,
    gl_pointers: jlong,
    world: jlong,
    section_x: jint,
    section_z: jint
) { unsafe {
    STORAGE = gl_pointers as *mut PointerStorage;
    let world = &mut *(world as *mut World);

    for section_y in 0..16 {
        let mut section: Box<Section> = Box::new(Section::new());
        section.has_light = true;
        section.set_pos(Vector3 {
            x: section_x,
            y: section_y,
            z: section_z,
        });
        world.add_section(section);
    }
    if DEBUG { println!("LOAD section_x:{section_x} section_z:{section_z}, total: {}", world.sections.len()); }
} }
// unloads a 16-tall chunk column
#[no_mangle]
pub extern "system" fn Java_net_boat3d_NativeWorld_nativeUnloadChunk<'local>(
    mut env: JNIEnv<'local>,
    class: JClass<'local>,
    gl_pointers: jlong,
    world: jlong,
    section_x: jint,
    section_z: jint
) { unsafe {
    STORAGE = gl_pointers as *mut PointerStorage;
    let world = &mut *(world as *mut World);

    for section_y in 0..16 {
        world.remove_section(Vector3 { x: section_x, y: section_y, z: section_z });
    }
    if DEBUG { println!("UNLOAD section_x:{section_x} section_z:{section_z}, total: {}", world.sections.len()); }
} }

// sets the block data for a chunk section
#[no_mangle]
pub extern "system" fn Java_net_boat3d_NativeWorld_nativeSetSection<'local>(
    mut env: JNIEnv<'local>,
    class: JClass<'local>,
    gl_pointers: jlong,
    world: jlong,
    section_x: jint,
    section_y: jint,
    section_z: jint,
    block_data: jcharArray
) { unsafe {
    STORAGE = gl_pointers as *mut PointerStorage;
    let world = &mut *(world as *mut World);

    let section_pos = Vector3 { x: section_x, y: section_y, z: section_z };
    let section = world.sections.get_mut(&section_pos).unwrap();
    let block_data = mem::transmute::<jcharArray, JPrimitiveArray<jshort>>(block_data);
    let block_data = env.get_array_elements_critical(&block_data, ReleaseMode::NoCopyBack);
    
    if let Ok(block_data) = block_data {
        let raw_block_data = *(block_data.as_ptr() as *const [i16; 4096]);
        section.set_blocks(convert_block_data(raw_block_data));
    }
    world.mesh_section(section_pos);

    if DEBUG { println!("SET section_x:{section_x} section_y:{section_y} section_z:{section_z}, total: {}", world.sections.len()); }
} }

// updates a single block
#[no_mangle]
pub extern "system" fn Java_net_boat3d_NativeWorld_nativeUpdateBlock<'local>(
    mut env: JNIEnv<'local>,
    class: JClass<'local>,
    gl_pointers: jlong,
    world: jlong,
    x: jint,
    y: jint,
    z: jint,
    block_id: jchar
) { unsafe {
    STORAGE = gl_pointers as *mut PointerStorage;
    let world = &mut *(world as *mut World);

    let section_pos = Vector3 { x: x / 16, y: y / 16, z: z / 16 };
    if let Some(section) = world.sections.get_mut(&section_pos) {
        let block_pos = BlockPos::new(x & 15, y & 15, z & 15);
        let block_id = if block_id == 0 { 0 } else { 1 };
        section.set_block(block_pos, block_id);
    }
    world.mesh_section(section_pos);

    if DEBUG { println!("SETBLOCK x:{x} y:{y} z:{z}, total: {}", world.sections.len()); }
} }

// updates the light level of a single block
#[no_mangle]
pub extern "system" fn Java_net_boat3d_NativeWorld_nativeUpdateBlockLight<'local>(
    mut env: JNIEnv<'local>,
    class: JClass<'local>,
    gl_pointers: jlong,
    world: jlong,
    x: jint,
    y: jint,
    z: jint,
    block_light: jint
) { unsafe {
    STORAGE = gl_pointers as *mut PointerStorage;
    let world = &mut *(world as *mut World);

    let section_pos = Vector3 { x: x / 16, y: y / 16, z: z / 16 };
    if let Some(section) = world.sections.get_mut(&section_pos) {
        let block_pos = BlockPos::new(x & 15, y & 15, z & 15);
        section.set_light(block_pos, block_light as u8);
    }
    world.mesh_section(section_pos);

    if DEBUG { println!("LIGHT x:{x} y:{y} z:{z} val:{block_light}"); }
} }

// creates a new world
#[no_mangle]
pub extern "system" fn Java_net_boat3d_NativeWorld_nativeCreateWorld<'local>(
    mut env: JNIEnv<'local>,
    class: JClass<'local>,
    gl_pointers: jlong
) -> jlong { unsafe {
    STORAGE = gl_pointers as *mut PointerStorage;
    let mut world: Box<World> = Box::from(World::new());
    world.renderer.init(&WindowStatus {
        fill_mode: gl_wrapper::LINE,
        maximized: true,
        width: 600,
        height: 600,
        mouse_captured: true,
        rasterize: false
    });
    // world.generate();
    // world.mesh_all();
    
    return Box::into_raw(world) as i64;
} }
// deletes a world
#[no_mangle]
pub extern "system" fn Java_net_boat3d_NativeWorld_nativeDeleteWorld<'local>(
    mut env: JNIEnv<'local>,
    class: JClass<'local>,
    gl_pointers: jlong,
    world: jlong
) { unsafe {
    STORAGE = gl_pointers as *mut PointerStorage;
    let world = Box::from_raw(world as *mut World);

    world.kill();
} }

// meshes a chunk section
#[no_mangle]
pub extern "system" fn Java_net_boat3d_NativeWorld_nativeMeshSection<'local>(
    mut env: JNIEnv<'local>,
    class: JClass<'local>,
    gl_pointers: jlong,
    world: jlong,
    section_x: jint,
    section_y: jint,
    section_z: jint
) { unsafe {
    STORAGE = gl_pointers as *mut PointerStorage;
    let world = &mut *(world as *mut World);
    world.mesh_section(Vector3 { x: section_x, y: section_y, z: section_z });
} }

// loads OpenGL 4.6 function pointers
#[no_mangle]
pub extern "system" fn Java_net_boat3d_NativeWorld_nativeGetPointers<'local>(
    mut env: JNIEnv<'local>,
    class: JClass<'local>,
    glcontext_class: jclass,
    capabilities: jobject
) -> jlong { unsafe {
    STORAGE = Box::into_raw(Box::new(PointerStorage::new()));
    gl_wrapper::load_with(|symbol| {
        if let Ok(jobject) = env.call_static_method(
            mem::transmute::<_, JClass>(glcontext_class),
            "getFunctionAddress",
            "(Ljava/lang/String;)J",
            &[(&env.new_string(symbol).unwrap()).into()]
        ) {
            if let Ok(j) = jobject.j() {
                return j as *const c_void;
            } else {
                return 0 as *const c_void;
            }
        } else {
            return 0 as *const c_void;
        }
    });
    let _ = env.exception_clear();
    return STORAGE as i64;
} }

// sets camera position and angle
#[no_mangle]
pub extern "system" fn Java_net_boat3d_NativeWorld_nativeSetCamera<'local>(
    mut env: JNIEnv<'local>,
    class: JClass<'local>,
    gl_pointers: jlong,
    world: jlong,
    camera_x: jdouble,
    camera_y: jdouble,
    camera_z: jdouble,
    camera_pitch: jdouble,
    camera_yaw: jdouble,
    window_width: jint,
    window_height: jint
) { unsafe {
    STORAGE = gl_pointers as *mut PointerStorage;
    let world = &mut *(world as *mut World);
    if window_width != world.camera.window_width || window_height != world.camera.window_height {
        world.renderer.make_framebuffer(&WindowStatus {
            fill_mode: gl::LINE,
            maximized: true,
            width: window_width,
            height: window_height,
            mouse_captured: true,
            rasterize: false
        });
    }
    world.camera = Camera {
        prev_mouse: (0.0, 0.0),
        aspect: window_width as f32 / window_height as f32,
        camera_pos: Vector3 { x: camera_x as f32, y: camera_y as f32, z: camera_z as f32 },
        camera_rot: Euler { x: Deg(camera_pitch as f32).into(), y: Deg(camera_yaw as f32).into(), z: Rad(0.0) },
        frustum_pos: Vector3 { x: camera_x as f32, y: camera_y as f32, z: camera_z as f32 },
        frustum_rot: Euler { x: Deg(camera_pitch as f32).into(), y: Deg(camera_yaw as f32).into(), z: Rad(0.0) },
        frustum_frozen: false,
        window_width: window_width,
        window_height: window_height
    };
} }

// renders the world
#[no_mangle]
pub extern "system" fn Java_net_boat3d_NativeWorld_nativeRenderWorld<'local>(
    mut env: JNIEnv<'local>,
    class: JClass<'local>,
    gl_pointers: jlong,
    world: jlong,
    target_framebuffer_id: jint
) { unsafe {
    STORAGE = gl_pointers as *mut PointerStorage;
    let world = &mut *(world as *mut World);

    gl_wrapper::Disable(gl_wrapper::CULL_FACE);

    gl_wrapper::Enable(gl_wrapper::PRIMITIVE_RESTART);
    gl_wrapper::PrimitiveRestartIndex(u32::MAX as u32);
    
    // for (section_pos, section) in world.sections.iter_mut() {
    //     if section.dirty && section.can_mesh() {
    //         section.mesh(&mut world.solid_staging_buffer, &mut world.trans_staging_buffer, &mut world.geometry_buffer_allocator, &mut world.light_staging_buffer, &mut world.light_buffer_allocator);
    //     }
    // }

    world.render(&WindowStatus {
        fill_mode: gl_wrapper::FILL,
        maximized: true,
        width: world.camera.window_width,
        height: world.camera.window_height,
        mouse_captured: true,
        rasterize: false
    }, target_framebuffer_id);
    
    gl_wrapper::UseProgram(0);
    gl_wrapper::Enable(gl_wrapper::CULL_FACE);
    
    gl_wrapper::Disable(gl_wrapper::PRIMITIVE_RESTART);
    
    gl_wrapper::BindFramebuffer(gl_wrapper::FRAMEBUFFER, target_framebuffer_id as u32);
} }

pub fn convert_block_data(raw_block_data: [i16; 4096]) -> [u16; 4096] {
    let mut converted_data = [0; 4096];
    for i in 0..4096 {
        let y = i >> 8;
        let z = i >> 4 & 0xf;
        let x = i & 0xf;
        converted_data[BlockPos::new(x, y, z).index] = if raw_block_data[i] == 0 { 0 } else { 1 };
    }
    return converted_data;
}

pub const DEBUG: bool = false;
/*
    let boolean = token('Z').map(|_| Primitive::Boolean);
    let byte = token('B').map(|_| Primitive::Byte);
    let char_type = token('C').map(|_| Primitive::Char);
    let double = token('D').map(|_| Primitive::Double);
    let float = token('F').map(|_| Primitive::Float);
    let int = token('I').map(|_| Primitive::Int);
    let long = token('J').map(|_| Primitive::Long);
    let short = token('S').map(|_| Primitive::Short);
    let void = token('V').map(|_| Primitive::Void);
 */