use std::cell::{Ref, RefCell, UnsafeCell};
use std::collections::HashMap;
use std::ops::{Deref, Add};
use std::os::raw::c_void;
use std::{ptr, hint};
use std::{time, hint::black_box, alloc, mem};
use crate::block::blockface::{Normal, BlockFace};
use crate::util::byte_buffer::ByteBuffer;
use crate::util::gl_helper::{Buffer, PoolAllocator, log_if_error, Page};
use crate::world::chunk::{self, Vec3i};
use simdnoise::NoiseBuilder;

use super::{chunk::Chunk, camera::Camera};
#[derive(Debug)]
pub struct World<'a> {
    pub chunks: HashMap::<Vec3i, Box::<Chunk<'a>>>,
    pub camera: Camera,
    pub pool: PoolAllocator
}

impl World<'_> {
    pub fn new() -> World<'static> { unsafe {
        let noise = NoiseBuilder::gradient_3d(512, 512, 512).generate_scaled(0.0, 1.0);
        
        let mut world = World {
            chunks: HashMap::<Vec3i, Box::<Chunk>>::new(),
            camera: Camera::new(),
            pool: PoolAllocator::new()
        };

        for x in 0..32 {
            for y in 0..32 {
                for z in 0..32 {
                    let mut chunk = unsafe { Box::<Chunk>::new_zeroed().assume_init() };
                    chunk.make_terrain(&noise, x, y, z);
                    world.add_chunk(chunk);
                }
            }
        }
        
        let mut buffer = ByteBuffer::new();
        // let mut buffer2 = ByteBuffer::new();
        let start = time::Instant::now();
        let mut faces: usize = 0;
        let mesh_passes = 1;
        
        for _ in 0..mesh_passes {
            for x in 0..32 {
                for y in 0..32 {
                    for z in 0..32 {
                        if let Some(chunk) = world.chunks.get_mut(&Vec3i { x, y, z }) {

                            chunk.mesh_south_north(&mut *(&raw const buffer as *mut ByteBuffer), &mut *(&raw const buffer as *mut ByteBuffer));
                            chunk.mesh_west_east(&mut *(&raw const buffer as *mut ByteBuffer), &mut *(&raw const buffer as *mut ByteBuffer));
                            chunk.mesh_down_up(&mut *(&raw const buffer as *mut ByteBuffer), &mut *(&raw const buffer as *mut ByteBuffer));

                            // chunk.mesh_south_north_no_merge(&mut buffer);
                            // chunk.mesh_west_east_no_merge(&mut buffer);
                            // chunk.mesh_down_up_no_merge(&mut buffer);
    
                            buffer.format_quads();
    
                            chunk.face_count = (buffer.ind as u32) / 8;
                            faces += chunk.face_count as usize;
                            
                            chunk.page = world.pool.allocate(buffer.ind);
                            if let Some(page) = &chunk.page {
                                world.pool.upload(page, &buffer.arr.as_slice(), buffer.ind as isize);
                            }

                            buffer.reset();

                            // buffer2.reset();
                        }

                    }
                }
            }
        }
        
        
        
        let count = world.chunks.len() * mesh_passes;
        let elapsed = start.elapsed().as_millis();
        
        println!("[6/6 axes] [merged] {count} chunks | {}ms | {} chunks/s | {}ms/chunk | {} faces | {} faces/chunk", elapsed, (1000.0 / elapsed as f64 * count as f64) as u64, elapsed as f64 / count as f64, faces, faces as u64 / count as u64);
        return world;
    } }

    pub fn mesh_chunk() {}

    // avert your eyes
    pub fn add_chunk(&mut self, chunk: Box<Chunk<'_>>) { unsafe {

        let (x, y, z) = (chunk.pos.x, chunk.pos.y, chunk.pos.z);
        
        // into_raw must be called to prevent rust from dropping the chunk
        // Cast into usize and back to prevent rust from realizing the unboxed chunk is in fact the same chunk that was passed in

        let chunk = Box::<Chunk<'_>>::into_raw(chunk) as usize as *mut Chunk;
        if let Some(south) = self.chunks.get_mut(&Vec3i { x, y, z: z - 1 }) {
            south.neighbors.north = Some(chunk);
            (*chunk).neighbors.south = Some(**(&raw const south as *const *const *mut Chunk));
        }
        if let Some(west) = self.chunks.get_mut(&Vec3i { x: x - 1, y, z }) {
            west.neighbors.east = Some(chunk);
            (*chunk).neighbors.west = Some(**(&raw const west as *const *const *mut Chunk));
        }
        if let Some(down) = self.chunks.get_mut(&Vec3i { x, y: y - 1, z }) {
            down.neighbors.up = Some(chunk);
            (*chunk).neighbors.down = Some(**(&raw const down as *const *const *mut Chunk));
        }
        if let Some(north) = self.chunks.get_mut(&Vec3i { x, y, z: z + 1 }) {
            north.neighbors.south = Some(chunk);
            (*chunk).neighbors.north = Some(**(&raw const north as *const *const *mut Chunk));
        }
        if let Some(east) = self.chunks.get_mut(&Vec3i { x: x + 1, y, z }) {
            east.neighbors.west = Some(chunk);
            (*chunk).neighbors.east = Some(**(&raw const east as *const *const *mut Chunk));
        }
        if let Some(up) = self.chunks.get_mut(&Vec3i { x, y: y + 1, z }) {
            up.neighbors.down = Some(chunk);
            (*chunk).neighbors.up = Some(**(&raw const up as *const *const *mut Chunk));
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
            self.pool.deallocate(chunk.page.take());
        }
    } }
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