use core::panic;
use core::slice;
use std::arch::asm;
use std::cell::Cell;
use std::cell::OnceCell;
use std::cell::UnsafeCell;
use std::fmt::DebugStruct;
use std::hint;
use std::hint::black_box;
use std::hint::unreachable_unchecked;
use std::intrinsics::prefetch_read_data;
use std::mem;
use std::num::NonZeroUsize;
use std::ops::Add;
use std::ops::Deref;
use std::ops::DerefMut;
// use std::mem;
// use std::ops::BitAnd;
use std::ops::Index;
use std::ops::Sub;
// use std::ops::Add;
// use std::ops::SubAssign;
use std::os::raw::c_void;
// use std::ptr;
use std::hash::Hash;
use std::ptr::NonNull;
use cgmath::Vector3;
use cgmath_culling::BoundingBox;
use cgmath_culling::Sphere;
use gl::BLOCK_INDEX;

use crate::OTHER_FACES;
use crate::block::blockface::GpuQuad;
use crate::util::gl_helper::Page;
use crate::util::gl_helper::BufferPoolAllocator;
use crate::{block::{blockstate::BlockState, blockface::{Normal, BlockFace}}, util::{gl_helper::{Buffer, log_if_error, log_error}, byte_buffer::StagingBuffer}, BLOCKS};
use super::camera::Camera;
use super::world::World;
#[derive(Debug)]
pub struct Chunk {
    // Block IDs
    pub blocks: [u8; 4096],
    // Light levels 0-15
    pub light: [u8; 4096],

    // Chunk position
    pub pos: Vector3<i32>,
    // Adjacent chunks
    pub neighbors: Neighbors,
    // Number of rectangular block faces in a chunk
    pub quad_count: u32,

    pub geometry_page: Option<Page<1024>>,
    pub light_page: Option<Page<1024>>
}

#[derive(Debug)]
pub struct Neighbors {
    pub south: Option<NonNull<Chunk>>,
    pub west: Option<NonNull<Chunk>>,
    pub down: Option<NonNull<Chunk>>,
    pub north: Option<NonNull<Chunk>>,
    pub east: Option<NonNull<Chunk>>,
    pub up: Option<NonNull<Chunk>>,
}
impl Chunk {
    pub fn get_neighbor<const N: Normal>(&self) -> Option<&Chunk> { unsafe {
        match N {
            Normal::NORTH => {
                return mem::transmute(self.neighbors.north);
            }
            Normal::SOUTH => {
                return mem::transmute(self.neighbors.south);
            }
            Normal::EAST => {
                return mem::transmute(self.neighbors.east);
            }
            Normal::WEST => {
                return mem::transmute(self.neighbors.west);
            }
            Normal::UP => {
                return mem::transmute(self.neighbors.up);
            }
            Normal::DOWN => {
                return mem::transmute(self.neighbors.down);
            }
            _ => { unreachable_unchecked(); }
        }
    } }
    
    pub fn get_block(&self, index: usize) -> &BlockState { unsafe {
        return &BLOCKS[*self.blocks.get_unchecked(index) as usize];
    } }
    pub fn get_opposing_block<const N: Normal>(&self, index: usize) -> &BlockState { unsafe {
        match N {
            Normal::SOUTH => {
                if index & 0x00f == 0 {
                    return self.get_neighbor::<N>().map_or(&BLOCKS[0], |chunk| chunk.get_block(index | 0x00f));
                }
                return &self.get_block(index - 0x001);
            }
            Normal::WEST => {
                if index & 0xf00 == 0 {
                    return self.get_neighbor::<N>().map_or(&BLOCKS[0], |chunk| chunk.get_block(index | 0xf00));
                }
                return &self.get_block(index - 0x100);
            }
            Normal::DOWN => {
                if index & 0x0f0 == 0 {
                    return self.get_neighbor::<N>().map_or(&BLOCKS[0], |chunk| chunk.get_block(index | 0x0f0));
                }
                return &self.get_block(index - 0x010);
            }
            _ => unsafe { hint::unreachable_unchecked(); }
        }
    } }
    
    pub fn get_face<const N: Normal>(&self, index: usize) -> &BlockFace {
        return &self.get_block(index).model.get_face(N);
    }
    pub fn get_opposing_face<const N: Normal>(&self, index: usize) -> &BlockFace {
        return &self.get_opposing_block::<N>(index).model.get_face(N.reverse());
    }
    
    pub fn get_light<const N: Normal>(&self, index: usize) -> u8 {
        match N {
            Normal::NORTH => {
                if index & 0x00f == 0x00f {
                    return self.get_neighbor::<N>().map_or(0, |chunk| chunk.light[index & 0xff0]);
                }
                return self.light[index + 0x001];
            }
            Normal::SOUTH => {
                if index & 0x00f == 0 {
                    return self.get_neighbor::<N>().map_or(0, |chunk| chunk.light[index | 0x00f]);
                }
                return self.light[index - 0x001];
            }
            Normal::EAST => {
                if index & 0xf00 == 0xf00 {
                    return self.get_neighbor::<N>().map_or(0, |chunk| chunk.light[index & 0x0ff]);
                }
                return self.light[index + 0x100];
            }
            Normal::WEST => {
                if index & 0xf00 == 0 {
                    return self.get_neighbor::<N>().map_or(0, |chunk| chunk.light[index | 0xf00]);
                }
                return self.light[index - 0x100];
            }
            Normal::UP => {
                if index & 0x0f0 == 0x0f0 {
                    return self.get_neighbor::<N>().map_or(0, |chunk| chunk.light[index & 0xf0f]);
                }
                return self.light[index + 0x010];
            }
            Normal::DOWN => {
                if index & 0x0f0 == 0 {
                    return self.get_neighbor::<N>().map_or(0, |chunk| chunk.light[index | 0x0f0]);
                }
                return self.light[index - 0x010];
            }
            _ => unsafe { hint::unreachable_unchecked(); }
        }

    }

