use std::{time, hint::black_box};

use crate::util::{buffer::ByteBuffer, gl_helper::{log_if_error, log_error, Buffer}};
use simdnoise::NoiseBuilder;


use super::{chunk::Chunk, camera::Camera};

pub struct World<'a> {
    pub chunks: Box<[Chunk<'a>; 4096]>,
    pub camera: Camera
}

impl <'a> World<'a> {
    pub fn new() -> World<'a> {
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
                // if x >= 1 || y >= 1 || z >= 1 { continue }
                // chunk.mesh_west(&mut buffer);
                // chunk.mesh_down(&mut buffer);
                // chunk.mesh_south(&mut buffer);
                // chunk.mesh_north(&mut buffer);
                chunk.mesh_north_south(&mut buffer);

                chunk.face_count = (buffer.pos as u32) / 8;

                black_box(&buffer);

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

    pub fn iter(&mut self) -> WorldIterator<'a> {
        return WorldIterator {
            chunks: self.chunks.as_mut_ptr() as *mut [Chunk; 4096],
            i: 0
        };
    }
}

pub struct WorldIterator<'a> {
    chunks: *mut [Chunk<'a>; 4096],
    i: usize
}

impl<'a> Iterator for WorldIterator<'a> {
    type Item = (&'a mut Chunk<'a>, usize, usize, usize, usize);

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

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub enum AxisOrder {
    XYZ,
    YXZ,
    ZYX
}

