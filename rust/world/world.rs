use std::cell::{Ref, RefCell, UnsafeCell};
use std::collections::HashMap;
use std::ops::{Deref, Add};
use std::os::raw::c_void;
use std::{ptr, hint};
use std::{time, hint::black_box, alloc, mem};
use crate::block::blockface::{Normal, BlockFace};
use crate::util::byte_buffer::StagingBuffer;
use crate::util::gl_helper::{Buffer, BufferPoolAllocator, log_if_error, Page, FrameBuffer, Texture, RenderBuffer, WindowStatus};
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
    pub renderbuffer_attachment: Option<RenderBuffer>
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
        };
        
        return world;
    } }

    pub fn make_framebuffer(&mut self, status: &WindowStatus) { unsafe {
        let framebuffer = FrameBuffer::create();
        framebuffer.bind(gl::FRAMEBUFFER);
    
        let texture_attachment = Texture::create();
        Texture::active(0);
        texture_attachment.bind(gl::TEXTURE_2D);
        gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RGBA32F as i32, status.width as i32, status.height as i32, 0, gl::RGBA, gl::UNSIGNED_BYTE, ptr::null());
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

    pub fn generate(&mut self) { unsafe {
        let noise = NoiseBuilder::gradient_3d(512, 512, 512).generate_scaled(0.0, 1.0);
        for x in 0..32 {
            for y in 0..32 {
                for z in 0..32 {
                    let mut chunk = Box::<Chunk>::new_zeroed().assume_init();
                    chunk.make_terrain(&noise, x, y, z);
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
                // if chunk.pos.x != 5 || chunk.pos.y != 2 || chunk.pos.z != 9 { continue; }
                chunk.generate_geometry_buffer(&mut geometry_staging_buffer, &mut self.geometry_pool);
                chunk.generate_light_buffer(&mut geometry_staging_buffer, &mut light_staging_buffer, &mut self.light_pool);
                geometry_staging_buffer.reset();
                light_staging_buffer.reset();
                quads += chunk.quad_count as usize;
                // break;
            }
        }
        
        
        
        let count = self.chunks.len() * mesh_passes;
        let elapsed = start.elapsed().as_millis();
        
        println!("[6/6 axes] [merged] {count} chunks | {}ms | {} chunks/s | {}ms/chunk | {} quads | {} quads/chunk", elapsed, (1000.0 / elapsed as f64 * count as f64) as u64, elapsed as f64 / count as f64, quads, quads as u64 / count as u64);
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