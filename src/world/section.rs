use std::hint;
use std::hint::unreachable_unchecked;
use std::mem;
use std::ptr::NonNull;
use cgmath::Vector3;
use cgmath_culling::BoundingBox;
use cgmath_culling::Sphere;

use crate::block::blockstate::BLOCKS;
use crate::block::normal::Normal::{self, *};
use crate::gl_util::buffer_allocator::{BufferAllocator, SortType::*};
use crate::gl_util::buffer_allocator::BufferSegment;
use crate::render::byte_buffer::StagingBuffer;
use crate::render::gpu_quad::GpuQuad;
use crate::block::{blockstate::BlockState, blockface::BlockFace};
use super::blockpos::BlockPos;
use super::camera::Camera;
use super::run::Run;
use super::world::World;

/// A 16x16x16 volume of blocks

#[derive(Debug, Clone)]
pub struct Section {
    // block ids
    pub blocks: [u16; 4096],
    // light levels 0-15
    pub light: [u8; 4096],
    // section position
    pub section_pos: Vector3<i32>,
    // adjacent chunks
    pub neighbors: SectionNeighbors,
    // if the chunk needs to be remeshed
    pub dirty: bool,
    // number of rectangular block faces in a section
    pub solid_quad_count: u32,
    pub translucent_quad_count: u32,
    pub solid_segment: Option<BufferSegment>,
    pub translucent_segment: Option<BufferSegment>,
    pub light_segment: Option<BufferSegment>
}

#[derive(Debug, Clone)]
pub struct SectionNeighbors {
    pub south: Option<NonNull<Section>>,
    pub west: Option<NonNull<Section>>,
    pub down: Option<NonNull<Section>>,
    pub north: Option<NonNull<Section>>,
    pub east: Option<NonNull<Section>>,
    pub up: Option<NonNull<Section>>,
}

impl Section {
    pub fn set_block(&mut self, block_pos: BlockPos, block_id: u16) {
        if block_id != self.blocks[block_pos.index] {
            self.dirty = true;
            self.blocks[block_pos.index] = block_id;
            if block_pos.x() == 0 && let Some(west) = self.get_neighbor::<{West}>() {
                west.dirty = true;
            }
            if block_pos.x() == 15 && let Some(east) = self.get_neighbor::<{East}>() {
                east.dirty = true;
            }
            if block_pos.z() == 0 && let Some(south) = self.get_neighbor::<{South}>() {
                south.dirty = true;
            }
            if block_pos.z() == 15 && let Some(north) = self.get_neighbor::<{North}>() {
                north.dirty = true;
            }
            if block_pos.y() == 0 && let Some(down) = self.get_neighbor::<{Down}>() {
                down.dirty = true;
            }
            if block_pos.y() == 15 && let Some(up) = self.get_neighbor::<{Up}>() {
                up.dirty = true;
            }
        }
    }

    pub fn get_neighbor<const N: Normal>(&self) -> Option<&mut Section> { unsafe {
        match N {
            North => {
                return mem::transmute(self.neighbors.north);
            }
            South => {
                return mem::transmute(self.neighbors.south);
            }
            East => {
                return mem::transmute(self.neighbors.east);
            }
            West => {
                return mem::transmute(self.neighbors.west);
            }
            Up => {
                return mem::transmute(self.neighbors.up);
            }
            Down => {
                return mem::transmute(self.neighbors.down);
            }
            _ => { hint::unreachable_unchecked(); }
        }
    } }

    pub fn get_block(&self, block_pos: BlockPos) -> &BlockState { unsafe {
        return &BLOCKS[self.blocks[block_pos.index] as usize];
    } }
    pub fn get_opposing_block<const N: Normal>(&self, pos: BlockPos) -> &BlockState { unsafe {
        match N {
            South => {
                if pos.z() == 0 {
                    return self.get_neighbor::<N>().map_or(&BLOCKS[0], |section| section.get_block(pos.set_z(15)));
                }
                return &self.get_block(pos - BlockPos::new(0, 0, 1));
            }
            West => {
                if pos.x() == 0 {
                    return self.get_neighbor::<N>().map_or(&BLOCKS[0], |section| section.get_block(pos.set_x(15)));
                }
                return &self.get_block(pos - BlockPos::new(1, 0, 0));
            }
            Down => {
                if pos.y() == 0 {
                    return self.get_neighbor::<N>().map_or(&BLOCKS[0], |section| section.get_block(pos.set_y(15)));
                }
                return &self.get_block(pos - BlockPos::new(0, 1, 0));
            }
            North => {
                if pos.z() == 15 {
                    return self.get_neighbor::<N>().map_or(&BLOCKS[0], |section| section.get_block(pos.set_z(0)));
                }
                return &self.get_block(pos + BlockPos::new(0, 0, 1));
            }
            East => {
                if pos.x() == 15 {
                    return self.get_neighbor::<N>().map_or(&BLOCKS[0], |section| section.get_block(pos.set_x(0)));
                }
                return &self.get_block(pos + BlockPos::new(1, 0, 0));
            }
            Up => {
                if pos.y() == 15 {
                    return self.get_neighbor::<N>().map_or(&BLOCKS[0], |section| section.get_block(pos.set_y(0)));
                }
                return &self.get_block(pos + BlockPos::new(0, 1, 0));
            }
            _ => unsafe { hint::unreachable_unchecked(); }
        }
    } }

