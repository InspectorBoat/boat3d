#![allow(incomplete_features)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_must_use)]
#![allow(non_snake_case)]
#![allow(unused_parens)]
#![feature(unchecked_math)]
#![feature(pointer_byte_offsets)]
#![feature(adt_const_params)]
#![feature(core_intrinsics)]
#![feature(new_uninit)]
#![feature(portable_simd)]
#![feature(raw_ref_op)]
#![allow(overflowing_literals)]
#![allow(unused_unsafe)]
#![feature(result_option_inspect)]
#![feature(int_roundings)]
mod block;
mod world;
mod util;

use std::{collections::HashMap, ptr, os::raw::c_void, hint::black_box, time::SystemTime, mem};

use block::{blockstate::BlockState, blockface::BlockFace, block::Block, blockface::Normal, blockmodel::BlockModel};
use glfw::{Context, Window, Action, Key};
use util::{gl_helper::*, byte_buffer::ByteBuffer};
use world::{world::{World, Lcg}, chunk::{Vec3i, Chunk}};

use crate::util::gl_helper;

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
    // /*
    let mut glfw = gl_helper::init_glfw();
    let mut status = WindowStatus::new();
    let (mut window, events) = gl_helper::create_window(&status);

    gl_helper::init_gl(&mut window);

    let geometry_program = Program::create(
        Shader::create(gl::VERTEX_SHADER, include_str!("shader/geometry.glsl.vert")),
        Shader::create(gl::FRAGMENT_SHADER, include_str!("shader/geometry.glsl.frag"))
    );
    Program::bind(&geometry_program);

    let post_program = Program::create(
        Shader::create(gl::VERTEX_SHADER, include_str!("shader/post.glsl.vert")),
        Shader::create(gl::FRAGMENT_SHADER, include_str!("shader/post.glsl.frag"))
    );

    gl_helper::setup_element_array();

    
    //screen quad buffer
    let quad_vertices: [f32; 24] = [
        -1.0,  1.0,  0.0, 1.0,
        -1.0, -1.0,  0.0, 0.0,
         1.0, -1.0,  1.0, 0.0,

        -1.0,  1.0,  0.0, 1.0,
         1.0, -1.0,  1.0, 0.0,
         1.0,  1.0,  1.0, 1.0
    ];

    let quad_buffer = Buffer::create();
    quad_buffer.upload(&quad_vertices, mem::size_of::<[f32; 24]>() as isize, gl::STATIC_DRAW);
    quad_buffer.bind_target(gl::ARRAY_BUFFER);
    gl::EnableVertexAttribArray(0);
    gl::VertexAttribPointer(0, 2, gl::FLOAT, gl::FALSE, 4 * mem::size_of::<f32>() as i32, ptr::null());
    gl::EnableVertexAttribArray(1);
    gl::VertexAttribPointer(1, 2, gl::FLOAT, gl::FALSE, 4 * mem::size_of::<f32>() as i32, (2 * mem::size_of::<f32>()) as *const c_void);

    //framebuffer
    let mut framebuffer: u32 = 0;
    gl::GenFramebuffers(1, &mut framebuffer);
    gl::BindFramebuffer(gl::FRAMEBUFFER, framebuffer);

    let mut texture_buffer: u32 = 0;
    gl::GenTextures(1, &mut texture_buffer);
    gl::BindTexture(gl::TEXTURE_2D, texture_buffer);
    gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RGBA32F as i32, status.width as i32, status.height as i32, 0, gl::RGBA, gl::UNSIGNED_BYTE, ptr::null());
    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
    gl::FramebufferTexture2D(gl::FRAMEBUFFER, gl::COLOR_ATTACHMENT0, gl::TEXTURE_2D, texture_buffer, 0);

    let mut rbo: u32 = 0;
    gl::GenRenderbuffers(1, &mut rbo);
    gl::BindRenderbuffer(gl::RENDERBUFFER, rbo); 
    gl::RenderbufferStorage(gl::RENDERBUFFER, gl::DEPTH24_STENCIL8, status.width as i32, status.height as i32);  
    gl::FramebufferRenderbuffer(gl::FRAMEBUFFER, gl::DEPTH_STENCIL_ATTACHMENT, gl::RENDERBUFFER, rbo);
    let mut world = World::new();
    
    // /*
    let mut keys: HashMap<glfw::Key, bool> = HashMap::new();
    let mut start = std::time::Instant::now();
    let mut frames = 1;

    world.pool.buffer.bind_indexed_target_base(gl::SHADER_STORAGE_BUFFER, 0);

    while !window.should_close() {
        glfw.poll_events();
        
        for (_, event) in glfw::flush_messages(&events) { handle_window_event(&mut window, &mut world, event, &mut keys, &mut status) }
        
        update(&mut world, &mut keys);
        
        draw(&mut world, framebuffer, &geometry_program, &post_program, texture_buffer);

        window.swap_buffers();

        if frames % 100 == 0 {
            frames = 1;
            
            println!("fps: {}", 1000.0 / (start.elapsed().as_millis() as f64 / 100.0));
            start = std::time::Instant::now();
        } else { frames += 1; }
    }
    // */
    // */
} }

// /* 
#[allow(unused_variables)]
fn handle_window_event(window: &mut Window, world: &mut World, event: glfw::WindowEvent, keys: &mut HashMap<Key, bool>, status: &mut WindowStatus) { unsafe {
    match event {
        glfw::WindowEvent::Size(width, height) => {
            unsafe {
                gl::Viewport(0, 0, width, height);
            }
            (status.width, status.height) = (width, height);
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
} }

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

fn draw(world: &mut World, framebuffer: u32, geometry_program: &Program, post_program: &Program, texture_buffer: u32) { unsafe {
    Program::bind(geometry_program);
    gl::BindFramebuffer(gl::FRAMEBUFFER, framebuffer);
    gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
    gl::Enable(gl::DEPTH_TEST);
    
    let camera_matrix = world.camera.get_matrix();
    gl::UniformMatrix4fv(0, 1, gl::FALSE, camera_matrix.as_array().as_ptr());
    for chunk in world.chunks.values() {
        if let Some(page) = &chunk.page {
            if chunk.pos.x >= 2 || chunk.pos.y >= 1 || chunk.pos.z >= 1 { continue; }
            gl::Uniform4iv(1, 1, &raw const chunk.pos as *const i32);
            gl::DrawElementsBaseVertex(gl::TRIANGLE_STRIP, chunk.face_count as i32 * 5, gl::UNSIGNED_INT, ptr::null(), (page.start * 1024 / 2) as i32);
        }
    }
    
    Program::bind(post_program);
    gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
    gl::ClearColor(1.0, 1.0, 1.0, 1.0);
    gl::Clear(gl::COLOR_BUFFER_BIT);
    gl::Disable(gl::DEPTH_TEST);
    gl::BindTexture(gl::TEXTURE_2D, texture_buffer);
    gl::DrawArrays(gl::TRIANGLES, 0, 6);
} }
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
