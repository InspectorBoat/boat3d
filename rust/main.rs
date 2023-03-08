// #![allow(dead_code)]
// #![allow(unused_variables)]
#![feature(let_chains)]
#![feature(unchecked_math)]
#![feature(pointer_byte_offsets)]
#![feature(adt_const_params)]
#![feature(associated_type_bounds)] 
#![feature(print_internals)]
#![feature(layout_for_ptr)]
#![feature(exclusive_range_pattern)]
#![feature(maybe_uninit_uninit_array)]
#![feature(maybe_uninit_slice)]
mod block;
mod world;
mod util;

use std::{collections::HashMap, ptr};

use block::{blockstate::BlockState, blockface::BlockFace, block::Block, blockface::Normal, blockmodel::BlockModel};
use glfw::{Context, Window, Action, Key};
use util::gl_helper::*;
use world::world::World;

use crate::util::{gl_helper, buffer::ByteBuffer};

static BLOCKS: [BlockState; 4] = [
    BlockState {
        block: Block { name: "air" },
        model: BlockModel([
            BlockFace::NONE,
            BlockFace::NONE,
            BlockFace::NONE,
            BlockFace::NONE,
            BlockFace::NONE,
            BlockFace::NONE
        ])
    },
    BlockState {
        block: Block { name: "grass" },
        model: BlockModel([
            BlockFace {
                u: 0, v: 0, d: 0, n: Normal::SOUTH,
                w: 15, h: 15, t: 0
            },
            BlockFace {
                u: 0, v: 0, d: 0, n: Normal::WEST,
                w: 15, h: 15, t: 0
            },
            BlockFace {
                u: 0, v: 0, d: 0, n: Normal::DOWN,
                w: 15, h: 15, t: 0
            },
            BlockFace {
                u: 0, v: 15, d: 0, n: Normal::UP,
                w: 15, h: 15, t: 0
            },
            BlockFace {
                u: 15, v: 0, d: 0, n: Normal::EAST,
                w: 15, h: 15, t: 0
            },
            BlockFace {
                u: 0, v: 0, d: 15, n: Normal::NORTH,
                w: 15, h: 15, t: 0
            },
        ])
    },
    BlockState {
        block: Block { name: "stone" },
        model: BlockModel([
            BlockFace {
                u: 0, v: 0, d: 0, n: Normal::SOUTH,
                w: 15, h: 15, t: 1
            },
            BlockFace {
                u: 0, v: 0, d: 0, n: Normal::WEST,
                w: 15, h: 15, t: 0
            },
            BlockFace {
                u: 0, v: 0, d: 0, n: Normal::DOWN,
                w: 15, h: 15, t: 0
            },
            BlockFace {
                u: 0, v: 15, d: 0, n: Normal::UP,
                w: 15, h: 15, t: 0
            },
            BlockFace {
                u: 15, v: 0, d: 0, n: Normal::EAST,
                w: 15, h: 15, t: 0
            },
            BlockFace {
                u: 0, v: 0, d: 15, n: Normal::NORTH,
                w: 15, h: 15, t: 0
            },
        ])
    },
    BlockState {
        block: Block { name: "dirt" },
        model: BlockModel([
            BlockFace {
                u: 0, v: 0, d: 0, n: Normal::SOUTH,
                w: 15, h: 15, t: 2
            },
            BlockFace {
                u: 0, v: 0, d: 0, n: Normal::WEST,
                w: 15, h: 15, t: 0
            },
            BlockFace {
                u: 0, v: 0, d: 0, n: Normal::DOWN,
                w: 15, h: 15, t: 0
            },
            BlockFace {
                u: 0, v: 0, d: 15, n: Normal::UP,
                w: 15, h: 15, t: 0
            },
            BlockFace {
                u: 0, v: 0, d: 15, n: Normal::EAST,
                w: 15, h: 15, t: 0
            },
            BlockFace {
                u: 0, v: 0, d: 15, n: Normal::NORTH,
                w: 15, h: 15, t: 0
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
    
    let vertex_shader = Shader::create(gl::VERTEX_SHADER, include_str!("shader/shader.vert"));
    let fragment_shader = Shader::create(gl::FRAGMENT_SHADER, include_str!("shader/shader.frag"));
    let program = Program::create(vertex_shader, fragment_shader);
    Program::bind(program);
    log_error();
    let index_buffer = Buffer::create();
    unsafe {
        gl::ClearColor(1.0, 1.0, 1.0, 1.0);
        gl::Enable(gl::DEPTH_TEST);
        // gl::Enable(gl::CULL_FACE);
        gl::Enable(gl::PRIMITIVE_RESTART);
        gl::PrimitiveRestartIndex(u32::MAX);
        let mut index_array = Vec::with_capacity(1024 * 1024 / 4);

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
        index_buffer.bind_target(gl::ELEMENT_ARRAY_BUFFER);
        index_buffer.storage(1024 * 1024, gl::DYNAMIC_STORAGE_BIT);
        index_buffer.upload_slice(&index_array.as_slice(), 0, index_array.len() as isize);
    }
    // */
    let mut world = World::new();

    // /*
    let mut keys: HashMap<glfw::Key, bool> = HashMap::new();
    while !window.should_close() {
        glfw.poll_events();
        
        for (_, event) in glfw::flush_messages(&events) {
            handle_window_event(&mut window, &mut world, event, &mut keys, &mut status);
        }
        
        update(&mut world, &mut keys);
        
        draw(&mut world);
        
        window.swap_buffers();
    }
    for chunk in &mut world.chunks[..] {
        chunk.kill_buffer();
    }
    // */
    // */

    // let mut arr: [u8; 8] = [0, 1, 2, 3, 4, 5, 6, 7];
    // let face = &BLOCKS[0].model[Normal::SOUTH];
    // println!("{:?}", arr);
    // unsafe {
    //     let loc = ptr::addr_of_mut!(arr) as *mut u64;
    //     *loc = face.as_u64();
    // }
    // println!("{:?}", arr);
    // arr[0] = face.u;
    // arr[1] = face.v;
    // arr[2] = face.d;
    // arr[3] = face.n.0;
    // arr[4] = face.w;
    // arr[5] = face.h;
    // arr[6] = 0;
    // arr[7] = face.t as u8;
    // println!("{:?}", arr);
    // let mut buffer = ByteBuffer::new();
    // buffer.put_u64(face.as_u64() + ((0x1 << 4) | (0x2 << 12) | (0x3 << 20)));
    // println!("{:?}", &buffer.arr[0..8]);
    // buffer.pos = 0;
    // buffer.put(face.u);
    // buffer.put(face.v);
    // buffer.put(face.d);
    // buffer.put(face.n.0);

    // buffer.put(face.w);
    // buffer.put(face.h);
    // buffer.put(0);
    // buffer.put(face.t as u8);
    // println!("{:?}", &buffer.arr[0..8]);
}

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
            // if x >= 1 || y >= 1 || z != 10 { continue }
            if let Some(buffer) = &chunk.buffer {
                buffer.bind_indexed_target(gl::SHADER_STORAGE_BUFFER);
                gl::DrawElements(gl::TRIANGLE_STRIP, (chunk.face_count * 5) as i32, gl::UNSIGNED_INT, ptr::null());
            }
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