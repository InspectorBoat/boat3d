use std::ops::Deref;
use std::{ptr, hint};
use std::{time, hint::black_box, alloc, mem};
use crate::util::byte_buffer::ByteBuffer;
use crate::util::gl_helper::Buffer;
use simdnoise::NoiseBuilder;
extern crate libc;

use super::{chunk::Chunk, camera::Camera};
#[derive(Debug)]
pub struct World {
    pub chunks: Box<[Chunk; 32768]>,
    pub camera: Camera
}

impl World {
    pub fn new() -> World {
        let chunks = unsafe { Box::<[Chunk; 32768]>::new_zeroed().assume_init() };
        let noise = NoiseBuilder::gradient_3d(512, 512, 512);
        let mut noise_vec = noise.generate_scaled(0.0, 1.0);
        
        let mut world = World {
            chunks,
            camera: Camera::new()
        };

        for (chunk, x, y, z, _) in world.iter() {
            // if x >= 1 || y >= 1 || z >= 2 { continue }
            chunk.make_terrain(&mut noise_vec, x, y, z);
            chunk.create_buffer();
            // let buffer = unsafe { chunk.buffer.take().unwrap_unchecked() };
            // buffer.storage(4096, gl::DYNAMIC_STORAGE_BIT);
            // chunk.buffer = Some(buffer);
        }
        
        let mut buffer = ByteBuffer::new();
        let start = time::Instant::now();
        let mut faces = 0;
        let iter = 1;
        for _ in 0..iter {
            for (chunk, x, y, z, _) in world.iter() {
                // if x >= 1 || y >= 1 || z >= 1 { continue }
                chunk.mesh_north_south(&mut buffer, &world);
                chunk.mesh_west_east(&mut buffer, &world);
                chunk.mesh_down_up(&mut buffer, &world);

                // chunk.mesh_north_south_no_merge(&mut buffer, &world);
                buffer.format_quads();
                
                chunk.face_count = (buffer.ind as u32) / 8;
                faces += chunk.face_count;
                
                // black_box(&buffer);
                // /*
                if chunk.face_count == 0 { continue }
                let gl_buffer = unsafe { chunk.buffer.take().unwrap_unchecked() };

                gl_buffer.storage((buffer.ind + 16) as isize, gl::DYNAMIC_STORAGE_BIT);
                gl_buffer.upload_slice(&[chunk.chunk_pos.x, chunk.chunk_pos.y, chunk.chunk_pos.z, 0], 0, 16);
                gl_buffer.upload_slice(&buffer.arr.as_slice(), 16, buffer.ind as isize);
                
                chunk.buffer = Some(gl_buffer);
                // */
                buffer.ind = 0;
            }
        }
        let count = world.chunks.len() * iter;
        let elapsed = start.elapsed().as_millis();
        println!("[6/6 axes] {count} chunks | {}ms | {}ms/chunk | {} faces | {} faces/chunk", elapsed, elapsed as f64 / count as f64, faces, faces as u64 / count as u64);
        return world;
    }

    pub fn iter(&mut self) -> WorldIterator {
        return WorldIterator {
            chunks: &mut *self.chunks as *mut [Chunk; 32768],
            i: 0
        };
    }
}

pub struct WorldIterator {
    chunks: *mut [Chunk; 32768],
    i: usize
}

impl Iterator for WorldIterator {
    type Item = (&'static mut Chunk, usize, usize, usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        return if self.i < 32768 { unsafe {
            let val = Some((
                (*self.chunks).get_unchecked_mut(self.i),
                (self.i & 0x7fff) >> 10, (self.i & 0x3ff) >> 5, (self.i & 0x01f) >> 0,
                self.i
            ));
            self.i += 1;
            return val;
        } } else {
            None
        }
    }
}