#![allow(incomplete_features)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_must_use)]
#![allow(non_snake_case)]
#![feature(unchecked_math)]
#![feature(pointer_byte_offsets)]
#![feature(adt_const_params)]
#![feature(core_intrinsics)]
#![feature(new_uninit)]
#![feature(portable_simd)]
#![feature(raw_ref_op)]
#![allow(overflowing_literals)]
#![allow(unused_unsafe)]
mod block;
mod world;
mod util;

use std::{collections::HashMap, ptr, os::raw::c_void, hint::black_box};

use block::{blockstate::BlockState, blockface::BlockFace, block::Block, blockface::Normal, blockmodel::BlockModel};
use glfw::{Context, Window, Action, Key};
use util::gl_helper::*;
use world::world::World;

use crate::util::gl_helper;

static BLOCKS: [BlockState; 6] = [
    BlockState {
        block: Block { name: "air" },
        model: BlockModel([
            BlockFace::NONE,
            BlockFace::NONE,
            BlockFace::NONE,
            BlockFace::NONE2,
            BlockFace::NONE2,
            BlockFace::NONE2
        ])
    },
    BlockState {
        block: Block { name: "bricks" },
        model: BlockModel([
            BlockFace {
                lef: 0, bot: 0, dep: 0, nor: Normal::SOUTH,
                rig: 0, top: 0, tex: 0
            },
            BlockFace {
                lef: 0, bot: 0, dep: 0, nor: Normal::WEST,
                rig: 0, top: 0, tex: 0
            },
            BlockFace {
                lef: 0, bot: 0, dep: 0, nor: Normal::DOWN,
                rig: 0, top: 0, tex: 0
            },
            BlockFace {
                lef: 0, bot: 0, dep: 15, nor: Normal::NORTH,
                rig: 0, top: 0, tex: 0
            },
            BlockFace {
                lef: 0, bot: 0, dep: 15, nor: Normal::EAST,
                rig: 0, top: 0, tex: 0
            },
            BlockFace {
                lef: 0, bot: 0, dep: 15, nor: Normal::UP,
                rig: 0, top: 0, tex: 0
            },
        ])
    },
    BlockState {
        block: Block { name: "bricks" },
        model: BlockModel([
            BlockFace {
                lef: 0, bot: 0, dep: 0, nor: Normal::SOUTH,
                rig: 0, top: 0, tex: 1
            },
            BlockFace {
                lef: 0, bot: 0, dep: 0, nor: Normal::WEST,
                rig: 0, top: 0, tex: 1
            },
            BlockFace {
                lef: 0, bot: 0, dep: 0, nor: Normal::DOWN,
                rig: 0, top: 0, tex: 1
            },
            BlockFace {
                lef: 0, bot: 0, dep: 15, nor: Normal::NORTH,
                rig: 0, top: 0, tex: 1
            },
            BlockFace {
                lef: 0, bot: 0, dep: 15, nor: Normal::EAST,
                rig: 0, top: 0, tex: 1
            },
            BlockFace {
                lef: 0, bot: 0, dep: 15, nor: Normal::UP,
                rig: 0, top: 0, tex: 1
            },
        ])
    },
    BlockState {
        block: Block { name: "bricks" },
        model: BlockModel([
            BlockFace {
                lef: 0, bot: 0, dep: 0, nor: Normal::SOUTH,
                rig: 0, top: 0, tex: 2
            },
            BlockFace {
                lef: 0, bot: 0, dep: 0, nor: Normal::WEST,
                rig: 0, top: 0, tex: 2
            },
            BlockFace {
                lef: 0, bot: 0, dep: 0, nor: Normal::DOWN,
                rig: 0, top: 0, tex: 2
            },
            BlockFace {
                lef: 0, bot: 0, dep: 15, nor: Normal::NORTH,
                rig: 0, top: 0, tex: 2
            },
            BlockFace {
                lef: 0, bot: 0, dep: 15, nor: Normal::EAST,
                rig: 0, top: 0, tex: 2
            },
            BlockFace {
                lef: 0, bot: 0, dep: 15, nor: Normal::UP,
                rig: 0, top: 0, tex: 2
            },
        ])
    },
    BlockState {
        block: Block { name: "bricks" },
        model: BlockModel([
            BlockFace {
                lef: 0, bot: 0, dep: 0, nor: Normal::SOUTH,
                rig: 0, top: 0, tex: 3
            },
            BlockFace {
                lef: 0, bot: 0, dep: 0, nor: Normal::WEST,
                rig: 0, top: 0, tex: 3
            },
            BlockFace {
                lef: 0, bot: 0, dep: 0, nor: Normal::DOWN,
                rig: 0, top: 0, tex: 3
            },
            BlockFace {
                lef: 0, bot: 0, dep: 15, nor: Normal::NORTH,
                rig: 0, top: 0, tex: 3
            },
            BlockFace {
                lef: 0, bot: 0, dep: 15, nor: Normal::EAST,
                rig: 0, top: 0, tex: 3
            },
            BlockFace {
                lef: 0, bot: 0, dep: 15, nor: Normal::UP,
                rig: 0, top: 0, tex: 3
            },
        ])
    },
    BlockState {
        block: Block { name: "bricks" },
        model: BlockModel([
            BlockFace {
                lef: 0x10, bot: 0x10, dep: 0, nor: Normal::SOUTH,
                rig: 0x10, top: 0x10, tex: 4
            },
            BlockFace {
                lef: 0x10, bot: 0x10, dep: 0, nor: Normal::WEST,
                rig: 0x10, top: 0x10, tex: 4
            },
            BlockFace {
                lef: 0x10, bot: 0x10, dep: 0, nor: Normal::DOWN,
                rig: 0x10, top: 0x10, tex: 4
            },
            BlockFace {
                lef: 0x00, bot: 0x00, dep: 15, nor: Normal::NORTH,
                rig: 0x00, top: 0x00, tex: 4
            },
            BlockFace {
                lef: 0x00, bot: 0x00, dep: 15, nor: Normal::EAST,
                rig: 0x00, top: 0x00, tex: 4
            },
            BlockFace {
                lef: 0x00, bot: 0x00, dep: 15, nor: Normal::UP,
                rig: 0x00, top: 0x00, tex: 4
            },
        ])
    }
];

