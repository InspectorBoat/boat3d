use std::{time, hint::black_box, alloc, mem};
use std::alloc::{alloc, System};
use crate::util::buffer::ByteBuffer;
use crate::util::gl_helper::Buffer;
use simdnoise::NoiseBuilder;
extern crate libc;

use super::{chunk::Chunk, camera::Camera};

pub struct World {
    pub chunks: Box<[Chunk; 4096]>,
    pub camera: Camera
}

impl World {
    pub fn new() -> World {
        let mut chunks: Vec<Chunk> = Vec::with_capacity(4096);
        unsafe { chunks.set_len(4096) }
        
        let chunks = Box::into_raw(chunks.into_boxed_slice());

        let chunks = unsafe { Box::from_raw(chunks as *mut [Chunk; 4096]) };

        let noise = NoiseBuilder::gradient_3d(256, 256, 256);
        let mut noise_vec = noise.generate_scaled(0.0, 1.0);
        
        let mut world = World {
            chunks,
            camera: Camera::new()
        };

        for (chunk, x, y, z, _) in world.iter() {
            chunk.make_terrain(&mut noise_vec, x, y, z);
            chunk.create_buffer();
        }
        
        let mut buffer = ByteBuffer::new();
        let start = time::Instant::now();
        let mut faces = 0;
        for _ in 0..100 {
            for (chunk, x, y, z, _) in world.iter() {
                // if x >= 8 || y >= 8 || z >= 8 { continue }
                chunk.mesh_north_south(&mut buffer);

                chunk.face_count = (buffer.pos as u32) / 8;

                // if chunk.face_count == 0 {
                    // continue;
                // }
                // let gl_buffer: Buffer;
                
                // unsafe {
                    // gl_buffer = chunk.buffer.take().unwrap_unchecked();
                // }
                
                // gl_buffer.storage((buffer.pos + 16) as isize, gl::DYNAMIC_STORAGE_BIT);
                // gl_buffer.upload_slice(&[chunk.chunk_pos.x, chunk.chunk_pos.y, chunk.chunk_pos.z, 0], 0, 16);
                // gl_buffer.upload_slice(&buffer.arr.as_slice(), 16, buffer.pos as isize);
                
                // chunk.buffer = Some(gl_buffer);
                
                faces += buffer.pos / 8;
                buffer.pos = 0;
            }
        }
        println!("meshed 409600 chunks in {}ms | {} faces", start.elapsed().as_millis(), faces);
        return world;
    }

    pub fn iter(&mut self) -> WorldIterator {
        return WorldIterator {
            chunks: &mut *self.chunks as *mut [Chunk; 4096],
            i: 0
        };
    }
}

pub struct WorldIterator {
    chunks: *mut [Chunk; 4096],
    i: usize
}

impl Iterator for WorldIterator {
    type Item = (&'static mut Chunk, usize, usize, usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        return if self.i < 4096 { unsafe {
            let val = Some((
                (*self.chunks).get_unchecked_mut(self.i),
                (self.i & 0xf00) >> 8, (self.i & 0x0f0) >> 4, (self.i & 0x00f) >> 0,
                self.i
            ));
            self.i += 1;
            return val;
        } } else {
            None
        }
    }
}