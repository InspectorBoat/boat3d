#![allow(incomplete_features)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_must_use)]
#![allow(non_snake_case)]
#![allow(unused_parens)]
#![allow(unused_unsafe)]
#![feature(slice_as_chunks)]
#![feature(pointer_byte_offsets)]
#![feature(adt_const_params)]
#![feature(core_intrinsics)]
#![feature(new_uninit)]
#![feature(raw_ref_op)]
#![feature(result_option_inspect)]
#![feature(int_roundings)]
#![feature(portable_simd)]
mod block;
mod world;
mod util;

use std::{collections::HashMap, ptr, os::raw::c_void, hint::{black_box, unreachable_unchecked}, time::SystemTime, mem};
use std::env;
use block::{blockstate::BlockState, blockface::BlockFace, block::Block, blockface::Normal, blockmodel::BlockModel};
use cgmath::Vector3;
use cgmath_culling::{BoundingBox, Intersection};
use gl::{types, FramebufferParameteri};
use glfw::{Context, Window, Action, Key};
use util::{gl_helper::*, byte_buffer::StagingBuffer};
use world::{world::World, chunk::Chunk};

use crate::{util::gl_helper, world::camera};

static BLOCKS: [BlockState; 3] = [
    BlockState {
        block: Block { name: "air" },
        model: BlockModel([
            BlockFace::NONE,
            BlockFace::NONE,
            BlockFace::NONE,
            BlockFace::NONE,
            BlockFace::NONE,
            BlockFace::NONE
        ]),
        otherFaces: BlockState::NONE
    },
    BlockState {
        block: Block { name: "bricks" },
        model: BlockModel([
            BlockFace {
                lef: 0x00, bot: 0x00, dep: 0x0, nor: Normal::SOUTH,
                rig: 0x00, top: 0x00, tex: 1
            },
            BlockFace {
                lef: 0x00, bot: 0x00, dep: 0x0, nor: Normal::WEST,
                rig: 0x00, top: 0x00, tex: 1
            },
            BlockFace {
                lef: 0x00, bot: 0x00, dep: 0x0, nor: Normal::DOWN,
                rig: 0x00, top: 0x00, tex: 1
            },
            BlockFace {
                lef: 0x00, bot: 0x00, dep: 0xf, nor: Normal::NORTH,
                rig: 0x00, top: 0x00, tex: 1
            },
            BlockFace {
                lef: 0x00, bot: 0x00, dep: 0xf, nor: Normal::EAST,
                rig: 0x00, top: 0x00, tex: 1
            },
            BlockFace {
                lef: 0x00, bot: 0x00, dep: 0xf, nor: Normal::UP,
                rig: 0x00, top: 0x00, tex: 1
            },
        ]),
        otherFaces: BlockState::NONE
    },
    BlockState {
        block: Block { name: "brick_stairs" },
        model: BlockModel([
            BlockFace {
                lef: 0x00, bot: 0x00, dep: 0x0, nor: Normal::SOUTH,
                rig: 0x00, top: 0x08, tex: 2
            },
            BlockFace {
                lef: 0x00, bot: 0x00, dep: 0x0, nor: Normal::WEST,
                rig: 0x00, top: 0x08, tex: 2
            },
            BlockFace {
                lef: 0x00, bot: 0x00, dep: 0x0, nor: Normal::DOWN,
                rig: 0x00, top: 0x00, tex: 2
            },
            BlockFace {
                lef: 0x00, bot: 0x00, dep: 0xf, nor: Normal::NORTH,
                rig: 0x00, top: 0x00, tex: 2
            },
            BlockFace {
                lef: 0x00, bot: 0x00, dep: 0xf, nor: Normal::EAST,
                rig: 0x00, top: 0x08, tex: 2
            },
            BlockFace {
                lef: 0x08, bot: 0x00, dep: 0xf, nor: Normal::UP,
                rig: 0x00, top: 0x00, tex: 2
            },
        ]),
        otherFaces: [
            0, 1, 0xffff, 0xffff, 2, 3
        ]
    }
];

static OTHER_FACES: [(BlockFace, bool); 4] = [
    (BlockFace {
        lef: 0x00, bot: 0x08, dep: 0x8, nor: Normal::SOUTH,
        rig: 0x00, top: 0x00, tex: 1
    }, false),
    (BlockFace {
        lef: 0x08, bot: 0x08, dep: 0x0, nor: Normal::WEST,
        rig: 0x00, top: 0x00, tex: 1
    }, false),
    (BlockFace {
        lef: 0x08, bot: 0x08, dep: 0xf, nor: Normal::EAST,
        rig: 0x00, top: 0x00, tex: 1
    }, false),
    (BlockFace {
        lef: 0x00, bot: 0x00, dep: 0x7, nor: Normal::UP,
        rig: 0x08, top: 0x00, tex: 1
    }, false),
];

