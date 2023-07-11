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
use std::marker::PhantomData;
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
use crate::OTHER_FACES;
use crate::block::blockface::GpuQuad;
use crate::util::gl_helper::Page;
use crate::util::gl_helper::BufferPoolAllocator;
use crate::{block::{blockstate::BlockState, blockface::{Normal, BlockFace}}, util::{gl_helper::{Buffer, log_if_error, log_error}, byte_buffer::StagingBuffer}, BLOCKS};

use super::world::{World, Lcg};
#[derive(Debug)]
pub struct Chunk {
    // Block IDs
    pub blocks: [u8; 4096],
    // Light levels 0-15
    pub light: [u8; 4096],

    // Chunk position
    pub pos: Vec3i,
    // Padding for page id
    pub dummy: i32,
    // Adjacent chunka
    pub neighbors: Neighbors,
    // Number of rectangular block faces in a chunk
    pub face_count: u32,

    pub geometry_page: Option<Page>,
    pub light_page: Option<Page>
}
#[derive(Debug)]
pub struct Neighbors {
    pub south: Option<*mut Chunk>,
    pub west: Option<*mut Chunk>,
    pub down: Option<*mut Chunk>,
    pub north: Option<*mut Chunk>,
    pub east: Option<*mut Chunk>,
    pub up: Option<*mut Chunk>,
}
impl Chunk {
    pub fn get_block(&self, index: usize) -> &'static BlockState { unsafe {
        return &BLOCKS[*self.blocks.get_unchecked(index) as usize];
    } }
    pub fn get_opposing_block<'a, const NORMAL: Normal>(&'a self, index: usize) -> &'a BlockState { unsafe {
        match NORMAL {
            Normal::SOUTH => {
                if index & 0x00f == 0 {
                    match self.neighbors.south {
                        Some(chunk) => return &(*chunk).get_block(index | 0x00f),
                        None => return &BLOCKS[0],
                    }
                }
                return &self.get_block(index - 0x001);
            }
            Normal::WEST => {
                if index & 0xf00 == 0 {
                    match self.neighbors.west {
                        Some(chunk) => return &(*chunk).get_block(index | 0xf00),
                        None => return &BLOCKS[0],
                    }
                }
                return &self.get_block(index - 0x100);
            }
            Normal::DOWN => {
                if index & 0x0f0 == 0 {
                    match self.neighbors.down {
                        Some(chunk) => return &(*chunk).get_block(index | 0x0f0),
                        None => return &BLOCKS[0],
                    }
                }
                return &self.get_block(index - 0x010);
            }
            _ => unsafe { std::hint::unreachable_unchecked(); }
        }

    } }
    
    pub fn get_face<const NORMAL: Normal>(&self, index: usize) -> &BlockFace {
        return &self.get_block(index).model.get_face(NORMAL);
    }
    pub fn get_opposing_face<'a, const NORMAL: Normal>(&'a self, index: usize) -> &BlockFace {
        return &self.get_opposing_block::<NORMAL>(index).model.get_face(NORMAL.reverse());
    }
    
    pub fn get_face_pair<'a, const NORMAL: Normal>(&'a self, index: usize) -> (&BlockFace, &BlockFace) {
        return (self.get_face::<NORMAL>(index), self.get_opposing_face::<NORMAL>(index))
    }

    pub fn has_other_face<const NORMAL: Normal>(&self, index: usize) -> bool {
        return self.get_block(index).otherFaces[NORMAL.0 as usize] != 0xffff;
    }
    pub fn has_opposing_other_face<const NORMAL: Normal>(&self, index: usize) -> bool {
        return self.get_opposing_block::<NORMAL>(index).otherFaces[NORMAL.reverse().0 as usize] != 0xffff;
    }
    
    pub fn get_other_face<const NORMAL: Normal>(&self, index: usize) -> &(BlockFace, bool) {
        return &OTHER_FACES[self.get_block(index).otherFaces[NORMAL.0 as usize] as usize];
    }
    pub fn get_opposing_other_face<const NORMAL: Normal>(&self, index: usize) -> &(BlockFace, bool) {
        return &OTHER_FACES[self.get_opposing_block::<NORMAL>(index).otherFaces[NORMAL.reverse().0 as usize] as usize];
    }

    pub fn make_terrain(&mut self, noise: &Vec<f32>, chunk_x: usize, chunk_y: usize, chunk_z: usize) { unsafe {
        self.pos = Vec3i {
            x: chunk_x as i32,
            y: chunk_y as i32,
            z: chunk_z as i32
        };
        
        for x in 0..16 {
            for y in 0..16 {
                for z in 0..16 {
                    let pos = {
                        (chunk_x << 22) |
                        (x << 18) |
                        (chunk_y << 13) |
                        (y << 9 ) |
                        (chunk_z << 4) |
                        (z << 0 )
                    };
                    let (noise_val, block, light) = (
                        noise.get_unchecked(pos),
                        self.blocks.get_unchecked_mut(Chunk::pos(x, y, z)),
                        self.light.get_unchecked_mut(Chunk::pos(x, y, z)),
                    );
                    *block = match *noise_val {
                        val if val < 0.5 => {
                            1
                        },
                        _ => {
                            0
                        }
                    };
                    *light = (*noise_val * 16.0) as u8;
                }
            }
        }
    } }

    pub fn make_terrain_alt(&mut self, random: &mut Lcg) {
        for i in 0..4096 {
            self.blocks[i] = (random.next() % 2) as u8;
        }
    }
    
    // U = X
    // V = Y
    // D = Z
    pub fn mesh_south_north(&mut self, staging_buffer: &mut StagingBuffer) {
        let mut row_s: [Run; 16] = Default::default();
        let mut run_s: &mut Run = &mut row_s[0];
        let mut active_run_s: bool = false;
        let mut same_row_s: bool = false;

        let mut row_n: [Run; 16] = Default::default();
        let mut run_n: &mut Run = &mut row_n[0];
        let mut active_run_n: bool = false;
        let mut same_row_n: bool = false;
        
        let mut row_id: u16 = 0;

        for d in 0..16_u8 { for v in 0..16_u8 { for u in 0..16_u8 {
            let pos = Chunk::pos(u, v, d);

            let (face_s, face_n) = self.get_face_pair::<{Normal::SOUTH}>(pos);
            let compare = BlockFace::should_cull(face_s, face_n);
            'south: {
                if self.has_other_face::<{ Normal::SOUTH }>(pos) {
                    Run::add_face::<{ Normal::SOUTH }>(staging_buffer, &self.get_other_face::<{Normal::SOUTH}>(pos).0, pos);
                }
    
                if compare.0 {
                    active_run_s = false;
                    break 'south
                }
                // /*
                if active_run_s && same_row_s {
                    if run_s.match_right(&face_s) {
                        run_s.merge_face(staging_buffer, &face_s);
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
                            run_s.pull(staging_buffer, &face_s, u, v, d);
                            active_run_s = false;
                        }
                        else {
                            run_s.pull_partial(staging_buffer, &face_s, u, v, d);
                            same_row_s = true;
                        }
                    }
                    else {
                        let next_pos = pos + 0x100;
                        let (next_face_s, next_face_n) = self.get_face_pair::<{Normal::SOUTH}>(next_pos);
                        let compare = BlockFace::should_cull(next_face_s, next_face_n);

                        if compare.0 || !Run::match_faces(face_s, next_face_s) {
                            run_s.pull_partial(staging_buffer, &face_s, u, v, d);
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
                run_s.begin::<{ Normal::SOUTH }>(staging_buffer, &face_s, pos, u, row_id);
            }
            'north: {
                // break 'north;

                if self.has_opposing_other_face::<{ Normal::SOUTH }>(pos) {
                    Run::add_face::<{ Normal::SOUTH }>(staging_buffer, &self.get_opposing_other_face::<{Normal::SOUTH}>(pos).0, pos);
                }

                if compare.1 {
                    active_run_n = false;
                    break 'north
                }
                // /*
                if active_run_n && same_row_n {
                    if run_n.match_right(&face_n) {
                        run_n.merge_face(staging_buffer, &face_n);
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
                            run_n.pull(staging_buffer, &face_n, u, v, d);
                            active_run_n = false;
                        }
                        else {
                            run_n.pull_partial(staging_buffer, &face_n, u, v, d);
                            same_row_n = true;
                        }
                    }
                    else {
                        let next_pos = pos + 0x100;
                        let (next_face_s, next_face_n) = self.get_face_pair::<{Normal::SOUTH}>(next_pos);
                        let compare = BlockFace::should_cull(next_face_s, next_face_n);

                        if compare.1 || !Run::match_faces(face_n, next_face_n) {
                            run_n.pull_partial(staging_buffer, &face_n, u, v, d);
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
                run_n.begin::<{ Normal::NORTH }>(staging_buffer, &face_n, pos, u, row_id);
            }
        } (active_run_s, active_run_n) = (false, false); row_id += 1;
        } row_id += 16;
        }
    }
    
    // U = Z
    // V = Y
    // D = X
    pub fn mesh_west_east(&mut self, staging_buffer: &mut StagingBuffer) {
        let mut row_w: [Run; 16] = Default::default();
        let mut run_w: &mut Run = &mut row_w[0];
        let mut active_run_w: bool = false;
        let mut same_row_w: bool = false;

        let mut row_e: [Run; 16] = Default::default();
        let mut run_e: &mut Run = &mut row_e[0];
        let mut active_run_e: bool = false;
        let mut same_row_e: bool = false;
        
        let mut row_id: u16 = 0;
        
        for d in 0..16_u8 { for v in 0..16_u8 { for u in 0..16_u8 {
            let pos = Chunk::pos(d, v, u);

            let (face_w, face_e) = self.get_face_pair::<{Normal::WEST}>(pos);
            let compare = BlockFace::should_cull(face_w, face_e);
            'west: {
                if self.has_other_face::<{ Normal::WEST }>(pos) {
                    Run::add_face::<{ Normal::WEST }>(staging_buffer, &self.get_other_face::<{Normal::WEST}>(pos).0, pos);
                }

                if compare.0 {
                    active_run_w = false;
                    break 'west
                }

                // /*
                if active_run_w && same_row_w {
                    if run_w.match_right(&face_w) {
                        run_w.merge_face(staging_buffer, &face_w);
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
                            run_w.pull(staging_buffer, &face_w, u, v, d);
                            active_run_w = false;
                        }
                        else {
                            run_w.pull_partial(staging_buffer, &face_w, u, v, d);
                            same_row_w = true;
                        }
                    }
                    else {
                        let next_pos = pos + 0x001;
                        let (next_face_w, next_face_e) = self.get_face_pair::<{Normal::WEST}>(next_pos);
                        let compare = BlockFace::should_cull(next_face_w, next_face_e);

                        if compare.0 || !Run::match_faces(face_w, next_face_w) {
                            run_w.pull_partial(staging_buffer, &face_w, u, v, d);
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
                run_w.begin::<{ Normal::WEST }>(staging_buffer, &face_w, pos, u, row_id);
            }
            'east: {
                // break 'east;

                if self.has_opposing_other_face::<{ Normal::WEST }>(pos) {
                    Run::add_face::<{ Normal::WEST }>(staging_buffer, &self.get_opposing_other_face::<{Normal::WEST}>(pos).0, pos);
                }

                if compare.1 {
                    active_run_e = false;
                    break 'east
                }
                // /*
                if active_run_e && same_row_e {
                    if run_e.match_right(&face_e) {
                        run_e.merge_face(staging_buffer, &face_e);
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
                            run_e.pull(staging_buffer, &face_e, u, v, d);
                            active_run_e = false;
                        }
                        else {
                            run_e.pull_partial(staging_buffer, &face_e, u, v, d);
                            same_row_e = true;
                        }
                    }
                    else {
                        let next_pos = pos + 0x001;
                        let (next_face_w, next_face_e) = self.get_face_pair::<{Normal::WEST}>(next_pos);
                        let compare = BlockFace::should_cull(next_face_w, next_face_e);

                        if compare.1 || !Run::match_faces(face_e, next_face_e) {
                            run_e.pull_partial(staging_buffer, &face_e, u, v, d);
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
                run_e.begin::<{ Normal::EAST }>(staging_buffer, &face_e, pos, u, row_id);
            }
        } (active_run_w, active_run_e) = (false, false); row_id += 1; } row_id += 16; }
    }
    
    // U = Z
    // V = X
    // D = Y
    pub fn mesh_down_up(&mut self, staging_buffer: &mut StagingBuffer) {
        let mut row_d: [Run; 16] = Default::default();
        let mut run_d: &mut Run = &mut row_d[0];
        let mut active_run_d: bool = false;
        let mut same_row_d: bool = false;

        let mut row_u: [Run; 16] = Default::default();
        let mut run_u: &mut Run = &mut row_u[0];
        let mut active_run_u: bool = false;
        let mut same_row_u: bool = false;
        
        let mut row_id: u16 = 0;
        
        for d in 0..16_u8 { for v in 0..16_u8 { for u in 0..16_u8 {
            let pos = Chunk::pos(v, d, u);

            let (face_d, face_u) = self.get_face_pair::<{Normal::DOWN}>(pos);
            let compare = BlockFace::should_cull(face_d, face_u);
            'down: {
                if self.has_other_face::<{ Normal::DOWN }>(pos) {
                    Run::add_face::<{ Normal::DOWN }>(staging_buffer, &self.get_other_face::<{Normal::DOWN}>(pos).0, pos);
                }

                if compare.0 {
                    active_run_d = false;
                    break 'down
                }

                // /*
                if active_run_d && same_row_d {
                    if run_d.match_right(&face_d) {
                        run_d.merge_face(staging_buffer, &face_d);
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
                            run_d.pull(staging_buffer, &face_d, u, v, d);
                            active_run_d = false;
                        }
                        else {
                            run_d.pull_partial(staging_buffer, &face_d, u, v, d);
                            same_row_d = true;
                        }
                    }
                    else {
                        let next_pos = pos + 0x001;
                        let (next_face_d, next_face_u) = self.get_face_pair::<{Normal::DOWN}>(next_pos);
                        let compare = BlockFace::should_cull(next_face_d, next_face_u);

                        if compare.0 || !Run::match_faces(face_d, next_face_d) {
                            run_d.pull_partial(staging_buffer, &face_d, u, v, d);
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
                run_d.begin::<{ Normal::DOWN }>(staging_buffer, &face_d, pos, u, row_id);
            }
            'up: {
                // break 'up;

                if self.has_opposing_other_face::<{ Normal::DOWN }>(pos) {
                    Run::add_face::<{ Normal::DOWN }>(staging_buffer, &self.get_opposing_other_face::<{Normal::DOWN}>(pos).0, pos);
                }

                if compare.1 {
                    active_run_u = false;
                    break 'up
                }
                // /*
                if active_run_u && same_row_u {
                    if run_u.match_right(&face_u) {
                        run_u.merge_face(staging_buffer, &face_u);
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
                            run_u.pull(staging_buffer, &face_u, u, v, d);
                            active_run_u = false;
                        }
                        else {
                            run_u.pull_partial(staging_buffer, &face_u, u, v, d);
                            same_row_u = true;
                        }
                    }
                    else {
                        let next_pos = pos + 0x001;
                        let (next_face_d, next_face_u) = self.get_face_pair::<{Normal::DOWN}>(next_pos);
                        let compare = BlockFace::should_cull(next_face_d, next_face_u);

                        if compare.1 || !Run::match_faces(face_u, next_face_u) {
                            run_u.pull_partial(staging_buffer, &face_u, u, v, d);
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
                run_u.begin::<{ Normal::UP }>(staging_buffer, &face_u, pos, u, row_id);
            }
        } (active_run_d, active_run_u) = (false, false); row_id += 1; } row_id += 16; }
    }
    
    pub fn mesh_south_north_no_merge(&mut self, staging_buffer: &mut StagingBuffer) { unsafe {
        for z in 0..16_u8 { for y in 0..16_u8 { for x in 0..16_u8 {
            let pos = Chunk::pos(x, y, z);
            
            let face_s = self.get_face::<{Normal::SOUTH}>(pos);
            let face_n = self.get_opposing_face::<{Normal::SOUTH}>(pos);

            let compare = face_s.as_u32() + 0x10101010 - face_n.as_u32();
            if compare == 0x10101010 { continue; }
            
            let offset = Chunk::INDICES_ZYX[pos] as u64;
            
            if compare < 0x10101010 { staging_buffer.put_u64(face_s.as_u64() + offset); }
            if compare > 0x10101010 { staging_buffer.put_u64(face_n.as_u64() + offset); }
            
            if self.has_other_face::<{Normal::SOUTH}>(pos) {
                staging_buffer.put_u64(self.get_other_face::<{Normal::SOUTH}>(pos).0.as_u64() + offset);
            }
            if self.has_opposing_other_face::<{Normal::SOUTH}>(pos) {
                staging_buffer.put_u64(self.get_opposing_other_face::<{Normal::SOUTH}>(pos).0.as_u64() + offset);
            }
        } } }
    } }
    pub fn mesh_west_east_no_merge(&mut self, staging_buffer: &mut StagingBuffer) { unsafe {
        for x in 0..16_u8 { for y in 0..16_u8 { for z in 0..16_u8 {
            let pos = Chunk::pos(x, y, z);

            let face_w = self.get_face::<{Normal::WEST}>(pos);
            let face_e = self.get_opposing_face::<{Normal::WEST}>(pos);

            let compare = face_w.as_u32() + 0x10101010 - face_e.as_u32();
            if compare == 0x10101010 { continue; }
            
            let offset = Chunk::INDICES_XYZ[pos] as u64;
            
            if compare < 0x10101010 { staging_buffer.put_u64(face_w.as_u64() + offset); }
            if compare > 0x10101010 { staging_buffer.put_u64(face_e.as_u64() + offset); }

            if self.has_other_face::<{Normal::WEST}>(pos) {
                staging_buffer.put_u64(self.get_other_face::<{Normal::WEST}>(pos).0.as_u64() + offset);
            }
            if self.has_opposing_other_face::<{Normal::WEST}>(pos) {
                staging_buffer.put_u64(self.get_opposing_other_face::<{Normal::WEST}>(pos).0.as_u64() + offset);
            }
        } } }
    } }
    pub fn mesh_down_up_no_merge(&mut self, staging_buffer: &mut StagingBuffer) { unsafe {
        for x in 0..16_u8 { for y in 0..16_u8 { for z in 0..16_u8 {
            let pos = Chunk::pos(x, y, z);

            let face_d = self.get_face::<{Normal::DOWN}>(pos);
            let face_u = self.get_opposing_face::<{Normal::DOWN}>(pos);

            let compare = face_d.as_u32() + 0x10101010 - face_u.as_u32();
            if compare == 0x10101010 { continue; }
            
            let offset = Chunk::INDICES_YXZ[pos] as u64;
            
            if compare < 0x10101010 { staging_buffer.put_u64(face_d.as_u64() + offset); }
            if compare > 0x10101010 { staging_buffer.put_u64(face_u.as_u64() + offset); }

            if self.has_other_face::<{Normal::DOWN}>(pos) {
                staging_buffer.put_u64(self.get_other_face::<{Normal::DOWN}>(pos).0.as_u64() + offset);
            }
            if self.has_opposing_other_face::<{Normal::DOWN}>(pos) {
                staging_buffer.put_u64(self.get_opposing_other_face::<{Normal::DOWN}>(pos).0.as_u64() + offset);
            }
        } } }
    } }

    pub fn generate_geometry_buffer(&mut self, staging_buffer: &mut StagingBuffer, buffer_allocator: &mut BufferPoolAllocator<1048576, 1024>) { unsafe {
        // self.mesh_south_north(staging_buffer);
        // self.mesh_west_east(staging_buffer);
        // self.mesh_down_up(staging_buffer);

        self.mesh_south_north_no_merge(staging_buffer);
        self.mesh_west_east_no_merge(staging_buffer);
        self.mesh_down_up_no_merge(staging_buffer);

        staging_buffer.format_quads();

        self.face_count = staging_buffer.index as u32 / 8;
        
        self.geometry_page = buffer_allocator.allocate(staging_buffer.index);
        if let Some(page) = &self.geometry_page {
            buffer_allocator.upload(page, &staging_buffer.buffer.0.as_slice(), staging_buffer.index as isize);
        }
    } }

    /*
    pub fn generate_light_buffer(&mut self, staging_buffer: &mut StagingBuffer, buffer_allocator: &mut BufferPoolAllocator<32768, { 18 * 18 * 18 / 2 }>) { unsafe {
        'empty_check: {
            for i in 0..4096 {
                if self.light[i] != 0 {
                    break 'empty_check;
                }
            }
            self.light_page = Some(
                Page { start: 0, size: NonZeroUsize::new_unchecked(1) }
            );
            return;
        }
        self.light_page = buffer_allocator.allocate(1);
        let mut light_staging_buffer = [0; 18 * 18 * 18];
        let i = 0;
        for x in -1_isize..17 {
            for y in -1_isize..17 {
                for z in -1_isize..17 {
                    let mut chunk = &*self;
                    if x == -1 {
                        if let Some(west) = chunk.neighbors.west {
                            chunk = &*west;
                        } else { continue; }
                    }
                    else if x == 17 {
                        if let Some(east) = chunk.neighbors.east {
                            chunk = &*east;
                        } else { continue; }
                    }
                    if y == -1 {
                        if let Some(down) = chunk.neighbors.down {
                            chunk = &*down;
                        } else { continue; }
                    }
                    else if y == 17 {
                        if let Some(up) = chunk.neighbors.up {
                            chunk = &*up;
                        } else { continue; }
                    }
                    if z == -1 {
                        if let Some(south) = chunk.neighbors.south {
                            chunk = &*south;
                        } else { continue; }
                    }
                    else if z == 17 {
                        if let Some(north) = chunk.neighbors.north {
                            chunk = &*north;
                        } else { continue; }
                    }

                    light_staging_buffer[i] = chunk.light[((x & 0x7 << 8) | (y & 0x7 << 4) | (z & 0x7 << 0)) as usize];
                }
            }
        }
    } }
    */

    pub fn generate_light_buffer(&mut self, geometry_staging_buffer: &mut StagingBuffer, light_staging_buffer: &mut StagingBuffer, buffer_allocator: &mut BufferPoolAllocator<1048576, 1024>) { unsafe {
        const BLOCKS_PER_QUAD: usize = 8;
        // reserve bytes for indices
        light_staging_buffer.index = geometry_staging_buffer.index / BLOCKS_PER_QUAD * mem::size_of::<u32>();
        for (i, quad) in geometry_staging_buffer.iter().map(|quad| &*(quad as *const [u8; 8] as *const GpuQuad)).enumerate() {
            *(&mut light_staging_buffer[i * mem::size_of::<u32>()] as *mut u8 as *mut u32) = light_staging_buffer.index as u32;
            match quad.nor {
                Normal::SOUTH => {
                    let start_x = quad.ure / 16;
                    let end_x = (quad.ure + quad.wid + 1) / 16;
    
                    let start_y = quad.ven / 16;
                    let end_y = (quad.ven + quad.hei + 1) / 16;
    
                    let z = (quad.dep + 1) / 16;
                    for x in start_x..end_x {
                        for y in start_y..end_y {
                            light_staging_buffer.put_u32(self.light[Chunk::pos(x, y, z)] as u32);
                        }
                    }
                }
                Normal::NORTH => {
                    let start_x = quad.ure / 16;
                    let end_x = (quad.ure + quad.wid + 1) / 16;
    
                    let start_y = quad.ven / 16;
                    let end_y = (quad.ven + quad.hei + 1) / 16;
    
                    let z = quad.dep / 16;
                    for x in start_x..end_x {
                        for y in start_y..end_y {
                            light_staging_buffer.put_u32(self.light[Chunk::pos(x, y, z)] as u32);
                        }
                    }
                }
                Normal::WEST => {
                    let x = quad.dep / 16;
    
                    let start_y = quad.ven / 16;
                    let end_y = (quad.ven + quad.hei + 1) / 16;
    
                    let start_z = quad.ure / 16;
                    let end_z = (quad.ure + quad.wid + 1) / 16;
                    
                    for y in start_y..end_y {
                        for z in start_z..end_z {
                            light_staging_buffer.put_u32(self.light[Chunk::pos(x, y, z)] as u32);
                        }
                    }
                }
                Normal::EAST => {
                    let x = (quad.dep + 1) / 16;
    
                    let start_y = quad.ven / 16;
                    let end_y = (quad.ven + quad.hei + 1) / 16;
    
                    let start_z = quad.ure / 16;
                    let end_z = (quad.ure + quad.wid + 1) / 16;
                    
                    for y in start_y..end_y {
                        for z in start_z..end_z {
                            light_staging_buffer.put_u32(self.light[Chunk::pos(x, y, z)] as u32);
                        }
                    }
                }
                Normal::DOWN => {
                    let start_x = quad.ven / 16;
                    let end_x = (quad.ven + quad.hei + 1) / 16;
    
                    let y = quad.dep / 16;
    
                    let start_z = quad.ure / 16;
                    let end_z = (quad.ure + quad.wid + 1) / 16;
                    
                    for x in start_x..end_x {
                        for z in start_z..end_z {
                            light_staging_buffer.put_u32(self.light[Chunk::pos(x, y, z)] as u32);
                        }
                    }
                }
                Normal::UP => {
                    let start_x = quad.ven / 16;
                    let end_x = (quad.ven + quad.hei + 1) / 16;
    
                    let y = (quad.dep + 1) / 16;
    
                    let start_z = quad.ure / 16;
                    let end_z = (quad.ure + quad.wid + 1) / 16;
                    
                    for x in start_x..end_x {
                        for z in start_z..end_z {
                            light_staging_buffer.put_u32(self.light[Chunk::pos(x, y, z)] as u32);
                        }
                    }
                }
    
                _ => {
                    unreachable_unchecked();
                }
            }
        }
        /*
        let nor = mem::transmute::<u8, Normal>(nor);
        match nor {
            Normal::SOUTH => {
                let start_x = ure / 16;
                let end_x = (ure + wid + 1) / 16;

                let start_y = ven / 16;
                let end_y = (ven + hei + 1) / 16;

                let z = (dep + 1) / 16;
                for x in start_x..end_x {
                    for y in start_y..end_y {
                        self.light_index += 1;
                        self.light[self.light_index] = chunk.light[((x as usize) << 8) | ((y as usize) << 4) | ((z as usize) << 0)];
                    }
                }
            }
            Normal::NORTH => {
                let start_x = ure / 16;
                let end_x = (ure + wid + 1) / 16;

                let start_y = ven / 16;
                let end_y = (ven + hei + 1) / 16;

                let z = dep / 16;
                for x in start_x..end_x {
                    for y in start_y..end_y {
                        self.light_index += 1;
                        self.light[self.light_index] = chunk.light[((x as usize) << 8) | ((y as usize) << 4) | ((z as usize) << 0)];
                    }
                }
            }
            Normal::WEST => {
                let x = dep / 16;

                let start_y = ven / 16;
                let end_y = (ven + hei + 1) / 16;

                let start_z = ure / 16;
                let end_z = (ure + wid + 1) / 16;
                
                for y in start_y..end_y {
                    for z in start_z..end_z {
                        self.light_index += 1;
                        self.light[self.light_index] = chunk.light[((x as usize) << 8) | ((y as usize) << 4) | ((z as usize) << 0)];
                    }
                }
            }
            Normal::EAST => {
                let x = (dep + 1) / 16;

                let start_y = ven / 16;
                let end_y = (ven + hei + 1) / 16;

                let start_z = ure / 16;
                let end_z = (ure + wid + 1) / 16;
                
                for y in start_y..end_y {
                    for z in start_z..end_z {
                        self.light_index += 1;
                        self.light[self.light_index] = chunk.light[((x as usize) << 8) | ((y as usize) << 4) | ((z as usize) << 0)];
                    }
                }
            }
            Normal::DOWN => {
                let start_x = ven / 16;
                let end_x = (ven + hei + 1) / 16;

                let y = dep / 16;

                let start_z = ure / 16;
                let end_z = (ure + wid + 1) / 16;
                
                for x in start_x..end_x {
                    for z in start_z..end_z {
                        self.light_index += 1;
                        self.light[self.light_index] = chunk.light[((x as usize) << 8) | ((y as usize) << 4) | ((z as usize) << 0)];
                    }
                }
            }
            Normal::UP => {
                let start_x = ven / 16;
                let end_x = (ven + hei + 1) / 16;

                let y = (dep + 1) / 16;

                let start_z = ure / 16;
                let end_z = (ure + wid + 1) / 16;
                
                for x in start_x..end_x {
                    for z in start_z..end_z {
                        self.light_index += 1;
                        self.light[self.light_index] = chunk.light[((x as usize) << 8) | ((y as usize) << 4) | ((z as usize) << 0)];
                    }
                }
            }
            _ => unreachable_unchecked()
        }
        // */

    } }

    pub fn cull_backfaces(&mut self, world: &mut World) {
        // let south = self.counts[0];
        // let north = self.counts[3];    
        // if world.camera.pos.z > self.pos.z as f32 * 16.0 + 16.0 { self.counts[0] = 0 }
        // if world.camera.pos.z < self.pos.z as f32 * 16.0 { self.counts[3] = 0 }

        // let west = self.counts[1];
        // let east = self.counts[4];
        // if world.camera.pos.x > self.pos.x as f32 * 16.0 + 16.0 { self.counts[1] = 0 }
        // if world.camera.pos.x < self.pos.x as f32 * 16.0 { self.counts[4] = 0 }

        // let down = self.counts[2];
        // let up = self.counts[5];
        // if world.camera.pos.y > self.pos.y as f32 * 16.0 + 16.0 { self.counts[2] = 0 }
        // if world.camera.pos.y < self.pos.y as f32 * 16.0 { self.counts[5] = 0 }

        // unsafe { gl::MultiDrawElements(
        //     gl::TRIANGLE_STRIP,
        //     &raw const self.counts as *const i32,
        //     gl::UNSIGNED_INT,
        //     &raw const self.offsets as *const *const c_void,
        //     6
        // ) };

        // self.counts[0] = south;
        // self.counts[1] = west;
        // self.counts[2] = down;
        // self.counts[3] = north;
        // self.counts[4] = east;
        // self.counts[5] = up;
    }

    pub fn pos<T: TryInto<usize>>(x: T, y: T, z: T) -> usize { unsafe {
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
    fn match_right(&self, face: &BlockFace) -> bool { unsafe {
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
    fn match_top_left(&self, face: &BlockFace) -> bool { unsafe {
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
    fn match_top_right(&self, face: &BlockFace) -> bool { unsafe {
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
    fn merge_face(&mut self, buffer: &mut StagingBuffer, face: &BlockFace) { unsafe {
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
    fn pull_partial(&mut self, buffer: &mut StagingBuffer, face: &BlockFace, u: u8, v: u8, d: u8) {
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
    fn pull(&mut self, buffer: &mut StagingBuffer, face: &BlockFace, u: u8, v: u8, d: u8) {
        buffer[self.idx as usize + 4] += 0x10;
        buffer[self.idx as usize + 3] &= 0xf0;
        buffer[self.idx as usize + 3] |= face.top;
        self.top = face.top;
        self.row += 1;
    }
    /**
     * Begins a new run
     */
    fn begin<const NORMAL: Normal>(&mut self, buffer: &mut StagingBuffer, face: &BlockFace, pos: usize, u: u8, row: u16) {
        self.idx = buffer.index as u16;
        let offset: u32;
        match NORMAL {
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
    fn add_face<const NORMAL: Normal>(buffer: &mut StagingBuffer, face: &BlockFace, pos: usize) {
        let offset: u32;
        match NORMAL {
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
    fn match_faces(face: &BlockFace, next: &BlockFace) -> bool {
        return (
            face.tex == next.tex &&
            face.rig == next.lef &&
            face.bot == next.bot &&
            face.top == next.top
        );
    }
    
    fn as_u32(&self) -> &u32 { unsafe {
        return &*(&raw const self as *mut u32);
    } }

    fn new() -> Run {
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

#[derive(Debug)]
#[derive(Clone, Copy)]
#[derive(Eq, PartialEq, Hash)]
#[repr(C)]
pub struct Vec3i {
    pub x: i32, pub y: i32, pub z: i32
}