fn main() {
    // /*
    let mut glfw = gl_helper::init_glfw();
    let (mut window, events) = gl_helper::create_window();
    gl_helper::init_gl(&mut window);
    let mut status = WindowStatus::new();

    window.set_all_polling(true);
    window.make_current();
    let vertex_shader = Shader::create(gl::VERTEX_SHADER, include_str!("shader/shader.glsl.vert"));
    let fragment_shader = Shader::create(gl::FRAGMENT_SHADER, include_str!("shader/shader.glsl.frag"));
    let program = Program::create(vertex_shader, fragment_shader);
    Program::bind(program);
    unsafe {
        gl::ClearColor(1.0, 1.0, 1.0, 1.0);
        gl::Enable(gl::DEPTH_TEST);
        gl::Enable(gl::PRIMITIVE_RESTART);
        gl::PrimitiveRestartIndex(u32::MAX);
        let mut index_array = Vec::<u32>::with_capacity(1024 * 1024 / 4);

        let mut j = 0;
        for i in 0..(1024 * 1024 / 4) {
            if i % 5 == 4 {
                index_array.push(u32::MAX);
            }
            else {
                index_array.push(j);
                j += 1;
            }
        }
        let index_buffer = Buffer::create();

        index_buffer.bind_target(gl::ELEMENT_ARRAY_BUFFER);
        index_buffer.storage(1024 * 1024 / 4, gl::DYNAMIC_STORAGE_BIT);
        index_buffer.upload_slice(&index_array.as_slice(), 0, index_array.len() as isize);
    }
    // */
    
    let mut world = World::new();
    

    // /*
    let mut keys: HashMap<glfw::Key, bool> = HashMap::new();
    let mut start = std::time::Instant::now();
    let mut frames = 0;
    while !window.should_close() {
        glfw.poll_events();
        
        for (_, event) in glfw::flush_messages(&events) { handle_window_event(&mut window, &mut world, event, &mut keys, &mut status) }
        
        update(&mut world, &mut keys);
        
        draw(&mut world);
        
        window.swap_buffers();

        if frames % 100 == 0 {
            frames = 1;
            
            println!("{}", start.elapsed().as_millis() as f64 / 100.0);
            start = std::time::Instant::now();
        } else { frames += 1; }
    }
    // */
    // */
}

