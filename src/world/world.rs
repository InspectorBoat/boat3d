use core::slice;
use std::cell::{Ref, RefCell, UnsafeCell};
use std::collections::HashMap;
use std::hint::unreachable_unchecked;
use std::mem::MaybeUninit;
use std::ops::{Deref, Add};
use std::os::raw::c_void;
use std::ptr::NonNull;
use std::str::CharIndices;
use std::{ptr, hint};
use std::{time, hint::black_box, alloc, mem};
use crate::block::blockface::{Normal, BlockFace};
use crate::util::byte_buffer::StagingBuffer;
use crate::util::gl_helper::{Buffer, BufferPoolAllocator, log_if_error, Page, FrameBuffer, Texture, RenderBuffer, WindowStatus, Program, Shader};
use crate::world::{section, self};
use cgmath::Vector3;
use cgmath_culling::{Intersection, BoundingBox, FrustumCuller};
use simdnoise::NoiseBuilder;

use super::{section::Section, camera::Camera};
#[derive(Debug)]
pub struct World {
    pub sections: HashMap::<Vector3<i32>, Box::<Section>>,
    pub camera: Camera,
    pub geometry_pool: BufferPoolAllocator<1048576, 1024>,
    pub light_pool: BufferPoolAllocator<1048576, 1024>,
    pub geometry_staging_buffer: StagingBuffer,
    pub light_staging_buffer: StagingBuffer,
    pub framebuffer: Option<FrameBuffer>,
    pub texture_attachment: Option<Texture>,
    pub renderbuffer_attachment: Option<RenderBuffer>,
    pub index_buffer: Option<Buffer>,
    pub geometry_program: Option<Program>,
    pub post_program: Option<Program>,
    pub screen_buffer: Option<Buffer>,
    pub block_texture: Option<Texture>,
    pub counts: Vec<i32>,
    pub indices: Vec<*const c_void>,
    pub base_vertices: Vec<i32>,
}

impl World {
    pub fn new() -> World { unsafe {
        let mut world = World {
            sections: HashMap::<Vector3<i32>, Box::<Section>>::new(),
            camera: Camera::new(),
            geometry_pool: BufferPoolAllocator::new(),
            light_pool: BufferPoolAllocator::new(),
            geometry_staging_buffer: StagingBuffer::new(),
            light_staging_buffer: StagingBuffer::new(),
            framebuffer: None,
            texture_attachment: None,
            renderbuffer_attachment: None,
            index_buffer: None,
            geometry_program: None,
            post_program: None,
            screen_buffer: None,
            block_texture: None,
            counts: Vec::new(),
            indices: Vec::new(),
            base_vertices: Vec::new(),
        };

        // reserve space for the sky
        let sky_page = world.light_pool.allocate(world.light_pool.block_size());
        let sky_data: [(u32, u32, u32, u32); 1024 / 4 / 4] = [(15, 0, 0, 0); 1024 / 4 / 4];
        world.light_pool.upload(&sky_page.unwrap(), &sky_data, 1024);

        return world;
    } }