    pub fn get_face_pair<const N: Normal>(&self, index: usize) -> (&BlockFace, &BlockFace) {
        return (self.get_face::<N>(index), self.get_opposing_face::<N>(index))
    }

    pub fn has_extra_face<const N: Normal>(&self, index: usize) -> bool {
        return self.get_block(index).otherFaces[N as usize] != 0xffff;
    }
    pub fn has_opposing_extra_face<const N: Normal>(&self, index: usize) -> bool {
        return self.get_opposing_block::<N>(index).otherFaces[N.reverse() as usize] != 0xffff;
    }
    
    pub fn get_extra_face<const N: Normal>(&self, index: usize) -> Option<&(BlockFace, bool)> {
        if self.has_extra_face::<N>(index) {
            return Some(&OTHER_FACES[self.get_block(index).otherFaces[N as usize] as usize]);
        }
        return None;
    }
    pub fn get_opposing_extra_face<const N: Normal>(&self, index: usize) -> Option<&(BlockFace, bool)> {
        if self.has_opposing_extra_face::<N>(index) {
            return Some(&OTHER_FACES[self.get_opposing_block::<N>(index).otherFaces[N.reverse() as usize] as usize]);
        }
        return None;
    }

    pub fn make_terrain(&mut self, noise: &Vec<f32>, chunk_x: usize, chunk_y: usize, chunk_z: usize) { unsafe {
        self.pos = Vector3 {
            x: chunk_x as i32,
            y: chunk_y as i32,
            z: chunk_z as i32
        };
        
        for x in 0..16 { for y in 0..16 { for z in 0..16 {
            let max_world_y = World::MAX_CHUNK_Y * 16;
            let max_world_z = World::MAX_CHUNK_Z * 16;
            let world_x = chunk_x * 16 + x;
            let world_y = chunk_y * 16 + y;
            let world_z = chunk_z * 16 + z;
            let index = {
                world_x * max_world_z * max_world_y +
                world_y * max_world_z +
                world_z
            };
            let (noise_val, block, light) = (
                noise.get_unchecked(index),
                self.blocks.get_unchecked_mut(Chunk::index(x, y, z)),
                self.light.get_unchecked_mut(Chunk::index(x, y, z)),
            );
            *block = match *noise_val {
                // val if val < 0.49 => {
                //     1
                // },
                val if val < 0.50 => {
                    1
                },
                _ => {
                    0
                }
            };
            *light = rand::random::<u8>() & 0xf;
            // *light = (*noise_val * 64.0 - 32.0) as u8;
        } } }
    } }

    pub fn make_terrain_alt(&mut self, noise: &Vec<f32>, chunk_x: usize, chunk_y: usize, chunk_z: usize) {
        self.pos = Vector3 {
            x: chunk_x as i32,
            y: chunk_y as i32,
            z: chunk_z as i32
        };
        self.blocks[Chunk::index(1, 1, 0)] = 1;
        self.blocks[Chunk::index(1, 0, 1)] = 1;
        self.blocks[Chunk::index(0, 1, 1)] = 1;

        self.blocks[Chunk::index(1, 1, 2)] = 1;
        self.blocks[Chunk::index(1, 2, 1)] = 1;
        self.blocks[Chunk::index(2, 1, 1)] = 1;

        self.light[Chunk::index(1, 1, 1)] = 15;


        for i in 0..4096 {
            // self.blocks[i] = rand::random::<u8>() % 2;
            // self.light[i] = rand::random::<u8>() % 16;
        }
    }
    