fn main() { unsafe {
    // env::set_var("RUST_BACKTRACE", "1");

    let mut glfw = gl_helper::init_glfw();
    let mut status = WindowStatus::new();
    let (mut window, events) = gl_helper::create_window(&status);

    gl_helper::init_gl(&mut window);

    let mut keys: HashMap<glfw::Key, bool> = HashMap::new();
    let mut start = std::time::Instant::now();
    let mut frames = 1;
    
    let mut world = World::new();

    world.generate();
    world.mesh();
    world.make_block_texture();
    world.make_framebuffer(&status);
    world.make_index_buffer();
    world.make_shader_programs();
    world.make_screen_buffer();

    world.geometry_pool.buffer.bind_indexed_target_base(gl::SHADER_STORAGE_BUFFER, 0);
    world.light_pool.buffer.bind_indexed_target_base(gl::SHADER_STORAGE_BUFFER, 1);

    while !window.should_close() {
        glfw.poll_events();
        
        for (_, event) in glfw::flush_messages(&events) {
            handle_window_event(&mut window, &mut world, event, &mut keys, &mut status);
        }
        
        update(&mut world, &mut keys);
        
        gl::PolygonMode(gl::FRONT_AND_BACK, status.fill_mode);
        draw(&mut world);

        window.swap_buffers();

        if frames % 100 == 0 {
            frames = 1;
            
            println!("fps: {}", 1000.0 / (start.elapsed().as_millis() as f64 / 100.0));
            start = std::time::Instant::now();
        } else { frames += 1; }
    }
} }

fn draw(world: &mut World) { unsafe {
    world.geometry_program.as_ref().unwrap().bind();
    world.framebuffer.as_ref().unwrap().bind(gl::FRAMEBUFFER);

    gl::ClearColor(16.0, 16.0, 16.0, 16.0);
    gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
    gl::Enable(gl::DEPTH_TEST);
    // let camera_matrix = world.camera.get_matrix().as_array();
    let camera_matrix: [f32; 16] = *(world.camera.get_matrix().as_ref());
    gl::UniformMatrix4fv(0, 1, gl::FALSE, camera_matrix.as_ptr());

    let frustum = world.camera.get_frustum();
    const ELEMENT_INDICES_PER_QUAD: i32 = 5;
    const BYTES_PER_QUAD: usize = 8;
    const ELEMENTS_PER_QUAD: usize = 4;

    let mut frustum_results: [Intersection; 8 * 8 * 8] = [Intersection::Inside; 512];
    for x in 0..8 {
        for y in 0..8 {
            for z in 0..8 {
                frustum_results[(x << 6) | (y << 3) | (z << 0)] = frustum.test_bounding_box(
                    BoundingBox {
                        min: Vector3 {
                            x: (x * 256 * 4) as f32 - world.camera.frustum_pos.x,
                            y: (y * 256 * 4) as f32 - world.camera.frustum_pos.y,
                            z: (z * 256 * 4) as f32 - world.camera.frustum_pos.z,
                        },
                        max: Vector3 {
                            x: (x * 256 * 4) as f32 - world.camera.frustum_pos.x + 256.0 * 4.0,
                            y: (y * 256 * 4) as f32 - world.camera.frustum_pos.y + 256.0 * 4.0,
                            z: (z * 256 * 4) as f32 - world.camera.frustum_pos.z + 256.0 * 4.0,
                        }
                    }
                );
            }
        }
    }

    for chunk in world.chunks.values() {
        if let Some(geometry_page) = &chunk.geometry_page {
            // if chunk.pos.x >= 8 || chunk.pos.y >= 8 || chunk.pos.z >= 8 { continue; }
            let Some(light_page) = &chunk.light_page else { unreachable_unchecked(); };
            match frustum_results[(((chunk.pos.x / 4) << 6) | ((chunk.pos.y / 4) << 3) | ((chunk.pos.z / 4) << 0)) as usize] {
                Intersection::Inside => {}
                // Intersection::Partial => { if frustum.test_bounding_box(chunk.get_bounding_box(&world.camera)) == Intersection::Outside { continue; } }
                Intersection::Partial => { if frustum.test_sphere(chunk.get_bounding_sphere(&world.camera)) == Intersection::Outside { continue; } }
                Intersection::Outside => { continue; }
            }
            let pos = [chunk.pos.x, chunk.pos.y, chunk.pos.z];
            gl::Uniform3iv(1, 1, &raw const chunk.pos as *const i32);
            gl::Uniform1ui(2, (light_page.start * light_page.block_size() / mem::size_of::<u32>() / 2) as u32);
            gl::DrawElementsBaseVertex(
                gl::TRIANGLE_STRIP,
                chunk.quad_count as i32 * ELEMENT_INDICES_PER_QUAD,
                gl::UNSIGNED_INT,
                ptr::null(),
                (geometry_page.start * geometry_page.block_size() / BYTES_PER_QUAD * ELEMENTS_PER_QUAD) as i32
            );
        }
    }

    world.post_program.as_ref().unwrap().bind();

    gl::PolygonMode(gl::FRONT_AND_BACK, gl::FILL);
    FrameBuffer::clear_bind(gl::FRAMEBUFFER);
    gl::ClearColor(1.0, 1.0, 1.0, 1.0);
    gl::Clear(gl::COLOR_BUFFER_BIT);
    gl::Disable(gl::DEPTH_TEST);

    gl::ActiveTexture(gl::TEXTURE0);
    world.texture_attachment.as_ref().unwrap().bind(gl::TEXTURE_2D);
    gl::ActiveTexture(gl::TEXTURE1);
    world.block_texture.as_ref().unwrap().bind(gl::TEXTURE_2D_ARRAY);
    
    gl::DrawArrays(gl::TRIANGLES, 0, 6);
} }