    pub fn make_block_texture(&mut self) { unsafe {
        gl::ActiveTexture(gl::TEXTURE0);
        let block_texture = Texture::create();
        block_texture.bind(gl::TEXTURE_2D_ARRAY);
        let mut image: [u8; 16 * 4 * 64] = [0; 16 * 4 * 64].map(|_| rand::random());
        image[0] = 127;
        image[1] = 127;
        image[2] = 127;
        image[3] = 0;

        gl::TexImage3D(gl::TEXTURE_2D_ARRAY, 0, gl::RGBA as i32, 4, 4, 64, 0, gl::RGBA, gl::BYTE, &raw const image as *const c_void);
        gl::TexParameteri(gl::TEXTURE_2D_ARRAY, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
        gl::TexParameteri(gl::TEXTURE_2D_ARRAY, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
        // set texture filtering parameters
        gl::TexParameteri(gl::TEXTURE_2D_ARRAY, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
        gl::TexParameteri(gl::TEXTURE_2D_ARRAY, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
        log_if_error();
        self.block_texture = Some(block_texture);
    } }
    
    pub fn make_framebuffer(&mut self, status: &WindowStatus) { unsafe {
        if let Some(framebuffer) = self.framebuffer.take() {
            framebuffer.kill();
        }
        if let Some(texture_attachment) = self.texture_attachment.take() {
            texture_attachment.kill();
        }
        if let Some(renderbuffer_attachment) = self.renderbuffer_attachment.take() {
            renderbuffer_attachment.kill();
        }

        let framebuffer = FrameBuffer::create();
        framebuffer.bind(gl::FRAMEBUFFER);
    
        let texture_attachment = Texture::create();
        gl::ActiveTexture(gl::TEXTURE0);
        texture_attachment.bind(gl::TEXTURE_2D);
        gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RGBA32UI as i32, status.width as i32, status.height as i32, 0, gl::RGBA_INTEGER, gl::INT, ptr::null());
        // gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RGBA32UI as i32, status.width as i32, status.height as i32, 0, gl::RGBA_INTEGER, gl::INT, ptr::null());
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
        FrameBuffer::texture2d_attachment(gl::FRAMEBUFFER, gl::COLOR_ATTACHMENT0, gl::TEXTURE_2D, &texture_attachment, 0);
    
        let renderbuffer_attachment = RenderBuffer::create();
        renderbuffer_attachment.bind(gl::RENDERBUFFER);
        gl::RenderbufferStorage(gl::RENDERBUFFER, gl::DEPTH_COMPONENT32F, status.width as i32, status.height as i32);  
        FrameBuffer::renderbuffer_attachment(gl::FRAMEBUFFER, gl::DEPTH_ATTACHMENT, gl::RENDERBUFFER, &renderbuffer_attachment);
        
        let attachments: [u32; 1] = [gl::COLOR_ATTACHMENT0];
        gl::DrawBuffers(1, &raw const attachments as *const u32);

        self.framebuffer = Some(framebuffer);
        self.texture_attachment = Some(texture_attachment);
        self.renderbuffer_attachment = Some(renderbuffer_attachment);
    } }

    pub fn make_index_buffer(&mut self) { unsafe {
        gl::Enable(gl::PRIMITIVE_RESTART);
        gl::PrimitiveRestartIndex(u32::MAX as u32);
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
        index_buffer.storage(1024 * 1024, gl::DYNAMIC_STORAGE_BIT);
        index_buffer.upload_slice(&index_array.as_slice(), 0, 1024 * 1024);
        self.index_buffer = Some(index_buffer);    
    } }

    pub fn make_shader_programs(&mut self) { unsafe {
        let geometry_program = Program::create(
            Shader::create(gl::VERTEX_SHADER, include_str!("../shader/geometry.glsl.vert")),
            Shader::create(gl::FRAGMENT_SHADER, include_str!("../shader/geometry.glsl.frag"))
        );
        let post_program = Program::create(
            Shader::create(gl::VERTEX_SHADER, include_str!("../shader/post.glsl.vert")),
            Shader::create(gl::FRAGMENT_SHADER, include_str!("../shader/post.glsl.frag"))
        );
        post_program.uniform1i(0, 0);
        post_program.uniform1i(1, 1);
    
        self.geometry_program = Some(geometry_program);
        self.post_program = Some(post_program);
    } }

    pub fn make_screen_buffer(&mut self) { unsafe {
        let screen_vertices: [f32; 24] = [
            -1.0,  1.0,  0.0, 1.0,
            -1.0, -1.0,  0.0, 0.0,
             1.0, -1.0,  1.0, 0.0,
    
            -1.0,  1.0,  0.0, 1.0,
             1.0, -1.0,  1.0, 0.0,
             1.0,  1.0,  1.0, 1.0,
        ];
    
        let screen_buffer = Buffer::create();
        screen_buffer.upload(&screen_vertices, mem::size_of::<[f32; 24]>() as isize, gl::STATIC_DRAW);
        screen_buffer.bind_target(gl::ARRAY_BUFFER);
        gl::EnableVertexAttribArray(0);
        gl::VertexAttribPointer(0, 2, gl::FLOAT, gl::FALSE, 4 * mem::size_of::<f32>() as i32, ptr::null());
        gl::EnableVertexAttribArray(1);
        gl::VertexAttribPointer(1, 2, gl::FLOAT, gl::FALSE, 4 * mem::size_of::<f32>() as i32, (2 * mem::size_of::<f32>()) as *const c_void);
        self.screen_buffer = Some(screen_buffer);
    } }

    pub fn generate(&mut self) { unsafe {
        let noise = NoiseBuilder::gradient_3d(World::MAX_SECTION_X * 16, World::MAX_SECTION_Y * 16, World::MAX_SECTION_Z * 16).generate_scaled(0.0, 1.0);
        for x in 0..World::MAX_SECTION_X {
            for y in 0..World::MAX_SECTION_Y {
                for z in 0..World::MAX_SECTION_Z {
                    let mut section = Box::<Section>::new_zeroed().assume_init();
                    section.set_pos(Vector3 { x: x as i32, y: y as i32, z: z as i32 });
                    section.make_terrain(&noise);
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

        // let mut j = 0;
        let total = self.sections.len();
        for (i, section) in self.sections.values_mut().enumerate() {
            section.generate_geometry_buffer(&mut self.geometry_staging_buffer, &mut self.geometry_pool);
            section.generate_light_buffer(&mut self.geometry_staging_buffer, &mut self.geometry_pool, &mut self.light_staging_buffer, &mut self.light_pool);
            exact_geometry_bytes += self.geometry_staging_buffer.index;
            exact_light_bytes += self.light_staging_buffer.index;
            self.geometry_staging_buffer.reset();
            self.light_staging_buffer.reset();
            total_quads += section.quad_count as usize;

            // if j == 100 {
            //     println!("meshed {i}/{total} : {}%", i as f32 / total as f32 * 100.0);
            //     j = 0;
            //     continue;
            // }
            // j += 1;
        }
        
        let total_sections = self.sections.len();
        let elapsed = start.elapsed().as_millis();
        let sections_per_sec = (1000.0 / elapsed as f64 * total_sections as f64) as u64;
        let ms_per_section = elapsed as f64 / total_sections as f64;
        let quads_per_section = total_quads as u64 / total_sections as u64;
        println!("[6/6 axes] [merged] {total_sections} sections | {elapsed}ms | {sections_per_sec} sections/s | {ms_per_section}ms/section | {total_quads} quads | {quads_per_section} quads/section");
        let geometry_pool_bytes_used = self.geometry_pool.furthest * self.geometry_pool.block_size();
        let light_pool_bytes_used = self.light_pool.furthest * self.light_pool.block_size();
        let total_pool_megabytes_used = (geometry_pool_bytes_used + light_pool_bytes_used) / 1024 / 1024;
        println!("{geometry_pool_bytes_used} pooled geometry bytes | {light_pool_bytes_used} pooled light bytes | {total_pool_megabytes_used} pooled megabytes");
        let total_exact_megabytes_used = (exact_geometry_bytes + exact_light_bytes) / 1024 / 1024;
        println!("{exact_geometry_bytes} exact geometry bytes | {exact_light_bytes} exact light bytes | {total_exact_megabytes_used} megabytes");
    } }

    pub fn mesh(&mut self, pos: Vector3<i32>) {
        if let Some(section) = self.sections.get_mut(&pos) {
            section.generate_geometry_buffer(&mut self.geometry_staging_buffer, &mut self.geometry_pool);
            section.generate_light_buffer(&mut self.geometry_staging_buffer, &mut self.geometry_pool, &mut self.light_staging_buffer, &mut self.light_pool);
            self.geometry_staging_buffer.reset();
            self.light_staging_buffer.reset();
        }
    }

    pub fn add_section(&mut self, section: Box<Section>) { unsafe {
        let (x, y, z) = (section.pos.x, section.pos.y, section.pos.z);

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
        self.sections.insert(section.pos, section);
    } }

    pub fn remove_section(&mut self, pos: Vector3<i32>) { unsafe {
        if let Some(mut section) = self.sections.remove(&pos) {
            println!("removing section at {} {} {}", pos.x, pos.y, pos.z);
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
            self.geometry_pool.deallocate(section.geometry_page.take());
            self.light_pool.deallocate(section.light_page.take());
        }
    } }

    pub fn draw(&mut self) { unsafe {
        let framebuffer = self.framebuffer.as_ref().unwrap();

        self.geometry_program.as_ref().unwrap().bind();
        self.framebuffer.as_ref().unwrap().bind(gl::FRAMEBUFFER);
    
        let clear_color: [u32; 4] = [0, 0, 0, 0];
        gl::ClearNamedFramebufferuiv(framebuffer.id, gl::COLOR, 0, &raw const clear_color as *const u32);

        gl::ClearNamedFramebufferfi(framebuffer.id, gl::DEPTH_STENCIL, 0, 1.0, 0);

        gl::Enable(gl::DEPTH_TEST);

        let camera_matrix: [f32; 16] = *(self.camera.get_matrix().as_ref());
        gl::UniformMatrix4fv(0, 1, gl::FALSE, camera_matrix.as_ptr());
    
        let frustum = self.camera.get_frustum();
        const ELEMENT_INDICES_PER_QUAD: i32 = 5;
        const BYTES_PER_QUAD: usize = 8;
        const ELEMENTS_PER_QUAD: usize = 4;
        
        // do chunked frustum culling
        let mut drawnSections = 0;

        for section in self.sections.values() {
            // only render section if it has a geometry and light page
            if let (Some(geometry_page), Some(light_page)) = (&section.geometry_page, &section.light_page) {
                if frustum.test_sphere(section.get_bounding_sphere(&self.camera)) == Intersection::Outside { continue; }
                let count = section.quad_count as i32 * ELEMENT_INDICES_PER_QUAD;
                let basevertex = (geometry_page.start * geometry_page.block_size() / BYTES_PER_QUAD * ELEMENTS_PER_QUAD) as i32;
                self.counts.push(count);
                self.indices.push(0 as *const c_void);
                self.base_vertices.push(basevertex);
                drawnSections += 1;
            }
        }
        if drawnSections >= 1 {
            gl::MultiDrawElementsBaseVertex(
                gl::TRIANGLE_STRIP,
                self.counts.as_ptr(),
                gl::UNSIGNED_INT,
                self.indices.as_ptr(),
                drawnSections as i32,
                self.base_vertices.as_ptr()
            );
        }
        
        self.counts.set_len(0);
        self.indices.set_len(0);
        self.base_vertices.set_len(0);

        self.post_program.as_ref().unwrap().bind();

        gl::PolygonMode(gl::FRONT_AND_BACK, gl::FILL);
        FrameBuffer::clear_bind(gl::FRAMEBUFFER);
        gl::ClearColor(1.0, 1.0, 1.0, 1.0);
        gl::Clear(gl::COLOR_BUFFER_BIT);
        gl::Disable(gl::DEPTH_TEST);
        
        gl::ActiveTexture(gl::TEXTURE0);
        self.texture_attachment.as_ref().unwrap().bind(gl::TEXTURE_2D);
        gl::ActiveTexture(gl::TEXTURE1);
        self.block_texture.as_ref().unwrap().bind(gl::TEXTURE_2D_ARRAY);
        
        gl::DrawArrays(gl::TRIANGLES, 0, 6);
    } }
    
    pub const MAX_SECTION_X: usize = 32;
    pub const MAX_SECTION_Y: usize = 32;
    pub const MAX_SECTION_Z: usize = 32;
}

pub enum SectionType {
    Filled(Box<Section>),
    Empty,
    Unloaded,
}