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
mod cull;

use std::hint::black_box;
use std::time::Instant;
use std::{collections::HashMap, hint};
use std::{env, mem};
use cgmath::{Vector3, Vector4, Rad};
use cull::bounding_box::{self, BoundingBox};
use cull::rasterizer::Rasterizer;
use gl_util::fps_tracker::FpsTracker;
use gl_util::framebuffer::FrameBuffer;
use gl_util::texture::Texture;
use glfw::{Context, Window, Action, Key};
use gl_util::{gl_helper::*, gl_wrapper};
use rand::{thread_rng, Rng};
use render::world_renderer::WorldRenderer;
use world::camera;
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
    
    glfw.set_swap_interval(glfw::SwapInterval::None);
    
    let mut keys: HashMap<glfw::Key, bool> = HashMap::new();
    let mut fps = FpsTracker::new();

    let mut rasterizer = Rasterizer::new(600, 600);
    
    let framebuffer = FrameBuffer::create();
    framebuffer.bind(gl_wrapper::FRAMEBUFFER);
    framebuffer.texture2d_attachment(gl_wrapper::COLOR_ATTACHMENT0, &rasterizer.texture, 0);

    while !window.should_close() {
        glfw.poll_events();
        glfw::flush_messages(&events).for_each(|(_, event)| handle_window_event(&mut window, &mut world, event, &mut keys, &mut status));
        
        world.update(&keys);

        if status.rasterize {
            rasterizer.clear();
            let mut bounding_box = BoundingBox {
                min: Vector3 { x: 16.0, y: 0.0, z: 16.0 } - world.camera.camera_pos,
                max: Vector3 { x: 128.0, y: 16.0, z: 128.0 } - world.camera.camera_pos
            };
            rasterizer.rasterize(&mut bounding_box, world.camera.get_local_camera_matrix());
            
            rasterizer.render_to_texture();
            gl_wrapper::BindFramebuffer(gl_wrapper::DRAW_FRAMEBUFFER, 0 as u32);
            gl_wrapper::BindFramebuffer(gl_wrapper::READ_FRAMEBUFFER, framebuffer.id);
            gl_wrapper::BlitFramebuffer(0, 0, rasterizer.width as i32, rasterizer.height as i32, 0, 0, status.width, status.height, gl_wrapper::COLOR_BUFFER_BIT, gl_wrapper::NEAREST);
        } else {
            world.render(&status, 0);
        }
        window.swap_buffers();

        fps.tick();
    }
} }

fn handle_window_event(window: &mut Window, world: &mut World, event: glfw::WindowEvent, keys: &mut HashMap<Key, bool>, status: &mut WindowStatus) { unsafe {
    match event {
        glfw::WindowEvent::Size(width, height) => {
            gl_wrapper::Viewport(0, 0, width, height);
            (status.width, status.height) = (width, height);
            world.camera.aspect = width as f32 / height as f32;
            world.renderer.make_framebuffer(status);
        }
        glfw::WindowEvent::CursorPos(x, y) => {
            if !status.mouse_captured { return }
            let delta = (x - world.camera.prev_mouse.0, y - world.camera.prev_mouse.1);
            if world.camera.prev_mouse != (f64::MAX, f64::MAX) {
                world.camera.camera_rot.y += Rad((delta.0 / 500.0) as f32);
                world.camera.camera_rot.x += Rad((delta.1 / 500.0) as f32);
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
                    status.fill_mode = if status.fill_mode == gl_wrapper::LINE { gl_wrapper::PolygonMode(gl_wrapper::FRONT_AND_BACK, gl_wrapper::FILL); gl_wrapper::FILL } else { gl_wrapper::LINE };
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
                Key::R => {
                    status.rasterize = !status.rasterize;
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