use std::collections::hash_map::RandomState;
use std::hint;
use std::{collections::HashMap, mem, os::raw::c_void, ptr::NonNull, time};
use crate::gl_util::framebuffer;
use crate::gl_util::{buffer::Buffer, buffer_allocator::BufferAllocator, framebuffer::FrameBuffer, gl_helper::WindowStatus, program::Program, renderbuffer::RenderBuffer, shader::Shader, texture::Texture};
use crate::render::byte_buffer::StagingBuffer;
use crate::render::world_renderer::WorldRenderer;
use cgmath::{Vector3, Euler, Deg, Rad};
use gl::types::GLsync;
use glfw::Key;
use simdnoise::NoiseBuilder;

use super::blockpos::BlockPos;
use super::section;
use super::{section::Section, camera::Camera};
#[derive(Debug)]
pub struct World {
    pub sections: HashMap::<Vector3<i32>, Box::<Section>>,
    pub camera: Camera,
    pub geometry_buffer_allocator: BufferAllocator,
    pub light_buffer_allocator: BufferAllocator,
    pub solid_staging_buffer: StagingBuffer,
    pub trans_staging_buffer: StagingBuffer,
    pub light_staging_buffer: StagingBuffer,
    pub renderer: WorldRenderer
}

impl World {
    pub fn new() -> World { unsafe {
        let mut world = World {
            sections: HashMap::<Vector3<i32>, Box::<Section>>::new(),
            camera: Camera::new(),
            geometry_buffer_allocator: BufferAllocator::new(1073741824),
            light_buffer_allocator: BufferAllocator::new(1073741824),
            solid_staging_buffer: StagingBuffer::new(),
            trans_staging_buffer: StagingBuffer::new(),
            light_staging_buffer: StagingBuffer::new(),
            renderer: WorldRenderer::new(),
        };

        // reserve space for the sky
        let sky_segment = world.light_buffer_allocator.alloc(1024);
        let sky_data = [(15_u32, 15_u32, 15_u32, 15_u32); 1024 / 4 / 4];
        world.light_buffer_allocator.buffer_sub_data(&sky_segment.unwrap(), 1024, 0, &raw const sky_data as *const c_void);
        return world;
    } }

    pub fn generate(&mut self) { unsafe {
        let noise = NoiseBuilder::gradient_3d(World::MAX_SECTION_X * 16, World::MAX_SECTION_Y * 16, World::MAX_SECTION_Z * 16).generate_scaled(0.0, 1.0);
        for x in 0..World::MAX_SECTION_X {
            for y in 0..World::MAX_SECTION_Y {
                for z in 0..World::MAX_SECTION_Z {
                    let mut section = Box::new(Section::new());
                    section.set_pos(Vector3 { x: x as i32, y: y as i32, z: z as i32 });
                    section.make_terrain(&noise);
                    // section.make_terrain_debug();
                    self.add_section(section);
                }
            }
        }
    } }

    pub fn mesh_all(&mut self) { unsafe {
        let start = time::Instant::now();
        let mut total_quads: usize = 0;

        let mut j = 0;
        let total = self.sections.len();
        for (i, section) in self.sections.values_mut().enumerate() {
            section.mesh(&mut self.solid_staging_buffer, &mut self.trans_staging_buffer, &mut self.geometry_buffer_allocator, &mut self.light_staging_buffer, &mut self.light_buffer_allocator);
            total_quads += section.solid_quad_count as usize;
            // if j == 100 {
            //     println!("meshed {i}/{total} : {}%", i as f32 / total as f32 * 100.0);
            //     j = 0;
            //     continue;
            // }
            j += 1;
        }
        
        let total_sections = self.sections.len();
        let elapsed = start.elapsed().as_millis();
        let sections_per_sec = (1000.0 / elapsed as f64 * total_sections as f64) as u64;
        let ms_per_section = elapsed as f64 / total_sections as f64;
        let quads_per_section = total_quads as u64 / total_sections as u64;

        let elapsed_nanos = start.elapsed().as_nanos() as f64;

        println!("[6/6 axes] [merged] {total_sections} sections | {elapsed}ms | {sections_per_sec} sections/s | {ms_per_section:.4}ms/section | {total_quads} quads | {quads_per_section} quads/section");
        let geometry_bytes_used = self.geometry_buffer_allocator.used;
        let light_bytes_used = self.light_buffer_allocator.used;
        let total_megabytes_used = (geometry_bytes_used + light_bytes_used) / 1024 / 1024;
        println!("{geometry_bytes_used} geometry bytes | {light_bytes_used} light bytes | {total_megabytes_used} megabytes");
    } }

    pub fn mesh_section(&mut self, section_pos: Vector3<i32>) {
        if let Some(section) = self.sections.get_mut(&section_pos) {
            section.mesh(&mut self.solid_staging_buffer, &mut self.trans_staging_buffer, &mut self.geometry_buffer_allocator, &mut self.light_staging_buffer, &mut self.light_buffer_allocator)
        }
    }

