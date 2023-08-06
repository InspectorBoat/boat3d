#![allow(incomplete_features)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(unused_parens)]
#![allow(unused_unsafe)]
#![allow(unused_imports)]
#![allow(unreachable_code)]
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

use std::hint::black_box;
use std::time::Instant;
use std::{collections::HashMap, hint};
use std::env;
use cgmath::{Vector3, Vector4};
use cgmath_culling::BoundingBox;
use gl::types::GLsync;
use gl_util::fps_tracker::FpsTracker;
use glfw::{Context, Window, Action, Key};
use gl_util::gl_helper::*;
use world::world::World;
use crate::gl_util::gl_helper;

fn main() { unsafe {
    // env::set_var("RUST_BACKTRACE", "1");
    let mut glfw = gl_helper::init_glfw();
    let mut status = WindowStatus::new();
    let (mut window, events) = gl_helper::create_window(&status);
    gl_helper::init_gl(&mut window);
    
    let mut world = World::new();
    
    world.renderer.init(&status);
    world.generate();
    world.mesh_all();
    
    world.geometry_buffer_allocator.device_buffer.bind_indexed_target_base(gl::SHADER_STORAGE_BUFFER, 0);
    world.light_buffer_allocator.device_buffer.bind_indexed_target_base(gl::SHADER_STORAGE_BUFFER, 1);
    
    glfw.set_swap_interval(glfw::SwapInterval::None);
    
    let mut keys: HashMap<glfw::Key, bool> = HashMap::new();
    let mut fps = FpsTracker::new();
    while !window.should_close() {
        glfw.poll_events();
        glfw::flush_messages(&events).for_each(|(_, event)| handle_window_event(&mut window, &mut world, event, &mut keys, &mut status));
        
        world.update(&mut keys);
        world.render(&status);
        window.swap_buffers();
        fps.tick();
    }
} }

fn handle_window_event(window: &mut Window, world: &mut World, event: glfw::WindowEvent, keys: &mut HashMap<Key, bool>, status: &mut WindowStatus) { unsafe {
    match event {
        glfw::WindowEvent::Size(width, height) => {
            gl::Viewport(0, 0, width, height);
            (status.width, status.height) = (width, height);
            world.camera.ratio = width as f32 / height as f32;
            world.renderer.make_framebuffer(status);
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
                Key::Escape => {
                    window.set_should_close(true);
                }
                Key::X => {
                    status.fill_mode = if status.fill_mode == gl::LINE { gl::PolygonMode(gl::FRONT_AND_BACK, gl::FILL); gl::FILL } else { gl::LINE };
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
                Key::G => {
                    println!("{:?}", world.camera);
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