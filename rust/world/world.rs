use std::ops::Deref;
use std::os::raw::c_void;
use std::{ptr, hint};
use std::{time, hint::black_box, alloc, mem};
use crate::block::blockface::Normal;
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
            // if x != 0 || y != 0 || z != 0 { continue }
            chunk.make_terrain(&mut noise_vec, x, y, z);
            chunk.create_buffer();
            // let buffer = unsafe { chunk.buffer.take().unwrap_unchecked() };
            // buffer.storage(4096, gl::DYNAMIC_STORAGE_BIT);
            // chunk.buffer = Some(buffer);
        }
        
        let mut buffer = ByteBuffer::new();
        let mut buffer2 = ByteBuffer::new();
        let start = time::Instant::now();
        let mut faces = 0;
        let iter = 1;
        for _ in 0..iter {
            for (chunk, x, y, z, _) in world.iter() {
                // if x != 0 || y != 0 || z != 0 { continue }

                let prev_ind = buffer.ind;
                let prev_ind2 = buffer2.ind;
                chunk.mesh_north_south(&mut buffer, &mut buffer2, &world);
                chunk.counts[0] = ((buffer.ind - prev_ind) / 8 * 5) as isize;
                chunk.counts[3] = ((buffer2.ind - prev_ind2) / 8 * 5) as isize;

                let prev_ind = buffer.ind;
                let prev_ind2 = buffer2.ind;
                chunk.mesh_west_east(&mut buffer, &mut buffer2, &world);
                chunk.counts[1] = ((buffer.ind - prev_ind) / 8 * 5) as isize;
                chunk.counts[4] = ((buffer2.ind - prev_ind2) / 8 * 5) as isize;

                let p_ind = buffer.ind;
                let p_ind2 = buffer2.ind;
                chunk.mesh_down_up(&mut buffer, &mut buffer2, &world);
                chunk.counts[2] = ((buffer.ind - p_ind) / 8 * 5) as isize;
                chunk.counts[5] = ((buffer2.ind - p_ind2) / 8 * 5) as isize;

                // TODO:
                // This is REALLY BAD, but for some reason using a i32/u32 causes a crash
                // Since OpenGL can only read 8/16/32 bit ints, half the indices aren't reached
                // I worked around this in a really terrible way by giving each element twice as much padding and reading twice the elements
                // This is killing framerate
                // FIX THIS LATER
                chunk.offsets[0] = 0;
                chunk.offsets[1 * 2] = chunk.offsets[0 * 2] + (chunk.counts[0] * 4) as isize;
                chunk.offsets[2 * 2] = chunk.offsets[1 * 2] + (chunk.counts[1] * 4) as isize;
                chunk.offsets[3 * 2] = chunk.offsets[2 * 2] + (chunk.counts[2] * 4) as isize;
                chunk.offsets[4 * 2] = chunk.offsets[3 * 2] + (chunk.counts[3] * 4) as isize;
                chunk.offsets[5 * 2] = chunk.offsets[4 * 2] + (chunk.counts[4] * 4) as isize;
                
                buffer.format_quads();
                buffer2.format_quads();

                chunk.face_count = (buffer.ind as u32 + buffer2.ind as u32) / 8;
                faces += chunk.face_count;
                
                // black_box(&buffer);
                // /*
                if chunk.face_count == 0 { continue }
                let gl_buffer = unsafe { chunk.buffer.take().unwrap_unchecked() };

                gl_buffer.storage((buffer.ind + buffer2.ind + 16) as isize, gl::DYNAMIC_STORAGE_BIT);
                gl_buffer.upload_slice(&[chunk.chunk_pos.x, chunk.chunk_pos.y, chunk.chunk_pos.z, 0], 0, 16);
                gl_buffer.upload_slice(&buffer.arr.as_slice(), 16, buffer.ind as isize);
                gl_buffer.upload_slice(&buffer2.arr.as_slice(), 16 + buffer.ind as isize, buffer2.ind as isize);
                
                chunk.buffer = Some(gl_buffer);
                // */
                buffer.reset();
                buffer2.reset();
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