    pub fn add_section(&mut self, section: Box<Section>) { unsafe {
        let (x, y, z) = (section.section_pos.x, section.section_pos.y, section.section_pos.z);

        let section = Box::<Section>::into_raw(section);
        if let Some(south) = self.sections.get_mut(&Vector3 { x, y, z: z + 1 }) {
            south.neighbors.north = Some(NonNull::new_unchecked(section));
            (*section).neighbors.south = Some(NonNull::new_unchecked(&raw mut **south));
        }
        if let Some(west) = self.sections.get_mut(&Vector3 { x: x - 1, y, z }) {
            west.neighbors.east = Some(NonNull::new_unchecked(section));
            (*section).neighbors.west = Some(NonNull::new_unchecked(&raw mut **west));
        }
        if let Some(down) = self.sections.get_mut(&Vector3 { x, y: y - 1, z }) {
            down.neighbors.up = Some(NonNull::new_unchecked(section));
            (*section).neighbors.down = Some(NonNull::new_unchecked(&raw mut **down));
        }
        if let Some(north) = self.sections.get_mut(&Vector3 { x, y, z: z - 1 }) {
            north.neighbors.south = Some(NonNull::new_unchecked(section));
            (*section).neighbors.north = Some(NonNull::new_unchecked(&raw mut **north));
        }
        if let Some(east) = self.sections.get_mut(&Vector3 { x: x + 1, y, z }) {
            east.neighbors.west = Some(NonNull::new_unchecked(section));
            (*section).neighbors.east = Some(NonNull::new_unchecked(&raw mut **east));
        }
        if let Some(up) = self.sections.get_mut(&Vector3 { x, y: y + 1, z }) {
            up.neighbors.down = Some(NonNull::new_unchecked(section));
            (*section).neighbors.up = Some(NonNull::new_unchecked(&raw mut **up));
        }

        let section = Box::from_raw(*(&raw const section as *mut *mut Section));
        self.sections.insert(section.section_pos, section);
    } }

    pub fn remove_section(&mut self, section_pos: Vector3<i32>) { unsafe {
        if let Some(mut section) = self.sections.remove(&section_pos) {
            // println!("removing section at {} {} {}", section_pos.x, section_pos.y, section_pos.z);
            if let Some(mut south) = section.neighbors.south {
                south.as_mut().neighbors.north = None;
            }
            if let Some(mut west) = section.neighbors.west {
                west.as_mut().neighbors.east = None;
            }
            if let Some(mut down) = section.neighbors.down {
                down.as_mut().neighbors.up = None;
            }
            if let Some(mut north) = section.neighbors.north {
                north.as_mut().neighbors.south = None;
            }
            if let Some(mut east) = section.neighbors.east {
                east.as_mut().neighbors.west = None;
            }
            if let Some(mut up) = section.neighbors.up {
                up.as_mut().neighbors.down = None;
            }
            self.geometry_buffer_allocator.free(section.solid_segment.take());
            self.light_buffer_allocator.free(section.solid_light_segment.take());
        }
    } }

    pub fn set_block(&mut self, world_pos: Vector3<i32>, block_id: u16) { unsafe {
        let section_pos = Vector3 {
            x: world_pos.x / 16,
            y: world_pos.y / 16,
            z: world_pos.z / 16,
        };
        let section = self.sections.get_mut(&section_pos);
        if let Some(section) = section {
            let block_pos = BlockPos::new(world_pos.x & 15, world_pos.y & 15, world_pos.z & 15);
            section.set_block(block_pos, block_id)
        }
    } }

    pub fn render(&mut self, status: &WindowStatus, end_framebuffer_id: i32) { unsafe {
        self.renderer.render(self, status, end_framebuffer_id);
    } }

    pub fn update(&mut self, keys: &HashMap<Key, bool>) {
        let speed = 0.05 * (if *keys.get(&Key::LeftControl).unwrap_or(&false) { 10.0 } else { 1.0 });
        for (key, pressed) in keys.iter().filter(|(_, pressed)| **pressed) {
            match key {
                Key::W => {
                    self.camera.step(0.0, 0.0, -speed as f64);
                }
                Key::S => {
                    self.camera.step(0.0, 0.0, speed as f64);
                }
                Key::A => {
                    self.camera.step(-speed as f64, 0.0, 0.0);
                }
                Key::D => {
                    self.camera.step(speed as f64, 0.0, 0.0);
                }
                Key::Q=> {
                    self.camera.camera_rot.z += Rad(0.005);
                }
                Key::E => {
                    self.camera.camera_rot.z -= Rad(0.005);
                }
                Key::Space => {
                    self.camera.step(0.0, speed as f64, 0.0);
                }
                Key::LeftShift => {
                    self.camera.step(0.0, - speed as f64, 0.0);
                }
                _ => {}
            }
        }
        if !self.camera.frustum_frozen {
            self.camera.frustum_pos = self.camera.camera_pos;
            self.camera.frustum_rot = self.camera.camera_rot;
        }
    }

    pub fn kill(self) {
        self.geometry_buffer_allocator.kill();
        self.light_buffer_allocator.kill();
        self.renderer.kill();
    }

    pub const MAX_SECTION_X: usize = 8;
    pub const MAX_SECTION_Y: usize = 8;
    pub const MAX_SECTION_Z: usize = 8;
}