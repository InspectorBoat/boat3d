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
#![feature(maybe_uninit_uninit_array, maybe_uninit_slice, maybe_uninit_array_assume_init)]
#![feature(slice_from_ptr_range)]

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
use world::{world::World, section::Section};

use Normal::*;
use crate::{util::gl_helper, world::camera};

const BLOCKS: [BlockState; 3] = [
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
                lef: 0x00, bot: 0x00, dep: 0x0, nor: South,
                rig: 0x00, top: 0x00, tex: 1
            },
            BlockFace {
                lef: 0x00, bot: 0x00, dep: 0x0, nor: West,
                rig: 0x00, top: 0x00, tex: 1
            },
            BlockFace {
                lef: 0x00, bot: 0x00, dep: 0x0, nor: Down,
                rig: 0x00, top: 0x00, tex: 1
            },
            BlockFace {
                lef: 0x00, bot: 0x00, dep: 0xf, nor: North,
                rig: 0x00, top: 0x00, tex: 1
            },
            BlockFace {
                lef: 0x00, bot: 0x00, dep: 0xf, nor: East,
                rig: 0x00, top: 0x00, tex: 1
            },
            BlockFace {
                lef: 0x00, bot: 0x00, dep: 0xf, nor: Up,
                rig: 0x00, top: 0x00, tex: 1
            },
        ]),
        otherFaces: BlockState::NONE
    },
    BlockState {
        block: Block { name: "brick_stairs" },
        model: BlockModel([
            BlockFace {
                lef: 0x00, bot: 0x00, dep: 0x0, nor: South,
                rig: 0x00, top: 0x08, tex: 2
            },
            BlockFace {
                lef: 0x00, bot: 0x00, dep: 0x0, nor: West,
                rig: 0x00, top: 0x08, tex: 2
            },
            BlockFace {
                lef: 0x00, bot: 0x00, dep: 0x0, nor: Down,
                rig: 0x00, top: 0x00, tex: 2
            },
            BlockFace {
                lef: 0x00, bot: 0x00, dep: 0xf, nor: North,
                rig: 0x00, top: 0x00, tex: 2
            },
            BlockFace {
                lef: 0x00, bot: 0x00, dep: 0xf, nor: East,
                rig: 0x00, top: 0x08, tex: 2
            },
            BlockFace {
                lef: 0x08, bot: 0x00, dep: 0xf, nor: Up,
                rig: 0x00, top: 0x00, tex: 2
            },
        ]),
        otherFaces: [
            0, 1, 0xffff, 0xffff, 2, 3
        ]
    }
];

const OTHER_FACES: [(BlockFace, bool); 4] = [
    (BlockFace {
        lef: 0x00, bot: 0x08, dep: 0x8, nor: South,
        rig: 0x00, top: 0x00, tex: 1
    }, false),
    (BlockFace {
        lef: 0x08, bot: 0x08, dep: 0x0, nor: West,
        rig: 0x00, top: 0x00, tex: 1
    }, false),
    (BlockFace {
        lef: 0x08, bot: 0x08, dep: 0xf, nor: East,
        rig: 0x00, top: 0x00, tex: 1
    }, false),
    (BlockFace {
        lef: 0x00, bot: 0x00, dep: 0x7, nor: Up,
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
    world.mesh_all();
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
        world.draw();

        window.swap_buffers();

        if frames % 100 == 0 {
            frames = 1;
            
            println!("fps: {}", 1000.0 / (start.elapsed().as_millis() as f64 / 100.0));
            start = std::time::Instant::now();
        } else { frames += 1; }
    }
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
    let speed = 5.0 * (if *keys.get(&Key::LeftControl).unwrap_or(&false) { 10.0 } else { 1.0 });
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