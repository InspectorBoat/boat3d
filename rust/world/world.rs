use std::ops::{Deref, Add};
use std::os::raw::c_void;
use std::{ptr, hint};
use std::{time, hint::black_box, alloc, mem};
use crate::block::blockface::{Normal, BlockFace};
use crate::util::byte_buffer::ByteBuffer;
use crate::util::gl_helper::Buffer;
use crate::world::chunk;
use simdnoise::NoiseBuilder;

use super::{chunk::Chunk, camera::Camera};
#[derive(Debug)]
pub struct World {
    pub chunks: Box<[Chunk; 32768]>,
    pub camera: Camera
}

impl World {
    pub fn new() -> World { unsafe {
        let chunks = unsafe { Box::<[Chunk; 32768]>::new_zeroed().assume_init() };
        let noise = NoiseBuilder::gradient_3d(512, 512, 512);
        let mut noise_vec = noise.generate_scaled(0.0, 1.0);
        
        let mut world = World {
            chunks,
            camera: Camera::new()
        };

        let mut random = Lcg { val: 1 };
        let mut counter = 0;
        for x in 0..32 {
            for y in 0..32 {
                for z in 0..32 {
                    world.chunks[(x << 10) | (y << 5) | (z << 0)].make_terrain_alt(&mut random, &mut counter);
                    world.chunks[(x << 10) | (y << 5) | (z << 0)].create_buffer();
                }
            }
        }
        println!("counter: {counter}");
        for (chunk, x, y, z, _) in world.iter() {
            // if x != 0 || y != 0 || z != 0 { continue }
            // chunk.make_terrain_alt(&mut random);
            // chunk.make_terrain(&mut noise_vec, x, y, z);
            // chunk.create_buffer();
            // let buffer = unsafe { chunk.buffer.take().unwrap_unchecked() };
            // buffer.storage(4096, gl::DYNAMIC_STORAGE_BIT);
            // chunk.buffer = Some(buffer);
        }
        
        let mut buffer = ByteBuffer::new();
        let start = time::Instant::now();
        let mut faces: usize = 0;
        let mesh_passes = 1;
        
        for _ in 0..mesh_passes {
            let mut north_faces: [BlockFace; 256] = [BlockFace::NONE2; 256];
            for x in 0..32 {
                for y in 0..32 {
                    for z in 0..32 {
                        if x > 0 || y > 0 || z > 0 { continue }
                        let chunk = &mut *(&raw mut world.chunks[(x << 10) | (y << 5) | (z << 0)]);
                        // if x != 0 || y != 0 || z != 0 { continue }

                        if z == 0 {
                            north_faces = [BlockFace::NONE2; 256];
                        }

                        chunk.mesh_north_south_no_merge(&mut buffer, &mut north_faces, &world);

                        buffer.format_quads();
                        
                        chunk.face_count = (buffer.ind as u32) / 8;
                        faces += chunk.face_count as usize;
                        
                        black_box(&buffer);

                        // /*
                        if chunk.face_count == 0 { chunk.kill_buffer(); continue }
                        let gl_buffer = unsafe { chunk.buffer.take().unwrap_unchecked() };
                        gl_buffer.storage((buffer.ind + 16) as isize, gl::DYNAMIC_STORAGE_BIT);
                        gl_buffer.upload_slice(&[chunk.pos.x, chunk.pos.y, chunk.pos.z, 0], 0, 16);
                        gl_buffer.upload_slice(&buffer.arr.as_slice(), 16, buffer.ind as isize);
                        
                        chunk.buffer = Some(gl_buffer);
                        // */

                        buffer.reset();
                    }
                }
            }
        }
        
        
        
        let count = world.chunks.len() * mesh_passes;
        let elapsed = start.elapsed().as_millis();
        
        println!("[2/6 axes] [No merge] {count} chunks | {}ms | {} chunks/s | {}ms/chunk | {} faces | {} faces/chunk", elapsed, 1000.0 / elapsed as f64 * count as f64, elapsed as f64 / count as f64, faces, faces as u64 / count as u64);
        return world;
    } }

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

    fn next(&mut self) -> Option<Self::Item> { unsafe {
        return if self.i < 32768 {
            let x = (self.i & 0x7fff) >> 10;
            let y = (self.i & 0x3ff) >> 5;
            let z = (self.i & 0x01f) >> 0;
            let val = Some((
                (*self.chunks).get_unchecked_mut(self.i),
                x, y, z,
                self.i
            ));
            self.i += 1;
            return val;
        } else {
            None
        }
    } }
}
impl Add<usize> for WorldIterator {
    type Output = WorldIterator;

    fn add(self, rhs: usize) -> Self::Output {
        todo!()
    }
}

pub struct Lcg {
    pub val: u32,
}
impl Lcg {
    pub fn next(&mut self) -> u32 {
        self.val = (self.val * 1103515245 + 12346) % (2147483648 - 1);
        return self.val;
    }
}