    // U = X
    // V = Y
    // D = Z
    pub fn mesh_south_north(&mut self, geometry_staging_buffer: &mut StagingBuffer) {
        let mut row_s: [Run; 16] = Default::default();
        let mut run_s: &mut Run = &mut row_s[0];
        let mut active_run_s: bool = false;
        let mut same_row_s: bool = false;

        let mut row_n: [Run; 16] = Default::default();
        let mut run_n: &mut Run = &mut row_n[0];
        let mut active_run_n: bool = false;
        let mut same_row_n: bool = false;
        
        let mut row_id: u16 = 0;

        for d in 0..16_u8 {
            for v in 0..16_u8 {
                for u in 0..16_u8 {
                    let index = Chunk::index(u, v, d);

                    let (face_s, face_n) = self.get_face_pair::<{Normal::SOUTH}>(index);
                    let compare = BlockFace::should_cull(face_s, face_n);
                    'south: {
                        if let Some(face) = self.get_extra_face::<{Normal::SOUTH}>(index) {
                            Run::add_face::<{ Normal::SOUTH }>(geometry_staging_buffer, &face.0, index);
                        }
            
                        if compare.0 {
                            active_run_s = false;
                            break 'south
                        }
                        // /*
                        if active_run_s && same_row_s {
                            if run_s.match_right(&face_s) {
                                run_s.merge_face(geometry_staging_buffer, &face_s);
                                break 'south
                            } else {
                                active_run_s = false;
                            }
                        }
                        // /*
                        if !active_run_s {
                            run_s = &mut row_s[u as usize];
                            if run_s.row + 1 == row_id && run_s.match_top_left(&face_s) {
                                same_row_s = false;
                                active_run_s = true;
                            }
                        }
                        if active_run_s {
                            if run_s.end == u {
                                if run_s.match_top_right(&face_s) {
                                    run_s.pull(geometry_staging_buffer, &face_s, u, v, d);
                                    active_run_s = false;
                                }
                                else {
                                    run_s.pull_partial(geometry_staging_buffer, &face_s, u, v, d);
                                    same_row_s = true;
                                }
                            }
                            else {
                                let next_pos = index + 0x100;
                                let (next_face_s, next_face_n) = self.get_face_pair::<{Normal::SOUTH}>(next_pos);
                                let compare = BlockFace::should_cull(next_face_s, next_face_n);

                                if compare.0 || !Run::match_faces(face_s, next_face_s) {
                                    run_s.pull_partial(geometry_staging_buffer, &face_s, u, v, d);
                                    active_run_s = false;
                                }
                            }
                            break 'south
                        }
                        // */
                        // */
                        run_s = &mut row_s[u as usize];
                        same_row_s = true;
                        active_run_s = true;
                        run_s.begin::<{ Normal::SOUTH }>(geometry_staging_buffer, &face_s, index, u, row_id);
                    }
                    'north: {
                        // break 'north;

                        if let Some(face) = self.get_opposing_extra_face::<{ Normal::SOUTH }>(index) {
                            Run::add_face::<{ Normal::NORTH }>(geometry_staging_buffer, &face.0, index);
                        }

                        if compare.1 {
                            active_run_n = false;
                            break 'north
                        }
                        // /*
                        if active_run_n && same_row_n {
                            if run_n.match_right(&face_n) {
                                run_n.merge_face(geometry_staging_buffer, &face_n);
                                break 'north
                            } else {
                                active_run_n = false;
                            }
                        }
                        // /*
                        if !active_run_n {
                            run_n = &mut row_n[u as usize];
                            if run_n.row + 1 == row_id && run_n.match_top_left(&face_n) {
                                same_row_n = false;
                                active_run_n = true;
                            }
                        }
                        if active_run_n {
                            if run_n.end == u {
                                if run_n.match_top_right(&face_n) {
                                    run_n.pull(geometry_staging_buffer, &face_n, u, v, d);
                                    active_run_n = false;
                                }
                                else {
                                    run_n.pull_partial(geometry_staging_buffer, &face_n, u, v, d);
                                    same_row_n = true;
                                }
                            }
                            else {
                                let next_pos = index + 0x100;
                                let (next_face_s, next_face_n) = self.get_face_pair::<{Normal::SOUTH}>(next_pos);
                                let compare = BlockFace::should_cull(next_face_s, next_face_n);

                                if compare.1 || !Run::match_faces(face_n, next_face_n) {
                                    run_n.pull_partial(geometry_staging_buffer, &face_n, u, v, d);
                                    active_run_n = false;
                                }
                            }
                            break 'north
                        }
                        // */
                        // */
                        run_n = &mut row_n[u as usize];
                        active_run_n = true;
                        same_row_n = true;
                        run_n.begin::<{ Normal::NORTH }>(geometry_staging_buffer, &face_n, index, u, row_id);
                    }
                }
                (active_run_s, active_run_n) = (false, false);
                row_id += 1;
            }
            row_id += 16;
        }
    }
    
    // U = Z
    // V = Y
    // D = X
    pub fn mesh_west_east(&mut self, geometry_staging_buffer: &mut StagingBuffer) {
        let mut row_w: [Run; 16] = Default::default();
        let mut run_w: &mut Run = &mut row_w[0];
        let mut active_run_w: bool = false;
        let mut same_row_w: bool = false;

        let mut row_e: [Run; 16] = Default::default();
        let mut run_e: &mut Run = &mut row_e[0];
        let mut active_run_e: bool = false;
        let mut same_row_e: bool = false;
        
        let mut row_id: u16 = 0;
        
        for d in 0..16_u8 {
            for v in 0..16_u8 {
                for u in 0..16_u8 {
                    let index = Chunk::index(d, v, u);

                    let (face_w, face_e) = self.get_face_pair::<{Normal::WEST}>(index);
                    let compare = BlockFace::should_cull(face_w, face_e);
                    'west: {
                        if let Some(face) = self.get_extra_face::<{Normal::WEST}>(index) {
                            Run::add_face::<{ Normal::WEST }>(geometry_staging_buffer, &face.0, index);
                        }
            
                        if compare.0 {
                            active_run_w = false;
                            break 'west
                        }

                        // /*
                        if active_run_w && same_row_w {
                            if run_w.match_right(&face_w) {
                                run_w.merge_face(geometry_staging_buffer, &face_w);
                                break 'west
                            } else {
                                active_run_w = false;
                            }
                        }
                        // /*
                        if !active_run_w {
                            run_w = &mut row_w[u as usize];
                            if run_w.row + 1 == row_id && run_w.match_top_left(&face_w) {
                                same_row_w = false;
                                active_run_w = true;
                            }
                        }
                        if active_run_w {
                            if run_w.end == u {
                                if run_w.match_top_right(&face_w) {
                                    run_w.pull(geometry_staging_buffer, &face_w, u, v, d);
                                    active_run_w = false;
                                }
                                else {
                                    run_w.pull_partial(geometry_staging_buffer, &face_w, u, v, d);
                                    same_row_w = true;
                                }
                            }
                            else {
                                let next_pos = index + 0x001;
                                let (next_face_w, next_face_e) = self.get_face_pair::<{Normal::WEST}>(next_pos);
                                let compare = BlockFace::should_cull(next_face_w, next_face_e);

                                if compare.0 || !Run::match_faces(face_w, next_face_w) {
                                    run_w.pull_partial(geometry_staging_buffer, &face_w, u, v, d);
                                    active_run_w = false;
                                }
                            }
                            break 'west
                        }
                        // */
                        // */
                        run_w = &mut row_w[u as usize];
                        same_row_w = true;
                        active_run_w = true;
                        run_w.begin::<{ Normal::WEST }>(geometry_staging_buffer, &face_w, index, u, row_id);
                    }
                    'east: {
                        // break 'east;

                        if let Some(face) = self.get_opposing_extra_face::<{ Normal::WEST }>(index) {
                            Run::add_face::<{ Normal::EAST }>(geometry_staging_buffer, &face.0, index);
                        }
                        if compare.1 {
                            active_run_e = false;
                            break 'east
                        }
                        // /*
                        if active_run_e && same_row_e {
                            if run_e.match_right(&face_e) {
                                run_e.merge_face(geometry_staging_buffer, &face_e);
                                break 'east
                            } else {
                                active_run_e = false;
                            }
                        }
                        // /* 
                        if !active_run_e {
                            run_e = &mut row_e[u as usize];
                            if run_e.row + 1 == row_id && run_e.match_top_left(&face_e) {
                                same_row_e = false;
                                active_run_e = true;
                            }
                        }

                        if active_run_e {
                            if run_e.end == u {
                                if run_e.match_top_right(&face_e) {
                                    run_e.pull(geometry_staging_buffer, &face_e, u, v, d);
                                    active_run_e = false;
                                }
                                else {
                                    run_e.pull_partial(geometry_staging_buffer, &face_e, u, v, d);
                                    same_row_e = true;
                                }
                            }
                            else {
                                let next_pos = index + 0x001;
                                let (next_face_w, next_face_e) = self.get_face_pair::<{Normal::WEST}>(next_pos);
                                let compare = BlockFace::should_cull(next_face_w, next_face_e);

                                if compare.1 || !Run::match_faces(face_e, next_face_e) {
                                    run_e.pull_partial(geometry_staging_buffer, &face_e, u, v, d);
                                    active_run_e = false;
                                }
                            }
                            break 'east
                        }
                        // */
                        // */
                        run_e = &mut row_e[u as usize];
                        active_run_e = true;
                        same_row_e = true;
                        run_e.begin::<{ Normal::EAST }>(geometry_staging_buffer, &face_e, index, u, row_id);
                    }
                }
                (active_run_w, active_run_e) = (false, false); row_id += 1;
            }
            row_id += 16;
        }
    }
    
    // U = Z
    // V = X
    // D = Y
    pub fn mesh_down_up(&mut self, geometry_staging_buffer: &mut StagingBuffer) {
        let mut row_d: [Run; 16] = Default::default();
        let mut run_d: &mut Run = &mut row_d[0];
        let mut active_run_d: bool = false;
        let mut same_row_d: bool = false;

        let mut row_u: [Run; 16] = Default::default();
        let mut run_u: &mut Run = &mut row_u[0];
        let mut active_run_u: bool = false;
        let mut same_row_u: bool = false;
        
        let mut row_id: u16 = 0;
        
        for d in 0..16_u8 {
            for v in 0..16_u8 {
                for u in 0..16_u8 {
                    let index = Chunk::index(v, d, u);

                    let (face_d, face_u) = self.get_face_pair::<{Normal::DOWN}>(index);
                    let compare = BlockFace::should_cull(face_d, face_u);
                    'down: {
                        if let Some(face) = self.get_extra_face::<{Normal::DOWN}>(index) {
                            Run::add_face::<{ Normal::DOWN }>(geometry_staging_buffer, &face.0, index);
                        }
            
                        if compare.0 {
                            active_run_d = false;
                            break 'down
                        }

                        // /*
                        if active_run_d && same_row_d {
                            if run_d.match_right(&face_d) {
                                run_d.merge_face(geometry_staging_buffer, &face_d);
                                break 'down
                            } else {
                                active_run_d = false;
                            }
                        }
                        // /*
                        if !active_run_d {
                            run_d = &mut row_d[u as usize];
                            if run_d.row + 1 == row_id && run_d.match_top_left(&face_d) {
                                same_row_d = false;
                                active_run_d = true;
                            }
                        }
                        if active_run_d {
                            if run_d.end == u {
                                if run_d.match_top_right(&face_d) {
                                    run_d.pull(geometry_staging_buffer, &face_d, u, v, d);
                                    active_run_d = false;
                                }
                                else {
                                    run_d.pull_partial(geometry_staging_buffer, &face_d, u, v, d);
                                    same_row_d = true;
                                }
                            }
                            else {
                                let next_pos = index + 0x001;
                                let (next_face_d, next_face_u) = self.get_face_pair::<{Normal::DOWN}>(next_pos);
                                let compare = BlockFace::should_cull(next_face_d, next_face_u);

                                if compare.0 || !Run::match_faces(face_d, next_face_d) {
                                    run_d.pull_partial(geometry_staging_buffer, &face_d, u, v, d);
                                    active_run_d = false;
                                }
                            }
                            break 'down
                        }
                        // */
                        // */
                        run_d = &mut row_d[u as usize];
                        same_row_d = true;
                        active_run_d = true;
                        run_d.begin::<{ Normal::DOWN }>(geometry_staging_buffer, &face_d, index, u, row_id);
                    }
                    'up: {
                        // break 'up;

                        if let Some(face) = self.get_opposing_extra_face::<{ Normal::DOWN }>(index) {
                            Run::add_face::<{ Normal::UP }>(geometry_staging_buffer, &face.0, index);
                        }

                        if compare.1 {
                            active_run_u = false;
                            break 'up
                        }
                        // /*
                        if active_run_u && same_row_u {
                            if run_u.match_right(&face_u) {
                                run_u.merge_face(geometry_staging_buffer, &face_u);
                                break 'up
                            } else {
                                active_run_u = false;
                            }
                        }
                        // /* 
                        if !active_run_u {
                            run_u = &mut row_u[u as usize];
                            if run_u.row + 1 == row_id && run_u.match_top_left(&face_u) {
                                same_row_u = false;
                                active_run_u = true;
                            }
                        }

                        if active_run_u {
                            if run_u.end == u {
                                if run_u.match_top_right(&face_u) {
                                    run_u.pull(geometry_staging_buffer, &face_u, u, v, d);
                                    active_run_u = false;
                                }
                                else {
                                    run_u.pull_partial(geometry_staging_buffer, &face_u, u, v, d);
                                    same_row_u = true;
                                }
                            }
                            else {
                                let next_pos = index + 0x001;
                                let (next_face_d, next_face_u) = self.get_face_pair::<{Normal::DOWN}>(next_pos);
                                let compare = BlockFace::should_cull(next_face_d, next_face_u);

                                if compare.1 || !Run::match_faces(face_u, next_face_u) {
                                    run_u.pull_partial(geometry_staging_buffer, &face_u, u, v, d);
                                    active_run_u = false;
                                }
                            }
                            break 'up
                        }
                        // */
                        // */
                        run_u = &mut row_u[u as usize];
                        active_run_u = true;
                        same_row_u = true;
                        run_u.begin::<{ Normal::UP }>(geometry_staging_buffer, &face_u, index, u, row_id);
                    }
                }
                (active_run_d, active_run_u) = (false, false); row_id += 1;
            }
            row_id += 16;
        }
    }
    
    pub fn mesh_south_north_no_merge(&mut self, geometry_staging_buffer: &mut StagingBuffer) {
        for z in 0..16_u8 {
            for y in 0..16_u8 {
                for x in 0..16_u8 {
                    let index = Chunk::index(x, y, z);
                    
                    let face_s = self.get_face::<{Normal::SOUTH}>(index);
                    let face_n = self.get_opposing_face::<{Normal::SOUTH}>(index);

                    let compare = face_s.as_u32() + 0x10101010 - face_n.as_u32();
                    if compare == 0x10101010 { continue; }
                    
                    let offset = Chunk::INDICES_ZYX[index] as u64;
                    
                    if compare < 0x10101010 { geometry_staging_buffer.put_u64(face_s.as_u64() + offset); }
                    if compare > 0x10101010 { geometry_staging_buffer.put_u64(face_n.as_u64() + offset); }
                    
                    if let Some(face) = self.get_extra_face::<{Normal::SOUTH}>(index) {
                        Run::add_face::<{ Normal::SOUTH }>(geometry_staging_buffer, &face.0, index);
                    }
                    if let Some(face) = self.get_opposing_extra_face::<{ Normal::SOUTH }>(index) {
                        Run::add_face::<{ Normal::NORTH }>(geometry_staging_buffer, &face.0, index);
                    }
                }
            }
        }
    }
    pub fn mesh_west_east_no_merge(&mut self, geometry_staging_buffer: &mut StagingBuffer) {
        for x in 0..16_u8 {
            for y in 0..16_u8 {
                for z in 0..16_u8 {
                    let index = Chunk::index(x, y, z);

                    let face_w = self.get_face::<{Normal::WEST}>(index);
                    let face_e = self.get_opposing_face::<{Normal::WEST}>(index);

                    let compare = face_w.as_u32() + 0x10101010 - face_e.as_u32();
                    if compare == 0x10101010 { continue; }
                    
                    let offset = Chunk::INDICES_XYZ[index] as u64;
                    
                    if compare < 0x10101010 { geometry_staging_buffer.put_u64(face_w.as_u64() + offset); }
                    if compare > 0x10101010 { geometry_staging_buffer.put_u64(face_e.as_u64() + offset); }

                    if let Some(face) = self.get_extra_face::<{Normal::WEST}>(index) {
                        Run::add_face::<{ Normal::WEST }>(geometry_staging_buffer, &face.0, index);
                    }

                    if let Some(face) = self.get_opposing_extra_face::<{ Normal::WEST }>(index) {
                        Run::add_face::<{ Normal::EAST }>(geometry_staging_buffer, &face.0, index);
                    }
                }
            }
        }
    }
    pub fn mesh_down_up_no_merge(&mut self, geometry_staging_buffer: &mut StagingBuffer) {
        for x in 0..16_u8 {
            for y in 0..16_u8 {
                for z in 0..16_u8 {
                    let index = Chunk::index(x, y, z);

                    let face_d = self.get_face::<{Normal::DOWN}>(index);
                    let face_u = self.get_opposing_face::<{Normal::DOWN}>(index);

                    let compare = face_d.as_u32() + 0x10101010 - face_u.as_u32();
                    if compare == 0x10101010 { continue; }

                    let offset = Chunk::INDICES_YXZ[index] as u64;

                    if compare < 0x10101010 { geometry_staging_buffer.put_u64(face_d.as_u64() + offset); }
                    if compare > 0x10101010 { geometry_staging_buffer.put_u64(face_u.as_u64() + offset); }

                    if let Some(face) = self.get_extra_face::<{Normal::DOWN}>(index) {
                        Run::add_face::<{ Normal::DOWN }>(geometry_staging_buffer, &face.0, index);
                    }
                
                    if let Some(face) = self.get_opposing_extra_face::<{ Normal::DOWN }>(index) {
                        Run::add_face::<{ Normal::UP }>(geometry_staging_buffer, &face.0, index);
                    }
                }
            }
        }
    }

    pub fn generate_geometry_buffer(&mut self, geometry_staging_buffer: &mut StagingBuffer, geometry_buffer_allocator: &mut BufferPoolAllocator<524288, 1024>) { unsafe {
        self.mesh_south_north(geometry_staging_buffer);
        self.mesh_west_east(geometry_staging_buffer);
        self.mesh_down_up(geometry_staging_buffer);

        // self.mesh_south_north_no_merge(geometry_staging_buffer);
        // self.mesh_west_east_no_merge(geometry_staging_buffer);
        // self.mesh_down_up_no_merge(geometry_staging_buffer);

        geometry_staging_buffer.format_quads();

        self.quad_count = geometry_staging_buffer.index as u32 / 8;
        
        self.geometry_page = geometry_buffer_allocator.allocate(geometry_staging_buffer.index + 4 * mem::size_of::<u32>());
        if let Some(page) = &self.geometry_page {
            geometry_buffer_allocator.upload_offset(page, &geometry_staging_buffer.buffer.0.as_slice(), geometry_staging_buffer.index, 4 * mem::size_of::<u32>());
            geometry_buffer_allocator.upload(page, &[self.pos.x, self.pos.y, self.pos.z], 3 * mem::size_of::<u32>());
        }
    } }

    pub fn generate_light_buffer(&mut self, geometry_staging_buffer: &mut StagingBuffer, geometry_buffer_allocator: &mut BufferPoolAllocator<524288, 1024>, light_staging_buffer: &mut StagingBuffer, light_buffer_allocator: &mut BufferPoolAllocator<524288, 1024>) { unsafe {
        const BYTES_PER_QUAD: usize = 8;

        // bytes reserved to index a light chunk for each quad
        // need 1 u32 per quad
        let reserved_bytes = geometry_staging_buffer.index / BYTES_PER_QUAD * mem::size_of::<u32>();
        light_staging_buffer.index = reserved_bytes;

        for (i, quad) in geometry_staging_buffer.iter().map(|quad| mem::transmute::<&[u8; 8], &GpuQuad>(quad)).enumerate() {
            // insert the index offset of the light chunk
            light_staging_buffer.set_u32(i * mem::size_of::<u32>(), (light_staging_buffer.index / mem::size_of::<u32>()) as u32);
            
            match quad.nor {
                Normal::SOUTH => {
                    let start_x = quad.ure / 16;
                    let end_x = (quad.ure + quad.wid) / 16;
                    
                    let start_y = quad.ven / 16;
                    let end_y = (quad.ven + quad.hei) / 16;
                    
                    let z = (quad.dep + 1) / 16;
                    
                    for x in start_x..=end_x {
                        for y in start_y..=end_y {
                            light_staging_buffer.put_u32(self.get_light::<{ Normal::SOUTH }>(Chunk::index(x, y, z)) as u32);
                        }
                    }
                }
                Normal::NORTH => {
                    let start_x = quad.ure / 16;
                    let end_x = (quad.ure + quad.wid) / 16;
    
                    let start_y = quad.ven / 16;
                    let end_y = (quad.ven + quad.hei) / 16;

                    // if this from the neighboring chunk, z wraps to 15, which is the only way for it to be 15
                    let z = (quad.dep - 16) / 16;
                    
                    for x in start_x..=end_x {
                        for y in start_y..=end_y {
                            if z == 15 {
                                if let Some(south) = self.get_neighbor::<{ Normal::SOUTH }>() {
                                    light_staging_buffer.put_u32(south.get_light::<{ Normal::NORTH }>(Chunk::index(x, y, z)) as u32);
                                }
                                else {
                                    light_staging_buffer.put_u32(0);
                                }
                            }
                            light_staging_buffer.put_u32(self.get_light::<{ Normal::NORTH }>(Chunk::index(x, y, z)) as u32);
                        }
                    }
                }
                Normal::WEST => {
                    let x = quad.dep / 16;
    
                    let start_y = quad.ven / 16;
                    let end_y = (quad.ven + quad.hei) / 16;
    
                    let start_z = quad.ure / 16;
                    let end_z = (quad.ure + quad.wid) / 16;
                    
                    for y in start_y..=end_y {
                        for z in start_z..=end_z {
                            light_staging_buffer.put_u32(self.get_light::<{ Normal::WEST }>(Chunk::index(x, y, z)) as u32);
                        }
                    }
                }
                Normal::EAST => {
                    // if this from the neighboring chunk, x wraps to 15, which is the only way for it to be 15
                    let x = (quad.dep - 16) / 16;
    
                    let start_y = quad.ven / 16;
                    let end_y = (quad.ven + quad.hei) / 16;
    
                    let start_z = quad.ure / 16;
                    let end_z = (quad.ure + quad.wid) / 16;
                    
                    for y in start_y..=end_y {
                        for z in start_z..=end_z {
                            if x == 15 {
                                if let Some(west) = self.get_neighbor::<{ Normal::WEST }>() {
                                    light_staging_buffer.put_u32(west.get_light::<{ Normal::EAST }>(Chunk::index(x, y, z)) as u32);
                                }
                                else {
                                    light_staging_buffer.put_u32(0);
                                }
                            }
                            light_staging_buffer.put_u32(self.get_light::<{ Normal::EAST }>(Chunk::index(x, y, z)) as u32);
                        }
                    }
                }
                Normal::DOWN => {
                    let start_x = quad.ven / 16;
                    let end_x = (quad.ven + quad.hei) / 16;
    
                    let y = quad.dep / 16;
    
                    let start_z = quad.ure / 16;
                    let end_z = (quad.ure + quad.wid) / 16;
                    
                    for x in start_x..=end_x {
                        for z in start_z..=end_z {
                            light_staging_buffer.put_u32(self.get_light::<{ Normal::DOWN }>(Chunk::index(x, y, z)) as u32);
                        }
                    }
                }
                Normal::UP => {
                    let start_x = quad.ven / 16;
                    let end_x = (quad.ven + quad.hei) / 16;

                    // if this from the neighboring chunk, y wraps to 15, which is the only way for it to be 15
                    let y = (quad.dep - 16) / 16;
    
                    let start_z = quad.ure / 16;
                    let end_z = (quad.ure + quad.wid) / 16;
                    
                    for x in start_x..=end_x {
                        for z in start_z..=end_z {
                            if y == 15 {
                                if let Some(down) = self.get_neighbor::<{ Normal::DOWN }>() {
                                    light_staging_buffer.put_u32(down.get_light::<{ Normal::UP }>(Chunk::index(x, y, z)) as u32);
                                }
                                else {
                                    light_staging_buffer.put_u32(0);
                                }
                            }
                            light_staging_buffer.put_u32(self.get_light::<{ Normal::UP }>(Chunk::index(x, y, z)) as u32);
                        }
                    }
                }
                _ => {
                    unreachable_unchecked();
                }
            }
        }
        
        self.light_page = light_buffer_allocator.allocate(light_staging_buffer.index);
        if self.light_page.is_none() { return; }

        if let Some(light_page) = &self.light_page {
            light_buffer_allocator.upload(light_page, light_staging_buffer.buffer.0.as_slice(), light_staging_buffer.index);
            if let Some(page) = &self.geometry_page {
                geometry_buffer_allocator.upload_offset(page, &[(light_page.start * light_page.block_size() / mem::size_of::<u32>()) as u32], mem::size_of::<u32>(), 3 * mem::size_of::<u32>());
            }
        }

    } }

    pub fn get_bounding_box(&self, camera: &Camera) -> BoundingBox<f32> {
        return BoundingBox {
            min: Vector3 {
                x: (self.pos.x * 256) as f32 - camera.frustum_pos.x,
                y: (self.pos.y * 256) as f32 - camera.frustum_pos.y,
                z: (self.pos.z * 256) as f32 - camera.frustum_pos.z,
            },
            max: Vector3 {
                x: (self.pos.x * 256) as f32 - camera.frustum_pos.x + 256.0,
                y: (self.pos.y * 256) as f32 - camera.frustum_pos.y + 256.0,
                z: (self.pos.z * 256) as f32 - camera.frustum_pos.z + 256.0,
            }
        };
    }

    pub fn get_bounding_sphere(&self, camera: &Camera) -> Sphere<f32> {
        return Sphere {
            center: Vector3 {
                x: (self.pos.x * 256) as f32 - camera.frustum_pos.x + 128.0,
                y: (self.pos.y * 256) as f32 - camera.frustum_pos.y + 128.0,
                z: (self.pos.z * 256) as f32 - camera.frustum_pos.z + 128.0,
            },
            radius: 221.702503369
        };
    }

    pub fn index<T: TryInto<usize>>(x: T, y: T, z: T) -> usize { unsafe {
        return (x.try_into().unwrap_unchecked() << 8) |
                (y.try_into().unwrap_unchecked() << 4) |
                (z.try_into().unwrap_unchecked() << 0);
    } }

    pub const INDICES_ZYX: [u32; 4096] = {
        let mut arr = [0; 4096];
        let mut i = 0;
        let mut x = 0; while x < 16 {
            let mut y = 0; while y < 16 {
                let mut z = 0; while z < 16 {
                    
                    arr[i] = (x << 4) | (y << 12) | (z << 20);
                    i += 1;

                z += 1; }
            y += 1; }
        x += 1; }
        arr
    };

    pub const INDICES_XYZ: [u32; 4096] = {
        let mut arr = [0; 4096];
        let mut i = 0;
        let mut z = 0; while z < 16 {
            let mut y = 0; while y < 16 {
                    let mut x = 0; while x < 16 {
                    
                    arr[i] = (x << 4) | (y << 12) | (z << 20);
                    i += 1;

                x += 1; }
            y += 1; }
        z += 1; }
        arr
    };

    pub const INDICES_YXZ: [u32; 4096] = {
        let mut arr = [0; 4096];
        let mut i = 0;
        let mut y = 0; while y < 16 {
            let mut z = 0; while z < 16 {
                let mut x = 0; while x < 16 {
                    
                    arr[i] = (x << 4) | (y << 12) | (z << 20);
                    i += 1;

                    x += 1; }
                z += 1; }
            y += 1; }
        arr
    };
}
impl Drop for Chunk {
    fn drop(&mut self) {
        // println!("Dropped chunk {} {} {}", self.pos.x, self.pos.y, self.pos.z);
        // if let Some(buffer) = &self.buffer && buffer.valid() {
            // panic!()
        // }
    }
}

