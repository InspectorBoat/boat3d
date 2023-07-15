use std::cell::{Ref, RefCell, UnsafeCell};
use std::collections::HashMap;
use std::ops::{Deref, Add};
use std::os::raw::c_void;
use std::{ptr, hint};
use std::{time, hint::black_box, alloc, mem};
use crate::block::blockface::{Normal, BlockFace};
use crate::util::byte_buffer::StagingBuffer;
use crate::util::gl_helper::{Buffer, BufferPoolAllocator, log_if_error, Page, FrameBuffer, Texture, RenderBuffer, WindowStatus, Program, Shader};
use crate::world::chunk::{self, Vec3i};
use simdnoise::NoiseBuilder;

use super::{chunk::Chunk, camera::Camera};
#[derive(Debug)]
pub struct World {
    pub chunks: HashMap::<Vec3i, Box::<Chunk>>,
    pub camera: Camera,
    pub geometry_pool: BufferPoolAllocator<524288, 1024>,
    pub light_pool: BufferPoolAllocator<524288, 1024>,
    pub framebuffer: Option<FrameBuffer>,
    pub texture_attachment: Option<Texture>,
    pub renderbuffer_attachment: Option<RenderBuffer>,
    pub index_buffer: Option<Buffer>,
    pub geometry_program: Option<Program>,
    pub post_program: Option<Program>,
    pub screen_buffer: Option<Buffer>,
    pub block_texture: Option<Texture>
}

impl World {
    pub fn new() -> World { unsafe {
        let world = World {
            chunks: HashMap::<Vec3i, Box::<Chunk>>::new(),
            camera: Camera::new(),
            geometry_pool: BufferPoolAllocator::new(),
            light_pool: BufferPoolAllocator::new(),
            framebuffer: None,
            texture_attachment: None,
            renderbuffer_attachment: None,
            index_buffer: None,
            geometry_program: None,
            post_program: None,
            screen_buffer: None,
            block_texture: None,
        };

        return world;
    } }