fn handle_window_event(window: &mut Window, world: &mut World, event: glfw::WindowEvent, keys: &mut HashMap<Key, bool>, status: &mut WindowStatus) { unsafe {
    match event {
        glfw::WindowEvent::Size(width, height) => {
            gl::Viewport(0, 0, width, height);
            (status.width, status.height) = (width, height);
            world.camera.ratio = width as f32 / height as f32;
            world.make_framebuffer(status);
        }
        glfw::WindowEvent::CursorPos(x, y) => {
            if !status.mouse_captured { return }
            let delta = (x - world.camera.prev_mouse.0, y - world.camera.prev_mouse.1);
            if world.camera.prev_mouse != (f64::MAX, f64::MAX) {
                world.camera.camera_rot.yaw += (delta.0 / 500.0) as f32;
                world.camera.camera_rot.pitch += (delta.1 / 500.0) as f32;
            }
            world.camera.prev_mouse = (x, y);
        }
        glfw::WindowEvent::Key(key, scancode, action, modifiers) => {
            keys.insert(key, if action == Action::Release { false } else { true });
            
            if action != Action::Press { return }
            match key {
                Key::Escape => { window.set_should_close(true) }
                Key::X => {
                    status.fill_mode = if status.fill_mode == gl::LINE { gl::FILL } else { gl::LINE };
                }
                Key::Tab => {
                    if status.maximized { window.restore() }
                    else { window.maximize() }
                    status.maximized = !status.maximized;
                }
                Key::F => {
                    world.camera.frustum_frozen = !world.camera.frustum_frozen;
                    if world.camera.frustum_frozen == false {
                        world.camera.camera_pos = world.camera.frustum_pos;
                        world.camera.camera_rot = world.camera.frustum_rot;
                    }
                }
                _ => ()
            }
        }
        glfw::WindowEvent::MouseButton(button, action, modifiers) => {
            if action != Action::Press { return }
            match button {
                glfw::MouseButton::Button1 => {
                    window.set_cursor_mode(glfw::CursorMode::Disabled);
                    status.mouse_captured = true;
                    world.camera.prev_mouse = (f64::MAX, f64::MAX)
                },
                glfw::MouseButton::Button2 => {
                    window.set_cursor_mode(glfw::CursorMode::Normal);
                    status.mouse_captured = false;
                    world.camera.prev_mouse = (f64::MAX, f64::MAX)
                },
                _ => (),
            }
        }
        _ => {}
    }
} }

fn update(world: &mut World, keys: &mut HashMap<Key, bool>) {
    let speed = 1.6 * (if *keys.get(&Key::LeftControl).unwrap_or(&false) { 10.0 } else { 1.0 });
    for (key, pressed) in keys.iter() {
        if *pressed == false { continue; }
        match key {
            Key::W => {
                world.camera.step(0.0, 0.0, -speed as f64);
            }
            Key::S => {
                world.camera.step(0.0, 0.0, speed as f64);
            }
            Key::A => {
                world.camera.step(-speed as f64, 0.0, 0.0);
            }
            Key::D => {
                world.camera.step(speed as f64, 0.0, 0.0);
            }
            Key::Space => {
                world.camera.step(0.0, speed as f64, 0.0);
            }
            Key::LeftShift => {
                world.camera.step(0.0, - speed as f64, 0.0);
            }
            _ => ()
        }
    }
}

/*
 *              X Y Z        U V D
 *              -----        -----
 * SOUTH   ->   0 0 -   ->   X Y Z
 * NORTH   ->   0 0 +   ->   X Y Z
 * WEST    ->   - 0 0   ->   Z Y X
 * EAST    ->   + 0 0   ->   Z Y X
 * DOWN    ->   0 - 0   ->   Z X Y
 * UP      ->   0 + 0   ->   Z X Y
 * 
 * (UVD * t + WH * c + chunk) * look
 */
