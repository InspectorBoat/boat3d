use std::cell::{Ref, RefCell, UnsafeCell};
use std::collections::HashMap;
use std::ops::{Deref, Add};
use std::os::raw::c_void;
use std::{ptr, hint};
use std::{time, hint::black_box, alloc, mem};
use crate::block::blockface::{Normal, BlockFace};
use crate::util::byte_buffer::ByteBuffer;
use crate::util::gl_helper::Buffer;
use crate::world::chunk::{self, Vec3i};
use simdnoise::NoiseBuilder;

use super::{chunk::Chunk, camera::Camera};
#[derive(Debug)]
pub struct World<'a> {
    pub chunks: HashMap::<Vec3i, Box::<Chunk<'a>>>,
    pub camera: Camera
}

impl World<'_> {
    pub fn new() -> World<'static> { unsafe {
        let noise = NoiseBuilder::gradient_3d(512, 512, 512).generate_scaled(0.0, 1.0);
        
        let mut world = World {
            chunks: HashMap::<Vec3i, Box::<Chunk>>::new(),
            camera: Camera::new()
        };

        for x in 0..32 {
            for y in 0..32 {
                for z in 0..32 {
                    let mut chunk = unsafe { Box::<Chunk>::new_zeroed().assume_init() };
                    chunk.make_terrain(&noise, x, y, z);
                    chunk.create_buffer();
                    // world.add_chunk(chunk);
                    world.chunks.insert(chunk.pos, chunk);
                }
            }
        }
        
        let mut buffer = ByteBuffer::new();
        let start = time::Instant::now();
        let mut faces: usize = 0;
        let mesh_passes = 1;
        
        for _ in 0..mesh_passes {
            for x in 0..32 {
                for y in 0..32 {
                    for z in 0..32 {
                        // if x > 0 || y > 0 || z > 0 { continue }
                        let chunk = match world.chunks.get(&Vec3i { x, y, z }) {
                            Some(chunk) => chunk,
                            None => continue,
                        };
                        // I'm going to bomb an orphanage
                        let chunk = &mut *(&raw const chunk as *mut &mut Box<Chunk<'_>>);

                        // let chunk = &mut *(&raw mut world.chunks[(x << 10) | (y << 5) | (z << 0)]);

                        chunk.mesh_north_south_no_merge(&mut buffer, &world);
                        chunk.mesh_west_east_no_merge(&mut buffer, &world);
                        chunk.mesh_down_up_no_merge(&mut buffer, &world);

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

    pub fn add_chunk(&mut self, chunk: Box<Chunk<'_>>) { unsafe {
        // let (x, y, z) = (chunk.pos.x, chunk.pos.y, chunk.pos.z);
        // if let Some(south) = self.chunks.get_mut(&Vec3i { x, y, z: z - 1 }) {
        //     south.neighbors.north = Some(chunk);
        //     chunk.neighbors.south = Some(south);
        // }
        // if let Some(west) = self.chunks.get_mut(&Vec3i { x: x - 1, y, z }) {
        //     west.neighbors.east = Some(chunk);
        //     let west = &raw const west as *mut *mut *mut Chunk;
        //     let chunk = &raw const chunk as *mut *mut Chunk;
        //     chunk.neighbors.west = Some(west);
        // }
        // if let Some(down) = self.chunks.get_mut(&Vec3i { x, y: y - 1, z }) {
        //     down.neighbors.up = Some(chunk);
        //     let down = &raw const down as *mut *mut *mut Chunk;
        //     let chunk = &raw const chunk as *mut *mut Chunk;
        //     chunk.neighbors.down = Some(down);
        // }
        // if let Some(north) = self.chunks.get_mut(&Vec3i { x, y, z: z + 1 }) {
        //     north.neighbors.south = Some(chunk);
        //     let north = &raw const north as *mut *mut *mut Chunk;
        //     let chunk = &raw const chunk as *mut *mut Chunk;
        //     chunk.neighbors.north = Some(north);
        // }
        // if let Some(east) = self.chunks.get_mut(&Vec3i { x: x + 1, y, z }) {
        //     east.neighbors.west = Some(chunk);
        //     let east = &raw const east as *mut *mut *mut Chunk;
        //     let chunk = &raw const chunk as *mut *mut Chunk;
        //     (**chunk).neighbors.east = Some(east);
        // }
        // if let Some(up) = self.chunks.get_mut(&Vec3i { x, y: y + 1, z }) {
        //     up.neighbors.down = Some(chunk);
        //     let up = &raw const up as *mut *mut *mut Chunk;
        //     let chunk = &raw const chunk as *mut *mut Chunk;
        //     (**chunk).neighbors.up = Some(up);
        // }
        // self.chunks.insert((*chunk).pos, chunk);
    } }
}

impl Default for World<'_> {
    fn default() -> Self {
        Self::new()
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