    pub fn get_face<const N: Normal>(&self, block_pos: BlockPos) -> &BlockFace {
        return &self.get_block(block_pos).model.get_face(N);
    }
    pub fn get_opposing_face<const N: Normal>(&self, block_pos: BlockPos) -> &BlockFace {
        return &self.get_opposing_block::<N>(block_pos).model.get_face(N.reverse());
    }

    pub fn get_light(&self, block_pos: BlockPos) -> u8 {
        return self.light[block_pos.index];
    }
    pub fn get_face_light<const N: Normal>(&self, block_pos: BlockPos) -> u8 { unsafe {
        match N {
            North => {
                if block_pos.z() == 15 {
                    return self.get_neighbor::<N>().map_or(0, |section| section.get_light(block_pos.set_z(0)));
                }
                return self.get_light(block_pos + BlockPos::new(0, 0, 1));
            }
            South => {
                if block_pos.z() == 0 {
                    return self.get_neighbor::<N>().map_or(0, |section| section.get_light(block_pos.set_z(15)));
                }
                return self.get_light(block_pos - BlockPos::new(0, 0, 1));
            }
            East => {
                if block_pos.x() == 15 {
                    return self.get_neighbor::<N>().map_or(0, |section| section.get_light(block_pos.set_x(0)));
                }
                return self.get_light(block_pos + BlockPos::new(1, 0, 0));
            }
            West => {
                if block_pos.x() == 0 {
                    return self.get_neighbor::<N>().map_or(0, |section| section.get_light(block_pos.set_x(15)));
                }
                return self.get_light(block_pos - BlockPos::new(1, 0, 0));
            }
            Up => {
                if block_pos.y() == 15 {
                    return self.get_neighbor::<N>().map_or(0, |section| section.get_light(block_pos.set_y(0)));
                }
                return self.get_light(block_pos + BlockPos::new(0, 1, 0));
            }
            Down => {
                if block_pos.y() == 0 {
                    return self.get_neighbor::<N>().map_or(0, |section| section.get_light(block_pos.set_y(15)));
                }
                return self.get_light(block_pos - BlockPos::new(0, 1, 0));
            }
            _ => { hint::unreachable_unchecked(); }
        }
    } }

    pub fn get_face_pair<const N: Normal>(&self, block_pos: BlockPos) -> (&BlockFace, &BlockFace) {
        return (self.get_face::<N>(block_pos), self.get_opposing_face::<N>(block_pos))
    }

    pub fn has_extra_face<const N: Normal>(&self, block_pos: BlockPos) -> bool { unsafe {
        match N {
            South => { return self.get_block(block_pos).model.extra_south.len() > 0; }
            West => { return self.get_block(block_pos).model.extra_west.len() > 0; }
            Down => { return self.get_block(block_pos).model.extra_down.len() > 0; }
            _ => { hint::unreachable_unchecked(); }
        }
    } }
    pub fn has_opposing_extra_face<const N: Normal>(&self, block_pos: BlockPos) -> bool { unsafe {
        match N {
            South => { return self.get_opposing_block::<N>(block_pos).model.extra_north.len() > 0; }
            West => { return self.get_opposing_block::<N>(block_pos).model.extra_east.len() > 0; }
            Down => { return self.get_opposing_block::<N>(block_pos).model.extra_up.len() > 0; }
            _ => { hint::unreachable_unchecked(); }
        }
    } }

    pub fn get_extra_face<const N: Normal>(&self, block_pos: BlockPos) -> &[BlockFace] { unsafe {
        match N {
            South => { return self.get_block(block_pos).model.extra_south; }
            West => { return self.get_block(block_pos).model.extra_west; }
            Down => { return self.get_block(block_pos).model.extra_down; }
            North => { return self.get_block(block_pos).model.extra_north; }
            East => { return self.get_block(block_pos).model.extra_east; }
            Up => { return self.get_block(block_pos).model.extra_up; }
            _ => { unreachable_unchecked(); }
        }
    } }
    pub fn get_opposing_extra_face<const N: Normal>(&self, block_pos: BlockPos) -> &[BlockFace] { unsafe {
        match N {
            South => { return self.get_opposing_block::<N>(block_pos).model.extra_north; }
            West => { return self.get_opposing_block::<N>(block_pos).model.extra_east; }
            Down => { return self.get_opposing_block::<N>(block_pos).model.extra_up; }
            North => { return self.get_opposing_block::<N>(block_pos).model.extra_south; }
            East => { return self.get_opposing_block::<N>(block_pos).model.extra_west; }
            Up => { return self.get_opposing_block::<N>(block_pos).model.extra_down; }
            _ => { unreachable_unchecked(); }
        }
    } }

    pub fn make_terrain(&mut self, noise: &Vec<f32>) { unsafe {
        for x in 0..16 { for y in 0..16 { for z in 0..16 {
            let max_world_y = World::MAX_SECTION_Y * 16;
            let max_world_z = World::MAX_SECTION_Z * 16;
            let world_x = self.section_pos.x as usize * 16 + x;
            let world_y = self.section_pos.y as usize * 16 + y;
            let world_z = self.section_pos.z as usize * 16 + z;
            let noise_index = {
                world_x * max_world_z * max_world_y +
                world_y * max_world_z +
                world_z
            };
            let (noise_val, block, light) = (
                noise.get_unchecked(noise_index),
                self.blocks.get_unchecked_mut(BlockPos::new(x, y, z).index),
                self.light.get_unchecked_mut(BlockPos::new(x, y, z).index),
            );
            *block = match *noise_val {
                val if val < 0.5 => {
                    4
                },
                // val if val < 0.51 => {
                //     3
                // },
                _ => {
                    0
                }
            };
            *light = rand::random::<u8>() & 0xf;
            // *light = (*noise_val * 64.0 - 32.0) as u8 & 0xf;
        } } }
    } }
    pub fn make_terrain_alt(&mut self) {
        for i in 0..4096 {
            let pos = BlockPos { index: i };
            if pos.y() == 0 {
                self.blocks[i] = 4;
            }
            self.light[i] = rand::random::<u8>() % 16;
        }
        // self.blocks[BlockPos::new(1, 1, 1).index] = 3;
        // self.blocks[BlockPos::new(1, 2, 1).index] = 3;
    }