#[derive(Clone, Debug)]
#[allow(dead_code)]
#[repr(C)]
pub struct Run {
    lef: u8,
    bot: u8,
    rig: u8,
    top: u8,
    dep: u8,
    
    beg: u8,
    end: u8,
    
    tex: u16,
    
    idx: u16,
    
    row: u16,
    pad: u16
}

impl Run {
    /**
     * Matches the top and bottom right corners of the run with the top and bottom left corners of the face
     * Used to immediately merge a face
     */
    pub fn match_right(&self, face: &BlockFace) -> bool { unsafe {
        return (
            self.tex == face.tex &&
            self.rig == 0 &&
            face.lef == 0 &&
            self.bot == face.bot &&
            self.top == face.top
        );
    } }
    /**
     * Matches the top left corner of the run with the bottom left corner of the face
     * Used to begin a merge
     */
    pub fn match_top_left(&self, face: &BlockFace) -> bool { unsafe {
        return (
            self.tex == face.tex &&
            self.top == 0 &&
            face.bot == 0 &&
            self.lef == face.lef
        );
    } }
    /**
     * Matches the top right corner of the run with the bottom right corner of the face
     * Used to finalize a merge
     */
    pub fn match_top_right(&self, face: &BlockFace) -> bool { unsafe {
        return (
            face.bot == 0 &&
            self.rig == face.rig
        );
    } }
    /**
     * Merges a face into the run horizontally
     * Extends the run's end position and updates the end x
     * End y is already guaranteed to match
     */
    pub fn merge_face(&mut self, buffer: &mut StagingBuffer, face: &BlockFace) { unsafe {
        buffer[self.idx + 3] += 0x10;
        buffer[self.idx + 2] &= 0xf0;
        buffer[self.idx + 2] |= face.rig;
        self.end += 1;
        self.rig = face.rig;
    } }
    /**
     * Pulls the run up after an incomplete merge
     * min_x, min_y, min_z, and texture are already guaranteed to match
     */
    pub fn pull_partial(&mut self, buffer: &mut StagingBuffer, face: &BlockFace, u: u8, v: u8, d: u8) {
        let ind = buffer.index as u16;
        buffer.put_u64(buffer.get_u64(self.idx));
        
        buffer[ind + 1] = v << 4;
        buffer[ind + 2] = (d << 4) | face.rig;
        buffer[ind + 3] = ((u - self.beg) << 4) | face.top;
        buffer[ind + 4] &= 0x0f;
        self.idx = ind;
        self.end = u;
        self.row += 1;
    }
    /**
     * Pulls the run up after a complete merge
     * Only possible change is top
     */
    pub fn pull(&mut self, buffer: &mut StagingBuffer, face: &BlockFace, u: u8, v: u8, d: u8) {
        buffer[self.idx as usize + 4] += 0x10;
        buffer[self.idx as usize + 3] &= 0xf0;
        buffer[self.idx as usize + 3] |= face.top;
        self.top = face.top;
        self.row += 1;
    }
    /**
     * Begins a new run
     */
    pub fn begin<const N: Normal>(&mut self, buffer: &mut StagingBuffer, face: &BlockFace, pos: usize, u: u8, row: u16) {
        self.idx = buffer.index as u16;
        let offset: u32;
        match N {
            Normal::SOUTH | Normal::NORTH => {
                offset = Chunk::INDICES_ZYX[pos];
            }
            Normal::WEST | Normal::EAST => {
                offset = Chunk::INDICES_XYZ[pos];
            }
            Normal::DOWN | Normal::UP => {
                offset = Chunk::INDICES_YXZ[pos];
            }
            _ => {
                unsafe { unreachable_unchecked(); }
            }
        }
        buffer.put_u64(face.as_u64() + offset as u64);

        self.lef = face.lef;
        self.bot = face.bot;

        self.rig = face.rig;
        self.top = face.top;

        self.beg = u;
        self.end = u;
        self.tex = face.tex;


        self.row = row;
    }
    