    pub fn make_block_texture(&mut self) { unsafe {
        gl::ActiveTexture(gl::TEXTURE0);
        let block_texture = Texture::create();
        block_texture.bind(gl::TEXTURE_2D_ARRAY);
        let image: [u8; 16 * 3 * 64] = [0; 16 * 3 * 64].map(|_| rand::random());
        gl::TexImage3D(gl::TEXTURE_2D_ARRAY, 0, gl::RGBA as i32, 4, 4, 64, 0, gl::RGB, gl::BYTE, &image as *const u8 as *const c_void);
        gl::TexParameteri(gl::TEXTURE_2D_ARRAY, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as i32);
        gl::TexParameteri(gl::TEXTURE_2D_ARRAY, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as i32);
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
        gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RGBA32F as i32, status.width as i32, status.height as i32, 0, gl::RGBA, gl::BYTE, ptr::null());
        // gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RGBA32UI as i32, status.width as i32, status.height as i32, 0, gl::RGBA_INTEGER, gl::INT, ptr::null());
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
        FrameBuffer::texture2d_attachment(gl::FRAMEBUFFER, gl::COLOR_ATTACHMENT0, gl::TEXTURE_2D, &texture_attachment, 0);
    
        let renderbuffer_attachment = RenderBuffer::create();
        renderbuffer_attachment.bind(gl::RENDERBUFFER);
        gl::RenderbufferStorage(gl::RENDERBUFFER, gl::DEPTH24_STENCIL8, status.width as i32, status.height as i32);  
        FrameBuffer::renderbuffer_attachment(gl::FRAMEBUFFER, gl::DEPTH_STENCIL_ATTACHMENT, gl::RENDERBUFFER, &renderbuffer_attachment);
        
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
        index_buffer.storage(1024 * 1024 / 4, gl::DYNAMIC_STORAGE_BIT);
        index_buffer.upload_slice(&index_array.as_slice(), 0, index_array.len() as isize);
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
        let noise = NoiseBuilder::gradient_3d(512, 512, 512).generate_scaled(0.0, 1.0);
        for x in 0..32 {
            for y in 0..32 {
                for z in 0..32 {
                    let mut chunk = Box::<Chunk>::new_zeroed().assume_init();
                    chunk.make_terrain(&noise, x, y, z);
                    // chunk.make_terrain_alt(x, y, z);
                    self.add_chunk(chunk);
                }
            }
        }
    } }

    pub fn mesh(&mut self) { unsafe {
        let mut geometry_staging_buffer = StagingBuffer::new();
        let mut light_staging_buffer = StagingBuffer::new();
        let start = time::Instant::now();
        let mut quads: usize = 0;
        let mesh_passes = 1;
        

        for _ in 0..mesh_passes {
            for chunk in self.chunks.values_mut() {
                // if chunk.pos.x == 0 || chunk.pos.y == 0 || chunk.pos.z == 0 { continue; }
                chunk.generate_geometry_buffer(&mut geometry_staging_buffer, &mut self.geometry_pool);
                chunk.generate_light_buffer(&mut geometry_staging_buffer, &mut light_staging_buffer, &mut self.light_pool);
                geometry_staging_buffer.reset();
                light_staging_buffer.reset();
                quads += chunk.quad_count as usize;
            }
        }
        
        
        
        let count = self.chunks.len() * mesh_passes;
        let elapsed = start.elapsed().as_millis();
        
        // println!("[6/6 axes] [merged] {count} chunks | {}ms | {} chunks/s | {}ms/chunk | {} quads | {} quads/chunk", elapsed, (1000.0 / elapsed as f64 * count as f64) as u64, elapsed as f64 / count as f64, quads, quads as u64 / count as u64);
    } }

    pub fn add_chunk(&mut self, chunk: Box<Chunk>) { unsafe {
        let (x, y, z) = (chunk.pos.x, chunk.pos.y, chunk.pos.z);
        
        // into_raw must be called to prevent rust from dropping the chunk
        // Cast into usize and back to prevent rust from realizing the unboxed chunk is in fact the same chunk that was passed in
        
        let chunk = Box::<Chunk>::into_raw(chunk) as usize as *mut Chunk;
        if let Some(south) = self.chunks.get_mut(&Vec3i { x, y, z: z - 1 }) {
            south.neighbors.north = Some(chunk);
            (*chunk).neighbors.south = Some(&raw mut **south);
        }
        if let Some(west) = self.chunks.get_mut(&Vec3i { x: x - 1, y, z }) {
            west.neighbors.east = Some(chunk);
            (*chunk).neighbors.west = Some(&raw mut **west);
        }
        if let Some(down) = self.chunks.get_mut(&Vec3i { x, y: y - 1, z }) {
            down.neighbors.up = Some(chunk);
            (*chunk).neighbors.down = Some(&raw mut **down);
        }
        if let Some(north) = self.chunks.get_mut(&Vec3i { x, y, z: z + 1 }) {
            north.neighbors.south = Some(chunk);
            (*chunk).neighbors.north = Some(&raw mut **north);
        }
        if let Some(east) = self.chunks.get_mut(&Vec3i { x: x + 1, y, z }) {
            east.neighbors.west = Some(chunk);
            (*chunk).neighbors.east = Some(&raw mut **east);
        }
        if let Some(up) = self.chunks.get_mut(&Vec3i { x, y: y + 1, z }) {
            up.neighbors.down = Some(chunk);
            (*chunk).neighbors.up = Some(&raw mut **up);
        }

        let chunk = Box::from_raw(*(&raw const chunk as *mut *mut Chunk));
        self.chunks.insert(chunk.pos, chunk);
    } }

    pub fn remove_chunk(&mut self, pos: Vec3i) { unsafe {
        if let Some(mut chunk) = self.chunks.remove(&pos) {
            println!("removing chunk at {} {} {}", pos.x, pos.y, pos.z);
            chunk.neighbors.south.inspect(|south| (**south).neighbors.north = None);
            chunk.neighbors.west.inspect(|west| (**west).neighbors.east = None);
            chunk.neighbors.down.inspect(|down| (**down).neighbors.up = None);
            chunk.neighbors.north.inspect(|north| (**north).neighbors.south = None);
            chunk.neighbors.east.inspect(|east| (**east).neighbors.west = None);
            chunk.neighbors.up.inspect(|up| (**up).neighbors.down = None);
            self.geometry_pool.deallocate(chunk.geometry_page.take());
        }
    } }
}