// /* 
#[allow(unused_variables)]
fn handle_window_event(window: &mut Window, world: &mut World, event: glfw::WindowEvent, keys: &mut HashMap<Key, bool>, status: &mut WindowStatus) {
    match event {
        glfw::WindowEvent::Size(width, height) => {
            unsafe {
                gl::Viewport(0, 0, width, height);
            }
            world.camera.ratio = width as f32 / height as f32;
        }
        glfw::WindowEvent::CursorPos(x, y) => {
            if !status.mouse_captured { return }
            let delta = (x - world.camera.prev_mouse.0, y - world.camera.prev_mouse.1);
            if world.camera.prev_mouse != (f64::MAX, f64::MAX) {
                world.camera.rot.yaw += (delta.0 / 500.0) as f32;
                world.camera.rot.pitch += (delta.1 / 500.0) as f32;
            }
            world.camera.prev_mouse = (x, y);
        }
        glfw::WindowEvent::Key(key, scancode, action, modifiers) => {
            keys.insert(key, if action == Action::Release { false } else { true });
            
            if action != Action::Press { return }
            match key {
                Key::Escape => { window.set_should_close(true) }
                Key::X => {
                    unsafe {
                        status.fill_mode = if status.fill_mode == gl::LINE { gl::FILL } else { gl::LINE };
                        gl::PolygonMode(gl::FRONT_AND_BACK, status.fill_mode)
                    }
                }
                Key::Tab => {
                    if status.maximized { window.restore() }
                    else { window.maximize() }
                    status.maximized = !status.maximized;
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
}

fn update(world: &mut World, keys: &mut HashMap<Key, bool>) {
    let speed = 0.1 * (if *keys.get(&Key::LeftControl).unwrap_or(&false) { 10.0 } else { 1.0 });
    for (key, pressed) in keys.iter() {
        if *pressed == false { continue }
        match key {
            Key::W => {
                world.camera.step(0.0, -speed as f64);
            }
            Key::S => {
                world.camera.step(0.0, speed as f64);
            }
            Key::A => {
                world.camera.step(-speed as f64, 0.0);
            }
            Key::D => {
                world.camera.step(speed as f64, 0.0);
            }
            Key::Space => {
                world.camera.pos.y += speed;
            }
            Key::LeftShift => {
                world.camera.pos.y -= speed;
            }
            _ => ()
        }
    }
}

fn draw(world: &mut World) {
    unsafe {
        gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        let matrix = world.camera.get_matrix();
        gl::UniformMatrix4fv(0, 1, gl::FALSE, matrix.as_array().as_ptr());
        for (chunk, x, y, z, _) in world.iter() {
            // if x != 0 || y != 0 || z != 0 { continue }
            if let Some(buffer) = &chunk.buffer {
                buffer.bind_indexed_target(gl::SHADER_STORAGE_BUFFER);
                
                /*
                let south = chunk.counts[0];
                let north = chunk.counts[3];    
                if world.camera.pos.z > chunk.pos.z as f32 * 16.0 + 16.0 { chunk.counts[0] = 0 }
                if world.camera.pos.z < chunk.pos.z as f32 * 16.0 { chunk.counts[3] = 0 }

                let west = chunk.counts[1];
                let east = chunk.counts[4];
                if world.camera.pos.x > chunk.pos.x as f32 * 16.0 + 16.0 { chunk.counts[1] = 0 }
                if world.camera.pos.x < chunk.pos.x as f32 * 16.0 { chunk.counts[4] = 0 }

                let down = chunk.counts[2];
                let up = chunk.counts[5];
                if world.camera.pos.y > chunk.pos.y as f32 * 16.0 + 16.0 { chunk.counts[2] = 0 }
                if world.camera.pos.y < chunk.pos.y as f32 * 16.0 { chunk.counts[5] = 0 }

                gl::MultiDrawElements(
                    gl::TRIANGLE_STRIP,
                    &raw const chunk.counts as *const i32,
                    gl::UNSIGNED_INT,
                    &raw const chunk.offsets as *const *const c_void,
                    6
                );
                chunk.counts[0] = south;
                chunk.counts[1] = west;
                chunk.counts[2] = down;
                chunk.counts[3] = north;
                chunk.counts[4] = east;
                chunk.counts[5] = up;
                // */
                gl::DrawElements(gl::TRIANGLE_STRIP, chunk.face_count as i32 * 5, gl::UNSIGNED_INT, ptr::null());
            }
        }
    }
}
// */
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