    /**
     * Directly adds a face
     */
    pub fn add_face<const N: Normal>(buffer: &mut StagingBuffer, face: &BlockFace, pos: usize) {
        let offset: u32;
        match N {
            Normal::SOUTH | Normal::NORTH => {
                offset = Chunk::INDICES_ZYX[pos];
            }
            Normal::WEST | Normal::EAST => {
                offset = Chunk::INDICES_XYZ[pos];
            }
            Normal::DOWN | Normal::UP => {
                offset = Chunk::INDICES_YXZ[pos];
            }
            _ => {
                unsafe { unreachable_unchecked(); }
            }
        }
        buffer.put_u64(face.as_u64() + offset as u64);
    }
    /**
     * Matches the top and bottom right corners of the first face with the top and bottom left corners of the next face
     */
    pub fn match_faces(face: &BlockFace, next: &BlockFace) -> bool {
        return (
            face.tex == next.tex &&
            face.rig == next.lef &&
            face.bot == next.bot &&
            face.top == next.top
        );
    }
    
    pub fn as_u32(&self) -> &u32 { unsafe {
        return &*(&raw const self as *mut u32);
    } }

    pub fn new() -> Run {
        Run {
            lef: 0,
            bot: 0,
            rig: 0,
            top: 0,
            beg: 0,
            end: 0,
            dep: 0,
            tex: 0,
            row: u16::MAX - 1,
            idx: 0,
            pad: 0
        }
    }
}

impl Default for Run {
    fn default() -> Self {
        return Run::new();
    }
}