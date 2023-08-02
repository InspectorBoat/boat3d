use std::collections::hash_map::RandomState;
use std::hint;
use std::{collections::HashMap, mem, os::raw::c_void, ptr::NonNull, time};
use crate::gl_util::{buffer::Buffer, buffer_allocator::BufferAllocator, framebuffer::FrameBuffer, gl_helper::WindowStatus, program::Program, renderbuffer::RenderBuffer, shader::Shader, texture::Texture};
use crate::render::byte_buffer::StagingBuffer;
use crate::render::world_renderer::WorldRenderer;
use cgmath::Vector3;
use cgmath_culling::Intersection;
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
    pub geometry_staging_buffer: StagingBuffer,
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
            geometry_staging_buffer: StagingBuffer::new(),
            light_staging_buffer: StagingBuffer::new(),
            renderer: WorldRenderer::new(),
        };

        // reserve space for the sky
        let sky_segment = world.light_buffer_allocator.alloc(1024);
        let sky_data: [(u32, u32, u32, u32); 1024 / 4 / 4] = [(15, 15, 15, 15); 1024 / 4 / 4];
        world.light_buffer_allocator.upload_offset(&sky_segment.unwrap(), &sky_data, 1024, 0);
        return world;
    } }

    pub fn generate(&mut self) { unsafe {
        let noise = NoiseBuilder::gradient_3d(World::MAX_SECTION_X * 16, World::MAX_SECTION_Y * 16, World::MAX_SECTION_Z * 16).generate_scaled(0.0, 1.0);
        for x in 0..World::MAX_SECTION_X {
            for y in 0..World::MAX_SECTION_Y {
                for z in 0..World::MAX_SECTION_Z {
                    let mut section = Box::<Section>::new_zeroed().assume_init();
                    section.set_chunk_pos(Vector3 { x: x as i32, y: y as i32, z: z as i32 });
                    // section.make_terrain(&noise);
                    section.make_terrain_alt();
                    self.add_section(section);
                }
            }
        }
    } }

    pub fn mesh_all(&mut self) { unsafe {
        let start = time::Instant::now();
        let mut total_quads: usize = 0;

        let mut exact_geometry_bytes = 0;
        let mut exact_light_bytes = 0;

        let mut j = 0;
        let total = self.sections.len();
        for (i, section) in self.sections.values_mut().enumerate() {
            section.mesh(&mut self.geometry_staging_buffer, &mut self.geometry_buffer_allocator, &mut self.light_staging_buffer, &mut self.light_buffer_allocator);
            exact_geometry_bytes += self.geometry_staging_buffer.idx;
            exact_light_bytes += self.light_staging_buffer.idx;
            self.geometry_staging_buffer.reset();
            self.light_staging_buffer.reset();
            total_quads += section.quad_count as usize;

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
        const NANOSECONDS_PER_CYCLE: f64 = 0.4;
        let nano_per_section = elapsed_nanos / total_sections as f64;
        let nanos_per_pair = nano_per_section / 4096.0 / 3.0;
        let cycles_per_pair = nanos_per_pair / 0.4;

        println!("[6/6 axes] [merged] {total_sections} sections | {elapsed}ms | {sections_per_sec} sections/s | {ms_per_section:.4}ms/section | {cycles_per_pair:.1} cycles/face pair | {total_quads} quads | {quads_per_section} quads/section");
        let geometry_bytes_used = self.geometry_buffer_allocator.used;
        let light_bytes_used = self.light_buffer_allocator.used;
        let total_megabytes_used = (geometry_bytes_used + light_bytes_used) / 1024 / 1024;
        println!("{geometry_bytes_used} geometry bytes | {light_bytes_used} light bytes | {total_megabytes_used} megabytes");
    } }

    pub fn mesh(&mut self, section_pos: Vector3<i32>) {
        if let Some(section) = self.sections.get_mut(&section_pos) {
            section.mesh_geometry(&mut self.geometry_staging_buffer, &mut self.geometry_buffer_allocator);
            section.mesh_light(&mut self.geometry_staging_buffer, &mut self.geometry_buffer_allocator, &mut self.light_staging_buffer, &mut self.light_buffer_allocator);
            self.geometry_staging_buffer.reset();
            self.light_staging_buffer.reset();
        }
    }

    pub fn add_section(&mut self, section: Box<Section>) { unsafe {
        let (x, y, z) = (section.section_pos.x, section.section_pos.y, section.section_pos.z);

        let section = Box::<Section>::into_raw(section) as usize as *mut Section;
        if let Some(south) = self.sections.get_mut(&Vector3 { x, y, z: z - 1 }) {
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
        if let Some(north) = self.sections.get_mut(&Vector3 { x, y, z: z + 1 }) {
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
            println!("removing section at {} {} {}", section_pos.x, section_pos.y, section_pos.z);
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
            self.geometry_buffer_allocator.free(section.geometry_page.take());
            self.light_buffer_allocator.free(section.light_page.take());
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

    pub fn render(&self) { unsafe {
        self.renderer.render(self);
    } }

    pub fn update(&mut self, keys: &mut HashMap<Key, bool>) {
        let speed = 5.0 * (if *keys.get(&Key::LeftControl).unwrap_or(&false) { 10.0 } else { 1.0 });
        for (key, pressed) in keys.iter() {
            if *pressed == false { continue; }
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

    pub const MAX_SECTION_X: usize = 1;
    pub const MAX_SECTION_Y: usize = 1;
    pub const MAX_SECTION_Z: usize = 1;
}