    pub fn set_chunk_pos(&mut self, chunk_pos: Vector3<i32>) {
        self.section_pos = chunk_pos;
    }
    
    // rel_x, rel_y, rel_z = x, y, z
    pub fn mesh_south_north(&mut self, solid_staging_buffer: &mut StagingBuffer) {
        hint::black_box(0x1312ACAB);
        let mut row_s: [Run; 16] = Default::default();
        let mut run_s: &mut Run = &mut row_s[0];
        let mut active_run_s: bool = false;
        let mut same_row_s: bool = false;

        let mut row_n: [Run; 16] = Default::default();
        let mut run_n: &mut Run = &mut row_n[0];
        let mut active_run_n: bool = false;
        let mut same_row_n: bool = false;
        
        let mut row_id: u16 = 0;

        for rel_z in 0..16_u8 {
            for rel_y in 0..16_u8 {
                for rel_x in 0..16_u8 {
                    let block_pos = BlockPos::new(rel_x, rel_y, rel_z);

                    let (face_s, face_n) = self.get_face_pair::<{South}>(block_pos);
                    let compare = BlockFace::should_cull_pair(face_s, face_n);
                    'south: {
                        for face in self.get_extra_face::<{South}>(block_pos) {
                            Run::add_face::<{South}>(solid_staging_buffer, face, block_pos);
                        }
            
                        if compare.0 {
                            active_run_s = false;
                            break 'south
                        }
                        // /*
                        if active_run_s && same_row_s {
                            if run_s.match_right(&face_s) {
                                run_s.merge_face(solid_staging_buffer, &face_s);
                                break 'south
                            } else {
                                active_run_s = false;
                            }
                        }
                        // /*
                        if !active_run_s {
                            run_s = &mut row_s[rel_x as usize];
                            if run_s.row + 1 == row_id && run_s.match_top_left(&face_s) {
                                same_row_s = false;
                                active_run_s = true;
                            }
                        }
                        if active_run_s {
                            if run_s.end == rel_x {
                                if run_s.match_top_right(&face_s) {
                                    run_s.pull(solid_staging_buffer, &face_s, rel_x, rel_y, rel_z);
                                    active_run_s = false;
                                }
                                else {
                                    run_s.pull_partial(solid_staging_buffer, &face_s, rel_x, rel_y, rel_z);
                                    same_row_s = true;
                                }
                            }
                            else {
                                let next_block_pos = block_pos + BlockPos::new(1, 0, 0);
                                let (next_face_s, next_face_n) = self.get_face_pair::<{South}>(next_block_pos);
                                let compare = BlockFace::should_cull_pair(next_face_s, next_face_n);

                                if compare.0 || !Run::match_faces(face_s, next_face_s) {
                                    run_s.pull_partial(solid_staging_buffer, &face_s, rel_x, rel_y, rel_z);
                                    active_run_s = false;
                                }
                            }
                            break 'south
                        }
                        // */
                        // */
                        run_s = &mut row_s[rel_x as usize];
                        same_row_s = true;
                        active_run_s = true;
                        run_s.begin::<{South}>(solid_staging_buffer, &face_s, block_pos, rel_x, row_id);
                    }
                    'north: {
                        // break 'north;
                        for face in self.get_opposing_extra_face::<{South}>(block_pos) {
                            Run::add_face::<{South}>(solid_staging_buffer, &face, block_pos);
                        }

                        if compare.1 {
                            active_run_n = false;
                            break 'north;
                        }
                        // /*
                        if active_run_n && same_row_n {
                            if run_n.match_right(&face_n) {
                                run_n.merge_face(solid_staging_buffer, &face_n);
                                break 'north;
                            } else {
                                active_run_n = false;
                            }
                        }
                        // /*
                        if !active_run_n {
                            run_n = &mut row_n[rel_x as usize];
                            if run_n.row + 1 == row_id && run_n.match_top_left(&face_n) {
                                same_row_n = false;
                                active_run_n = true;
                            }
                        }
                        if active_run_n {
                            if run_n.end == rel_x {
                                if run_n.match_top_right(&face_n) {
                                    run_n.pull(solid_staging_buffer, &face_n, rel_x, rel_y, rel_z);
                                    active_run_n = false;
                                }
                                else {
                                    run_n.pull_partial(solid_staging_buffer, &face_n, rel_x, rel_y, rel_z);
                                    same_row_n = true;
                                }
                            }
                            else {
                                let next_block_pos = block_pos + BlockPos::new(1, 0, 0);
                                let (next_face_s, next_face_n) = self.get_face_pair::<{South}>(next_block_pos);
                                let compare = BlockFace::should_cull_pair(next_face_s, next_face_n);

                                if compare.1 || !Run::match_faces(face_n, next_face_n) {
                                    run_n.pull_partial(solid_staging_buffer, &face_n, rel_x, rel_y, rel_z);
                                    active_run_n = false;
                                }
                            }
                            break 'north
                        }
                        // */
                        // */
                        run_n = &mut row_n[rel_x as usize];
                        active_run_n = true;
                        same_row_n = true;
                        run_n.begin::<{North}>(solid_staging_buffer, &face_n, block_pos, rel_x, row_id);
                    }
                }
                (active_run_s, active_run_n) = (false, false);
                row_id += 1;
            }
            row_id += 16;
        }
    }
    
    // rel_x, rel_y, rel_z = z, y, x
    pub fn mesh_west_east(&mut self, solid_staging_buffer: &mut StagingBuffer) {
        let mut row_w: [Run; 16] = Default::default();
        let mut run_w: &mut Run = &mut row_w[0];
        let mut active_run_w: bool = false;
        let mut same_row_w: bool = false;

        let mut row_e: [Run; 16] = Default::default();
        let mut run_e: &mut Run = &mut row_e[0];
        let mut active_run_e: bool = false;
        let mut same_row_e: bool = false;
        
        let mut row_id: u16 = 0;
        
        for rel_z in 0..16_u8 {
            for rel_y in 0..16_u8 {
                for rel_x in 0..16_u8 {
                    let block_pos = BlockPos::new(rel_z, rel_y, rel_x);
                    
                    let (face_w, face_e) = self.get_face_pair::<{West}>(block_pos);
                    let compare = BlockFace::should_cull_pair(face_w, face_e);
                    'west: {
                        for face in self.get_extra_face::<{West}>(block_pos) {
                            Run::add_face::<{West}>(solid_staging_buffer, face, block_pos);
                        }
            
            
                        if compare.0 {
                            active_run_w = false;
                            break 'west
                        }

                        // /*
                        if active_run_w && same_row_w {
                            if run_w.match_right(&face_w) {
                                run_w.merge_face(solid_staging_buffer, &face_w);
                                break 'west
                            } else {
                                active_run_w = false;
                            }
                        }
                        // /*
                        if !active_run_w {
                            run_w = &mut row_w[rel_x as usize];
                            if run_w.row + 1 == row_id && run_w.match_top_left(&face_w) {
                                same_row_w = false;
                                active_run_w = true;
                            }
                        }
                        if active_run_w {
                            if run_w.end == rel_x {
                                if run_w.match_top_right(&face_w) {
                                    run_w.pull(solid_staging_buffer, &face_w, rel_x, rel_y, rel_z);
                                    active_run_w = false;
                                }
                                else {
                                    run_w.pull_partial(solid_staging_buffer, &face_w, rel_x, rel_y, rel_z);
                                    same_row_w = true;
                                }
                            }
                            else {
                                let next_block_pos = block_pos + BlockPos::new(0, 0, 1);
                                let (next_face_w, next_face_e) = self.get_face_pair::<{West}>(next_block_pos);
                                let compare = BlockFace::should_cull_pair(next_face_w, next_face_e);

                                if compare.0 || !Run::match_faces(face_w, next_face_w) {
                                    run_w.pull_partial(solid_staging_buffer, &face_w, rel_x, rel_y, rel_z);
                                    active_run_w = false;
                                }
                            }
                            break 'west
                        }
                        // */
                        // */
                        run_w = &mut row_w[rel_x as usize];
                        same_row_w = true;
                        active_run_w = true;
                        run_w.begin::<{West}>(solid_staging_buffer, &face_w, block_pos, rel_x, row_id);
                    }
                    'east: {
                        // break 'east;

                        for face in self.get_opposing_extra_face::<{West}>(block_pos) {
                            Run::add_face::<{East}>(solid_staging_buffer, face, block_pos);
                        }
                        if compare.1 {
                            active_run_e = false;
                            break 'east
                        }
                        // /*
                        if active_run_e && same_row_e {
                            if run_e.match_right(&face_e) {
                                run_e.merge_face(solid_staging_buffer, &face_e);
                                break 'east
                            } else {
                                active_run_e = false;
                            }
                        }
                        // /* 
                        if !active_run_e {
                            run_e = &mut row_e[rel_x as usize];
                            if run_e.row + 1 == row_id && run_e.match_top_left(&face_e) {
                                same_row_e = false;
                                active_run_e = true;
                            }
                        }

                        if active_run_e {
                            if run_e.end == rel_x {
                                if run_e.match_top_right(&face_e) {
                                    run_e.pull(solid_staging_buffer, &face_e, rel_x, rel_y, rel_z);
                                    active_run_e = false;
                                }
                                else {
                                    run_e.pull_partial(solid_staging_buffer, &face_e, rel_x, rel_y, rel_z);
                                    same_row_e = true;
                                }
                            }
                            else {
                                let next_block_pos = block_pos + BlockPos::new(0, 0, 1);
                                let (next_face_w, next_face_e) = self.get_face_pair::<{West}>(next_block_pos);
                                let compare = BlockFace::should_cull_pair(next_face_w, next_face_e);

                                if compare.1 || !Run::match_faces(face_e, next_face_e) {
                                    run_e.pull_partial(solid_staging_buffer, &face_e, rel_x, rel_y, rel_z);
                                    active_run_e = false;
                                }
                            }
                            break 'east
                        }
                        // */
                        // */
                        run_e = &mut row_e[rel_x as usize];
                        active_run_e = true;
                        same_row_e = true;
                        run_e.begin::<{East}>(solid_staging_buffer, &face_e, block_pos, rel_x, row_id);
                    }
                }
                (active_run_w, active_run_e) = (false, false); row_id += 1;
            }
            row_id += 16;
        }
    }
    
    // rel_x, rel_y, rel_z = z, x, y
    pub fn mesh_down_up(&mut self, solid_staging_buffer: &mut StagingBuffer) {
        let mut row_d: [Run; 16] = Default::default();
        let mut run_d: &mut Run = &mut row_d[0];
        let mut active_run_d: bool = false;
        let mut same_row_d: bool = false;

        let mut row_u: [Run; 16] = Default::default();
        let mut run_u: &mut Run = &mut row_u[0];
        let mut active_run_u: bool = false;
        let mut same_row_u: bool = false;
        
        let mut row_id: u16 = 0;
        
        for rel_z in 0..16_u8 {
            for rel_y in 0..16_u8 {
                for rel_x in 0..16_u8 {
                    let block_pos = BlockPos::new(rel_y, rel_z, rel_x);

                    let (face_d, face_u) = self.get_face_pair::<{Down}>(block_pos);
                    let compare = BlockFace::should_cull_pair(face_d, face_u);
                    'down: {
                        for face in self.get_extra_face::<{Down}>(block_pos) {
                            Run::add_face::<{Down}>(solid_staging_buffer, &face, block_pos);
                        }
            
                        if compare.0 {
                            active_run_d = false;
                            break 'down
                        }

                        // /*
                        if active_run_d && same_row_d {
                            if run_d.match_right(&face_d) {
                                run_d.merge_face(solid_staging_buffer, &face_d);
                                break 'down
                            } else {
                                active_run_d = false;
                            }
                        }
                        // /*
                        if !active_run_d {
                            run_d = &mut row_d[rel_x as usize];
                            if run_d.row + 1 == row_id && run_d.match_top_left(&face_d) {
                                same_row_d = false;
                                active_run_d = true;
                            }
                        }
                        if active_run_d {
                            if run_d.end == rel_x {
                                if run_d.match_top_right(&face_d) {
                                    run_d.pull(solid_staging_buffer, &face_d, rel_x, rel_y, rel_z);
                                    active_run_d = false;
                                }
                                else {
                                    run_d.pull_partial(solid_staging_buffer, &face_d, rel_x, rel_y, rel_z);
                                    same_row_d = true;
                                }
                            }
                            else {
                                let next_block_pos = block_pos + BlockPos::new(0, 0, 1);
                                let (next_face_d, next_face_u) = self.get_face_pair::<{Down}>(next_block_pos);
                                let compare = BlockFace::should_cull_pair(next_face_d, next_face_u);

                                if compare.0 || !Run::match_faces(face_d, next_face_d) {
                                    run_d.pull_partial(solid_staging_buffer, &face_d, rel_x, rel_y, rel_z);
                                    active_run_d = false;
                                }
                            }
                            break 'down
                        }
                        // */
                        // */
                        run_d = &mut row_d[rel_x as usize];
                        same_row_d = true;
                        active_run_d = true;
                        run_d.begin::<{Down}>(solid_staging_buffer, &face_d, block_pos, rel_x, row_id);
                    }
                    'up: {
                        // break 'up;

                        for face in self.get_opposing_extra_face::<{Down}>(block_pos) {
                            Run::add_face::<{Up}>(solid_staging_buffer, &face, block_pos);
                        }

                        if compare.1 {
                            active_run_u = false;
                            break 'up
                        }
                        // /*
                        if active_run_u && same_row_u {
                            if run_u.match_right(&face_u) {
                                run_u.merge_face(solid_staging_buffer, &face_u);
                                break 'up
                            } else {
                                active_run_u = false;
                            }
                        }
                        // /* 
                        if !active_run_u {
                            run_u = &mut row_u[rel_x as usize];
                            if run_u.row + 1 == row_id && run_u.match_top_left(&face_u) {
                                same_row_u = false;
                                active_run_u = true;
                            }
                        }

                        if active_run_u {
                            if run_u.end == rel_x {
                                if run_u.match_top_right(&face_u) {
                                    run_u.pull(solid_staging_buffer, &face_u, rel_x, rel_y, rel_z);
                                    active_run_u = false;
                                }
                                else {
                                    run_u.pull_partial(solid_staging_buffer, &face_u, rel_x, rel_y, rel_z);
                                    same_row_u = true;
                                }
                            }
                            else {
                                let next_block_pos = block_pos + BlockPos::new(0, 0, 1);
                                let (next_face_d, next_face_u) = self.get_face_pair::<{Down}>(next_block_pos);
                                let compare = BlockFace::should_cull_pair(next_face_d, next_face_u);

                                if compare.1 || !Run::match_faces(face_u, next_face_u) {
                                    run_u.pull_partial(solid_staging_buffer, &face_u, rel_x, rel_y, rel_z);
                                    active_run_u = false;
                                }
                            }
                            break 'up
                        }
                        // */
                        // */
                        run_u = &mut row_u[rel_x as usize];
                        active_run_u = true;
                        same_row_u = true;
                        run_u.begin::<{Up}>(solid_staging_buffer, &face_u, block_pos, rel_x, row_id);
                    }
                }
                (active_run_d, active_run_u) = (false, false); row_id += 1;
            }
            row_id += 16;
        }
    }
    
    pub fn mesh_south_north_no_merge(&mut self, solid_staging_buffer: &mut StagingBuffer) {
        for x in 0..16_u8 {
            for y in 0..16_u8 {
                for z in 0..16_u8 {
                    let block_pos = BlockPos::new(x, y, z);


                    let face_s = self.get_face::<{South}>(block_pos);
                    let face_n = self.get_opposing_face::<{South}>(block_pos);

                    let compare = face_s.as_u32() + 0x10101010 - face_n.as_u32();
                    if compare == 0x10101010 { continue; }
                    
                    let offset = Section::INDICES_ZYX[block_pos.index] as u64;
                    
                    if compare < 0x10101010 {
                        Run::add_face::<{South}>(solid_staging_buffer, face_s, block_pos);
                    }
                    if compare > 0x10101010 {
                        Run::add_face::<{North}>(solid_staging_buffer, face_n, block_pos);
                    }
                    
                    for face in self.get_extra_face::<{South}>(block_pos) {
                        Run::add_face::<{South}>(solid_staging_buffer, &face, block_pos);
                    }
                    for face in self.get_opposing_extra_face::<{South}>(block_pos) {
                        Run::add_face::<{North}>(solid_staging_buffer, &face, block_pos);
                    }
                }
            }
        }
    }
    pub fn mesh_west_east_no_merge(&mut self, solid_staging_buffer: &mut StagingBuffer) {
        for x in 0..16_u8 {
            for y in 0..16_u8 {
                for z in 0..16_u8 {
                    let block_pos = BlockPos::new(x, y, z);

                    let face_w = self.get_face::<{West}>(block_pos);
                    let face_e = self.get_opposing_face::<{West}>(block_pos);

                    let compare = face_w.as_u32() + 0x10101010 - face_e.as_u32();
                    if compare == 0x10101010 { continue; }
                    
                    let offset = Section::INDICES_XYZ[block_pos.index] as u64;
                    
                    if compare < 0x10101010 {
                        Run::add_face::<{West}>(solid_staging_buffer, face_w, block_pos);
                    }
                    if compare > 0x10101010 {
                        Run::add_face::<{East}>(solid_staging_buffer, face_e, block_pos);
                    }

                    for face in self.get_extra_face::<{West}>(block_pos) {
                        Run::add_face::<{West}>(solid_staging_buffer, &face, block_pos);
                    }

                    for face in self.get_opposing_extra_face::<{West}>(block_pos) {
                        Run::add_face::<{East}>(solid_staging_buffer, &face, block_pos);
                    }
                }
            }
        }
    }
    pub fn mesh_down_up_no_merge(&mut self, solid_staging_buffer: &mut StagingBuffer) {
        for x in 0..16_u8 {
            for y in 0..16_u8 {
                for z in 0..16_u8 {
                    let block_pos = BlockPos::new(x, y, z);

                    let face_d = self.get_face::<{Down}>(block_pos);
                    let face_u = self.get_opposing_face::<{Down}>(block_pos);

                    let compare = face_d.as_u32() + 0x10101010 - face_u.as_u32();
                    if compare == 0x10101010 { continue; }

                    let offset = Section::INDICES_YXZ[block_pos.index] as u64;

                    if compare < 0x10101010 {
                        Run::add_face::<{Down}>(solid_staging_buffer, face_d, block_pos);
                    }
                    if compare > 0x10101010 {
                        Run::add_face::<{Up}>(solid_staging_buffer, face_u, block_pos);
                    }

                    for face in self.get_extra_face::<{Down}>(block_pos) {
                        Run::add_face::<{Down}>(solid_staging_buffer, &face, block_pos);
                    }
                
                    for face in self.get_opposing_extra_face::<{Down}>(block_pos) {
                        Run::add_face::<{Up}>(solid_staging_buffer, &face, block_pos);
                    }
                }
            }
        }
    }

    pub fn mesh_unaligned(&mut self, solid_staging_buffer: &mut StagingBuffer) {
        for x in 0..16 {
            for y in 0..16 {
                for z in 0..16 {
                    let pos = BlockPos::new(x, y, z);
                    for face in self.get_block(pos).model.unaligned {
                        Run::add_face::<{Diagonal}>(solid_staging_buffer, face, pos);
                    }
                }
            }
        }
    }

    pub fn mesh_translucent(&mut self, transparent_solid_staging_buffer: &mut StagingBuffer) {
        for x in 0..16_u8 {
            for y in 0..16_u8 {
                for z in 0..16_u8 {
                    let block_pos = BlockPos::new(x, y, z);

                    let block = self.get_block(block_pos);
                    let opposing_block_south = self.get_opposing_block::<{South}>(block_pos);
                    let opposing_block_west = self.get_opposing_block::<{West}>(block_pos);
                    let opposing_block_down = self.get_opposing_block::<{Down}>(block_pos);

                    'south: for face in block.model.transparent_south {
                        for opposing_face in opposing_block_south.model.transparent_north {
                            if face.culled_by::<{South}>(opposing_face) { continue 'south; }
                        }
                        Run::add_face::<{South}>(transparent_solid_staging_buffer, &face, block_pos);
                    }
                    'west: for face in block.model.transparent_west {
                        for opposing_face in opposing_block_west.model.transparent_east {
                            if face.culled_by::<{West}>(opposing_face) { continue 'west; }
                        }
                        Run::add_face::<{West}>(transparent_solid_staging_buffer, &face, block_pos);
                    }
                    'down: for face in block.model.transparent_down {
                        for opposing_face in opposing_block_down.model.transparent_up {
                            if face.culled_by::<{Down}>(opposing_face) { continue 'down; }
                        }
                        Run::add_face::<{Down}>(transparent_solid_staging_buffer, &face, block_pos);
                    }
                    'north: for face in opposing_block_south.model.transparent_north {
                        for opposing_face in block.model.transparent_south {
                            if face.culled_by::<{North}>(opposing_face) { continue 'north; }
                        }
                        Run::add_face::<{North}>(transparent_solid_staging_buffer, &face, block_pos);
                    }
                    'east: for face in opposing_block_west.model.transparent_east {
                        for opposing_face in block.model.transparent_west {
                            if face.culled_by::<{East}>(opposing_face) { continue 'east; }
                        }
                        Run::add_face::<{East}>(transparent_solid_staging_buffer, &face, block_pos);
                    }
                    'up: for face in opposing_block_down.model.transparent_up {
                        for opposing_face in block.model.transparent_down {
                            if face.culled_by::<{Up}>(opposing_face) { continue 'up; }
                        }
                        Run::add_face::<{Up}>(transparent_solid_staging_buffer, &face, block_pos);
                    }
                }
            }
        }
    }

    pub fn mesh_geometry(&mut self, solid_staging_buffer: &mut StagingBuffer, translucent_staging_buffer: &mut StagingBuffer, geometry_buffer_allocator: &mut BufferAllocator) { unsafe {
        self.mesh_translucent(translucent_staging_buffer);

        translucent_staging_buffer.format_quads();
        self.translucent_quad_count = translucent_staging_buffer.idx as u32 / 8;

        self.mesh_south_north(solid_staging_buffer);
        self.mesh_west_east(solid_staging_buffer);
        self.mesh_down_up(solid_staging_buffer);
        self.mesh_unaligned(solid_staging_buffer);

        solid_staging_buffer.format_quads();
        self.solid_quad_count = solid_staging_buffer.idx as u32 / 8;
        
        self.solid_segment = geometry_buffer_allocator.alloc(solid_staging_buffer.idx + 4 * mem::size_of::<u32>());
        self.translucent_segment = geometry_buffer_allocator.alloc(translucent_staging_buffer.idx + 4 * mem::size_of::<u32>());
    } }

    pub fn mesh_light(&mut self, solid_staging_buffer: &mut StagingBuffer, geometry_buffer_allocator: &mut BufferAllocator, light_staging_buffer: &mut StagingBuffer, light_buffer_allocator: &mut BufferAllocator) { unsafe {
        const BYTES_PER_QUAD: usize = 8;

        // bytes reserved to index a light chunk for each quad
        // need 1 u32 per quad
        let reserved_bytes = solid_staging_buffer.idx / BYTES_PER_QUAD * mem::size_of::<u32>();
        light_staging_buffer.idx = reserved_bytes;

        for (i, quad) in solid_staging_buffer.iter().map(|quad| mem::transmute::<&[u8; 8], &GpuQuad>(quad)).enumerate() {
            // insert the index offset of the light section
            light_staging_buffer.set_u32(i * mem::size_of::<u32>(), (light_staging_buffer.idx / mem::size_of::<u32>()) as u32);
            
            match quad.normal {
                South => {
                    let start_x = quad.x / 16;
                    let end_x = (quad.x + quad.width) / 16;
                    
                    let start_y = quad.y / 16;
                    let end_y = (quad.y + quad.height) / 16;
                    
                    let z = (quad.z + 1) / 16;
                    
                    for x in start_x..=end_x {
                        for y in start_y..=end_y {
                            light_staging_buffer.put_u32(self.get_face_light::<{South}>(BlockPos::new(x, y, z)) as u32);
                        }
                    }
                }
                North => {
                    let start_x = quad.x / 16;
                    let end_x = (quad.x + quad.width) / 16;
    
                    let start_y = quad.y / 16;
                    let end_y = (quad.y + quad.height) / 16;

                    // if this from the neighboring section, z wraps to 15, which is the only way for it to be 15
                    let z = (quad.z - 16) / 16;
                    
                    for x in start_x..=end_x {
                        for y in start_y..=end_y {
                            if z == 15 {
                                if let Some(south) = self.get_neighbor::<{South}>() {
                                    light_staging_buffer.put_u32(south.get_face_light::<{North}>(BlockPos::new(x, y, z)) as u32);
                                }
                                else {
                                    light_staging_buffer.put_u32(0);
                                }
                            }
                            light_staging_buffer.put_u32(self.get_face_light::<{North}>(BlockPos::new(x, y, z)) as u32);
                        }
                    }
                }
                West => {
                    let x = quad.x / 16;
    
                    let start_y = quad.y / 16;
                    let end_y = (quad.y + quad.height) / 16;
    
                    let start_z = quad.z / 16;
                    let end_z = (quad.z + quad.width) / 16;
                    
                    for y in start_y..=end_y {
                        for z in start_z..=end_z {
                            light_staging_buffer.put_u32(self.get_face_light::<{West}>(BlockPos::new(x, y, z)) as u32);
                        }
                    }
                }
                East => {
                    // if this from the neighboring section, x wraps to 15, which is the only way for it to be 15
                    let x = (quad.x - 16) / 16;
    
                    let start_y = quad.y / 16;
                    let end_y = (quad.y + quad.height) / 16;
    
                    let start_z = quad.z / 16;
                    let end_z = (quad.z + quad.width) / 16;
                    
                    for y in start_y..=end_y {
                        for z in start_z..=end_z {
                            if x == 15 {
                                if let Some(west) = self.get_neighbor::<{West}>() {
                                    light_staging_buffer.put_u32(west.get_face_light::<{East}>(BlockPos::new(x, y, z)) as u32);
                                }
                                else {
                                    light_staging_buffer.put_u32(0);
                                }
                            }
                            light_staging_buffer.put_u32(self.get_face_light::<{East}>(BlockPos::new(x, y, z)) as u32);
                        }
                    }
                }
                Down => {
                    let start_x = quad.x / 16;
                    let end_x = (quad.x + quad.height) / 16;
    
                    let y = quad.y / 16;
    
                    let start_z = quad.z / 16;
                    let end_z = (quad.z + quad.width) / 16;
                    
                    for x in start_x..=end_x {
                        for z in start_z..=end_z {
                            light_staging_buffer.put_u32(self.get_face_light::<{Down}>(BlockPos::new(x, y, z)) as u32);
                        }
                    }
                }
                Up => {
                    let start_x = quad.x / 16;
                    let end_x = (quad.x + quad.height) / 16;

                    // if this from the neighboring section, y wraps to 15, which is the only way for it to be 15
                    let y = (quad.y - 16) / 16;
    
                    let start_z = quad.z / 16;
                    let end_z = (quad.z + quad.width) / 16;
                    
                    for x in start_x..=end_x {
                        for z in start_z..=end_z {
                            if y == 15 {
                                if let Some(down) = self.get_neighbor::<{Down}>() {
                                    light_staging_buffer.put_u32(down.get_face_light::<{Up}>(BlockPos::new(x, y, z)) as u32);
                                }
                                else {
                                    light_staging_buffer.put_u32(0);
                                }
                            }
                            light_staging_buffer.put_u32(self.get_face_light::<{Up}>(BlockPos::new(x, y, z)) as u32);
                        }
                    }
                }
                _ => {
                    let x = quad.x / 16;
                    
                    let y = quad.y / 16;
                    
                    let z = (quad.z + 1) / 16;

                    light_staging_buffer.put_u32(self.get_face_light::<{South}>(BlockPos::new(x, y, z)) as u32);

                }
            }
        }
        
        self.light_segment = light_buffer_allocator.alloc(light_staging_buffer.idx);
    } }

    pub fn mesh(&mut self, solid_staging_buffer: &mut StagingBuffer, translucent_staging_buffer: &mut StagingBuffer, geometry_buffer_allocator: &mut BufferAllocator, light_staging_buffer: &mut StagingBuffer, light_buffer_allocator: &mut BufferAllocator) {
        geometry_buffer_allocator.free(self.solid_segment.take());
        geometry_buffer_allocator.free(self.translucent_segment.take());
        light_buffer_allocator.free(self.light_segment.take());

        self.mesh_geometry(solid_staging_buffer, translucent_staging_buffer, geometry_buffer_allocator);
        self.mesh_light(solid_staging_buffer, geometry_buffer_allocator, light_staging_buffer, light_buffer_allocator);
        if let (Some(geometry_segment), Some(light_segment)) = (&self.solid_segment, &self.light_segment) {
            geometry_buffer_allocator.upload_offset(geometry_segment, &[self.section_pos.x, self.section_pos.y, self.section_pos.z], 3 * mem::size_of::<u32>(), 0);
            geometry_buffer_allocator.upload_offset(geometry_segment, &solid_staging_buffer.buffer.0.as_slice(), solid_staging_buffer.idx, 4 * mem::size_of::<u32>());
            geometry_buffer_allocator.upload_offset(geometry_segment, &[(light_segment.offset as usize / mem::size_of::<u32>()) as u32], mem::size_of::<u32>(), 3 * mem::size_of::<u32>());

            light_buffer_allocator.upload(light_segment, light_staging_buffer.buffer.0.as_slice(), light_staging_buffer.idx);
        }
        if let (Some(translucent_segment)) = (&self.solid_segment) {
            geometry_buffer_allocator.upload_offset(translucent_segment, &[self.section_pos.x, self.section_pos.y, self.section_pos.z], 3 * mem::size_of::<u32>(), 0);
            geometry_buffer_allocator.upload_offset(translucent_segment, &translucent_staging_buffer.buffer.0.as_slice(), solid_staging_buffer.idx, 4 * mem::size_of::<u32>());
            geometry_buffer_allocator.upload_offset(translucent_segment, &[0], mem::size_of::<u32>(), 3 * mem::size_of::<u32>());
            println!("{} {:?}", translucent_segment.length.get(), geometry_buffer_allocator.device_buffer.get_sub_data::<u8>(translucent_segment.offset as isize, translucent_segment.length.get() as isize));
        }
        solid_staging_buffer.reset();
        light_staging_buffer.reset();
        translucent_staging_buffer.reset();
    }

    pub fn get_bounding_box(&self, camera: &Camera) -> BoundingBox<f32> {
        return BoundingBox {
            min: Vector3 {
                x: (self.section_pos.x * 256) as f32 - camera.frustum_pos.x,
                y: (self.section_pos.y * 256) as f32 - camera.frustum_pos.y,
                z: (self.section_pos.z * 256) as f32 - camera.frustum_pos.z,
            },
            max: Vector3 {
                x: (self.section_pos.x * 256) as f32 - camera.frustum_pos.x + 256.0,
                y: (self.section_pos.y * 256) as f32 - camera.frustum_pos.y + 256.0,
                z: (self.section_pos.z * 256) as f32 - camera.frustum_pos.z + 256.0,
            }
        };
    }

    pub fn get_bounding_sphere(&self, camera: &Camera) -> Sphere<f32> {
        return Sphere {
            center: Vector3 {
                x: (self.section_pos.x * 256) as f32 - camera.frustum_pos.x + 128.0,
                y: (self.section_pos.y * 256) as f32 - camera.frustum_pos.y + 128.0,
                z: (self.section_pos.z * 256) as f32 - camera.frustum_pos.z + 128.0,
            },
            radius: 221.702503369
        };
    }

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