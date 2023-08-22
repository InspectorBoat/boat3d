use std::ffi::c_void;
use std::hint;
use std::hint::unreachable_unchecked;
use std::mem;
use std::ptr::NonNull;
use cgmath::Vector3;

use crate::block::blockstate::BLOCKS;
use crate::block::normal::Normal::{self, *};
use crate::cull::bounding_box::{SimdBoundingBox, BoundingBox};
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
    // whether the chunk's block store has been initialized
    pub has_blocks: bool,
    // whether the chunk's light store has been initialized
    pub has_light: bool,
    // number of rectangular block faces in a section
    pub solid_quad_count: u32,
    pub solid_segment: Option<BufferSegment>,
    pub solid_light_segment: Option<BufferSegment>,
    
    pub trans_quad_count: u32,
    pub trans_segment: Option<BufferSegment>,
    pub trans_light_segment: Option<BufferSegment>,
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
        self.has_blocks = true;
        if block_id != self.blocks[block_pos.index] {
            self.dirty = true;
            self.blocks[block_pos.index] = block_id;
            if block_pos.x() == 0 && let Some(west) = self.get_neighbor(East) {
                west.dirty = true;
            }
            if block_pos.x() == 15 && let Some(east) = self.get_neighbor(West) {
                east.dirty = true;
            }
            if block_pos.z() == 0 && let Some(south) = self.get_neighbor(North) {
                south.dirty = true;
            }
            if block_pos.z() == 15 && let Some(north) = self.get_neighbor(South) {
                north.dirty = true;
            }
            if block_pos.y() == 0 && let Some(down) = self.get_neighbor(Down) {
                down.dirty = true;
            }
            if block_pos.y() == 15 && let Some(up) = self.get_neighbor(Up) {
                up.dirty = true;
            }
        }
    }
    pub fn set_blocks(&mut self, blocks: [u16; 4096]) {
        self.has_blocks = true;
        self.blocks = blocks;
        if let Some(west) = self.get_neighbor(East) {
            west.dirty = true;
        }
        if let Some(east) = self.get_neighbor(West) {
            east.dirty = true;
        }
        if let Some(south) = self.get_neighbor(North) {
            south.dirty = true;
        }
        if let Some(north) = self.get_neighbor(South) {
            north.dirty = true;
        }
        if let Some(down) = self.get_neighbor(Down) {
            down.dirty = true;
        }
        if let Some(up) = self.get_neighbor(Up) {
            up.dirty = true;
        }
    }
    pub fn set_light(&mut self, block_pos: BlockPos, light: u8) {
        self.has_light = true;
        if light != self.light[block_pos.index] {
            self.dirty = true;
            self.light[block_pos.index] = light;
            if block_pos.x() == 0 && let Some(west) = self.get_neighbor(East) {
                west.dirty = true;
            }
            if block_pos.x() == 15 && let Some(east) = self.get_neighbor(West) {
                east.dirty = true;
            }
            if block_pos.z() == 0 && let Some(south) = self.get_neighbor(North) {
                south.dirty = true;
            }
            if block_pos.z() == 15 && let Some(north) = self.get_neighbor(South) {
                north.dirty = true;
            }
            if block_pos.y() == 0 && let Some(down) = self.get_neighbor(Down) {
                down.dirty = true;
            }
            if block_pos.y() == 15 && let Some(up) = self.get_neighbor(Up) {
                up.dirty = true;
            }
        }
    }
    pub fn set_lights(&mut self, light: [u8; 4096]) {
        self.has_light = true;
        self.light = light;
        if let Some(west) = self.get_neighbor(East) {
            west.dirty = true;
        }
        if let Some(east) = self.get_neighbor(West) {
            east.dirty = true;
        }
        if let Some(south) = self.get_neighbor(North) {
            south.dirty = true;
        }
        if let Some(north) = self.get_neighbor(South) {
            north.dirty = true;
        }
        if let Some(down) = self.get_neighbor(Down) {
            down.dirty = true;
        }
        if let Some(up) = self.get_neighbor(Up) {
            up.dirty = true;
        }
    }

    pub fn can_mesh(&self) -> bool {
        if let Some(north) = self.get_neighbor(North) && let Some(south) = self.get_neighbor(South)
            && let Some(east) = self.get_neighbor(East) && let Some(west) = self.get_neighbor(West)
            // && let Some(up) = self.get_neighbor(Up) && let Some(down) = self.get_neighbor(Down)
        {
            return true;
            // return (north.has_blocks && north.has_light) && (south.has_blocks && south.has_light)
            //     && (east.has_blocks && east.has_light) && (west.has_blocks && west.has_light)
                // && (up.has_blocks && north.has_light) && (down.has_blocks && down.has_light);
        }
        return false;
    }

    pub fn get_neighbor(&self, normal: Normal) -> Option<&mut Section> { unsafe {
        match normal {
            South => {
                return mem::transmute(self.neighbors.south);
            }
            North => {
                return mem::transmute(self.neighbors.north);
            }
            West => {
                return mem::transmute(self.neighbors.west);
            }
            East => {
                return mem::transmute(self.neighbors.east);
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
    pub fn get_offset_block(&self, block_pos: BlockPos, normal: Normal) -> &BlockState { unsafe {
        match normal {
            North => {
                if block_pos.z() == 0 {
                    return self.get_neighbor(North).map_or(&BLOCKS[0], |section| section.get_block(block_pos.set_z(15)));
                }
                return &self.get_block(block_pos - BlockPos::new(0, 0, 1));
            }
            South => {
                if block_pos.z() == 15 {
                    return self.get_neighbor(South).map_or(&BLOCKS[0], |section| section.get_block(block_pos.set_z(0)));
                }
                return &self.get_block(block_pos + BlockPos::new(0, 0, 1));
            }
            West => {
                if block_pos.x() == 0 {
                    return self.get_neighbor(West).map_or(&BLOCKS[0], |section| section.get_block(block_pos.set_x(15)));
                }
                return &self.get_block(block_pos - BlockPos::new(1, 0, 0));
            }
            East => {
                if block_pos.x() == 15 {
                    return self.get_neighbor(East).map_or(&BLOCKS[0], |section| section.get_block(block_pos.set_x(0)));
                }
                return &self.get_block(block_pos + BlockPos::new(1, 0, 0));
            }
            Down => {
                if block_pos.y() == 0 {
                    return self.get_neighbor(Down).map_or(&BLOCKS[0], |section| section.get_block(block_pos.set_y(15)));
                }
                return &self.get_block(block_pos - BlockPos::new(0, 1, 0));
            }
            Up => {
                if block_pos.y() == 15 {
                    return self.get_neighbor(Up).map_or(&BLOCKS[0], |section| section.get_block(block_pos.set_y(0)));
                }
                return &self.get_block(block_pos + BlockPos::new(0, 1, 0));
            }
            _ => unsafe { hint::unreachable_unchecked(); }
        }
    } }

    pub fn get_light(&self, block_pos: BlockPos) -> u8 {
        return self.light[block_pos.index];
    }
    pub fn get_offset_light(&self, block_pos: BlockPos, normal: Normal) -> u8 { unsafe {
        match normal {
            North => {
                if block_pos.z() == 0 {
                    return self.get_neighbor(North).map_or_else(|| 8, |section| section.get_light(block_pos.set_z(15)));
                }
                return self.get_light(block_pos - BlockPos::new(0, 0, 1));
            }
            South => {
                if block_pos.z() == 15 {
                    return self.get_neighbor(South).map_or_else(|| 8, |section| section.get_light(block_pos.set_z(0)));
                }
                return self.get_light(block_pos + BlockPos::new(0, 0, 1));
            }
            West => {
                if block_pos.x() == 0 {
                    return self.get_neighbor(West).map_or_else(|| 8, |section| section.get_light(block_pos.set_x(15)));
                }
                return self.get_light(block_pos - BlockPos::new(1, 0, 0));
            }
            East => {
                if block_pos.x() == 15 {
                    return self.get_neighbor(East).map_or_else(|| 8, |section| section.get_light(block_pos.set_x(0)));
                }
                return self.get_light(block_pos + BlockPos::new(1, 0, 0));
            }
            Down => {
                if block_pos.y() == 0 {
                    return self.get_neighbor(Down).map_or_else(|| 8, |section| section.get_light(block_pos.set_y(15)));
                }
                return self.get_light(block_pos - BlockPos::new(0, 1, 0));
            }
            Up => {
                if block_pos.y() == 15 {
                    return self.get_neighbor(Up).map_or_else(|| 8, |section| section.get_light(block_pos.set_y(0)));
                }
                return self.get_light(block_pos + BlockPos::new(0, 1, 0));
            }
            _ => unsafe { hint::unreachable_unchecked(); }
        }
    } }

    pub fn has_extra_face<const N: Normal>(&self, block_pos: BlockPos) -> bool { unsafe {
        match N {
            North => { return self.get_block(block_pos).model.extra_north.len() > 0; }
            East => { return self.get_block(block_pos).model.extra_east.len() > 0; }
            Down => { return self.get_block(block_pos).model.extra_down.len() > 0; }
            _ => { hint::unreachable_unchecked(); }
        }
    } }
    pub fn has_opposing_extra_face<const N: Normal>(&self, block_pos: BlockPos) -> bool { unsafe {
        match N {
            North => { return self.get_offset_block(block_pos, N).model.extra_south.len() > 0; }
            East => { return self.get_offset_block(block_pos, N).model.extra_west.len() > 0; }
            Down => { return self.get_offset_block(block_pos, N).model.extra_up.len() > 0; }
            _ => { hint::unreachable_unchecked(); }
        }
    } }

    pub fn get_extra_face<const N: Normal>(&self, block_pos: BlockPos) -> &[BlockFace] { unsafe {
        match N {
            North => { return self.get_block(block_pos).model.extra_north; }
            East => { return self.get_block(block_pos).model.extra_east; }
            Down => { return self.get_block(block_pos).model.extra_down; }
            South => { return self.get_block(block_pos).model.extra_south; }
            West => { return self.get_block(block_pos).model.extra_west; }
            Up => { return self.get_block(block_pos).model.extra_up; }
            _ => { unreachable_unchecked(); }
        }
    } }
    pub fn get_opposing_extra_face<const N: Normal>(&self, block_pos: BlockPos) -> &[BlockFace] { unsafe {
        match N {
            North => { return self.get_offset_block(block_pos, N).model.extra_south; }
            East => { return self.get_offset_block(block_pos, N).model.extra_west; }
            Down => { return self.get_offset_block(block_pos, N).model.extra_up; }
            South => { return self.get_offset_block(block_pos, N).model.extra_north; }
            West => { return self.get_offset_block(block_pos, N).model.extra_east; }
            Up => { return self.get_offset_block(block_pos, N).model.extra_down; }
            _ => { unreachable_unchecked(); }
        }
    } }

    pub fn make_terrain(&mut self, noise: &Vec<f32>) { unsafe {
        self.has_blocks = true;
        self.has_light = true;
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
                    1
                },
                // val if val < 0.51 => {
                    // 4
                // },
                _ => {
                    0
                }
            };
            // *light = rand::random::<u8>() & 0xf;
            *light = (*noise_val * 64.0 - 32.0) as u8 & 0xf;
        } } }
    } }
    pub fn make_terrain_debug(&mut self) {
        self.has_blocks = true;
        self.has_light = true;
        let mut i = 0;
        for x in 0..16 {
            for y in 0..16 {
                for z in 0..16 {
                    let pos = BlockPos::new(x, y, z);
                    if x == 15 || y == 15 || z == 15 {
                        self.blocks[pos.index] = 1;
                    }
                    if y == 0 && z == 0 && (x >= 2 && x <= 13 || x == 0){
                        self.light[pos.index] = 15;
                    }
                    i += 1;
                }
                i += 1;
            }
            i += 1;
        }
    }

    pub fn set_pos(&mut self, section_pos: Vector3<i32>) {
        self.section_pos = section_pos;
    }
    
    pub fn mesh_north_south(&mut self, solid_staging_buffer: &mut StagingBuffer) {
        let mut row_n: [Run; 16] = Default::default();
        let mut run_n: &mut Run = &mut row_n[0];
        let mut active_run_n: bool = false;
        let mut same_row_n: bool = false;
        
        let mut row_s: [Run; 16] = Default::default();
        let mut run_s: &mut Run = &mut row_s[0];
        let mut active_run_s: bool = false;
        let mut same_row_s: bool = false;
        
        let mut row_id: u16 = 0;
        
        // rel_x, rel_y, rel_z = x, y, z
        for rel_z in 0..16_u8 {
            for rel_y in 0..16_u8 {
                for rel_x in 0..16_u8 {
                    let block_pos = BlockPos::new(rel_x, rel_y, rel_z);

                    let north_face = self.get_block(block_pos).model.get_face(North);
                    let south_face = self.get_offset_block(block_pos, North).model.get_face(South);

                    let compare = BlockFace::should_cull_pair(north_face, south_face);
                    
                    'north: {
                        for face in self.get_extra_face::<{North}>(block_pos) {
                            Run::add_face::<{North}>(solid_staging_buffer, face, block_pos);
                        }
            
                        if compare.0 {
                            active_run_n = false;
                            break 'north
                        }
                        // /*
                        if active_run_n && same_row_n {
                            if run_n.match_right(north_face) {
                                run_n.merge_face(solid_staging_buffer, north_face);
                                break 'north
                            } else {
                                active_run_n = false;
                            }
                        }
                        // /*
                        if !active_run_n {
                            run_n = &mut row_n[rel_x as usize];
                            if run_n.row + 1 == row_id && run_n.match_top_left(north_face) {
                                same_row_n = false;
                                active_run_n = true;
                            }
                        }
                        if active_run_n {
                            if run_n.end == rel_x {
                                if run_n.match_top_right(north_face) {
                                    run_n.pull(solid_staging_buffer, north_face, rel_x, rel_y, rel_z);
                                    active_run_n = false;
                                }
                                else {
                                    run_n.pull_partial(solid_staging_buffer, north_face, rel_x, rel_y, rel_z);
                                    same_row_n = true;
                                }
                            }
                            else {
                                let next_block_pos = block_pos + BlockPos::new(1, 0, 0);
                                let next_north_face = self.get_block(next_block_pos).model.get_face(North);
                                let next_south_face = (self.get_offset_block(next_block_pos, North).model.get_face(South));

                                let compare = BlockFace::should_cull_pair(next_north_face, next_south_face);

                                if compare.0 || !Run::match_faces(north_face, next_north_face) {
                                    run_n.pull_partial(solid_staging_buffer, north_face, rel_x, rel_y, rel_z);
                                    active_run_n = false;
                                }
                            }
                            break 'north
                        }
                        // */
                        // */
                        run_n = &mut row_n[rel_x as usize];
                        same_row_n = true;
                        active_run_n = true;
                        run_n.begin::<{North}>(solid_staging_buffer, north_face, block_pos, rel_x, row_id);
                    }
                    'south: {
                        for face in self.get_opposing_extra_face::<{North}>(block_pos) {
                            Run::add_face::<{North}>(solid_staging_buffer, face, block_pos);
                        }

                        if compare.1 {
                            active_run_s = false;
                            break 'south;
                        }
                        // /*
                        if active_run_s && same_row_s {
                            if run_s.match_right(south_face) {
                                run_s.merge_face(solid_staging_buffer, south_face);
                                break 'south;
                            } else {
                                active_run_s = false;
                            }
                        }
                        // /*
                        if !active_run_s {
                            run_s = &mut row_s[rel_x as usize];
                            if run_s.row + 1 == row_id && run_s.match_top_left(south_face) {
                                same_row_s = false;
                                active_run_s = true;
                            }
                        }
                        if active_run_s {
                            if run_s.end == rel_x {
                                if run_s.match_top_right(south_face) {
                                    run_s.pull(solid_staging_buffer, south_face, rel_x, rel_y, rel_z);
                                    active_run_s = false;
                                }
                                else {
                                    run_s.pull_partial(solid_staging_buffer, south_face, rel_x, rel_y, rel_z);
                                    same_row_s = true;
                                }
                            }
                            else {
                                let next_block_pos = block_pos + BlockPos::new(1, 0, 0);
                                let next_north_face = self.get_block(next_block_pos).model.get_face(North);
                                let next_south_face = self.get_offset_block(next_block_pos, North).model.get_face(South);
                                let compare = BlockFace::should_cull_pair(next_north_face, next_south_face);
                                
                                if compare.1 || !Run::match_faces(south_face, next_south_face) {
                                    run_s.pull_partial(solid_staging_buffer, south_face, rel_x, rel_y, rel_z);
                                    active_run_s = false;
                                }
                            }
                            break 'south
                        }
                        // */
                        // */
                        run_s = &mut row_s[rel_x as usize];
                        active_run_s = true;
                        same_row_s = true;
                        run_s.begin::<{South}>(solid_staging_buffer, south_face, block_pos, rel_x, row_id);
                    }
                }
                (active_run_n, active_run_s) = (false, false);
                row_id += 1;
            }
            row_id += 16;
        }
    }
    pub fn mesh_east_west(&mut self, solid_staging_buffer: &mut StagingBuffer) {
        let mut row_w: [Run; 16] = Default::default();
        let mut run_w: &mut Run = &mut row_w[0];
        let mut active_run_w: bool = false;
        let mut same_row_w: bool = false;
        
        let mut row_e: [Run; 16] = Default::default();
        let mut run_e: &mut Run = &mut row_e[0];
        let mut active_run_e: bool = false;
        let mut same_row_e: bool = false;
        
        let mut row_id: u16 = 0;
        
        // rel_x, rel_y, rel_z = z, y, x
        for rel_z in 0..16_u8 {
            for rel_y in 0..16_u8 {
                for rel_x in 0..16_u8 {
                    let block_pos = BlockPos::new(rel_z, rel_y, rel_x);

                    let west_face = self.get_block(block_pos).model.get_face(West);
                    let east_face = self.get_offset_block(block_pos, West).model.get_face(East);

                    let compare = BlockFace::should_cull_pair(west_face, east_face);

                    'west: {
                        for face in self.get_extra_face::<{East}>(block_pos) {
                            Run::add_face::<{East}>(solid_staging_buffer, face, block_pos);
                        }
                        
                        if compare.0 {
                            active_run_w = false;
                            break 'west
                        }

                        // /*
                        if active_run_w && same_row_w {
                            if run_w.match_right(west_face) {
                                run_w.merge_face(solid_staging_buffer, west_face);
                                break 'west
                            } else {
                                active_run_w = false;
                            }
                        }
                        // /*
                        if !active_run_w {
                            run_w = &mut row_w[rel_x as usize];
                            if run_w.row + 1 == row_id && run_w.match_top_left(west_face) {
                                same_row_w = false;
                                active_run_w = true;
                            }
                        }
                        if active_run_w {
                            if run_w.end == rel_x {
                                if run_w.match_top_right(west_face) {
                                    run_w.pull(solid_staging_buffer, west_face, rel_x, rel_y, rel_z);
                                    active_run_w = false;
                                }
                                else {
                                    run_w.pull_partial(solid_staging_buffer, west_face, rel_x, rel_y, rel_z);
                                    same_row_w = true;
                                }
                            }
                            else {
                                let next_block_pos = block_pos + BlockPos::new(0, 0, 1);
                                let next_west_face = self.get_block(next_block_pos).model.get_face(West);
                                let next_east_face = self.get_offset_block(next_block_pos, West).model.get_face(East);
                                let compare = BlockFace::should_cull_pair(next_west_face, next_east_face);

                                if compare.0 || !Run::match_faces(west_face, next_west_face) {
                                    run_w.pull_partial(solid_staging_buffer, west_face, rel_x, rel_y, rel_z);
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
                        run_w.begin::<{East}>(solid_staging_buffer, west_face, block_pos, rel_x, row_id);
                    }
                    'east: {
                        for face in self.get_opposing_extra_face::<{East}>(block_pos) {
                            Run::add_face::<{West}>(solid_staging_buffer, face, block_pos);
                        }
                        if compare.1 {
                            active_run_e = false;
                            break 'east;
                        }
                        // /*
                        if active_run_e && same_row_e {
                            if run_e.match_right(east_face) {
                                run_e.merge_face(solid_staging_buffer, east_face);
                                break 'east;
                            } else {
                                active_run_e = false;
                            }
                        }
                        // /*
                        if !active_run_e {
                            run_e = &mut row_e[rel_x as usize];
                            if run_e.row + 1 == row_id && run_e.match_top_left(east_face) {
                                same_row_e = false;
                                active_run_e = true;
                            }
                        }

                        if active_run_e {
                            // reached end of run
                            if run_e.end == rel_x {
                                if run_e.match_top_right(east_face) {
                                    run_e.pull(solid_staging_buffer, east_face, rel_x, rel_y, rel_z);
                                    active_run_e = false;
                                }
                                else {
                                    run_e.pull_partial(solid_staging_buffer, east_face, rel_x, rel_y, rel_z);
                                    same_row_e = true;
                                }
                            }
                            else {
                                let next_block_pos = block_pos + BlockPos::new(0, 0, 1);
                                let next_west_face = self.get_block(next_block_pos).model.get_face(West);
                                let next_east_face = self.get_offset_block(next_block_pos, West).model.get_face(East);
                                let compare = BlockFace::should_cull_pair(next_west_face, next_east_face);

                                if compare.1 || !Run::match_faces(east_face, next_east_face) {
                                    run_e.pull_partial(solid_staging_buffer, east_face, rel_x, rel_y, rel_z);
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
                        run_e.begin::<{West}>(solid_staging_buffer, east_face, block_pos, rel_x, row_id);
                    }
                }
                (active_run_w, active_run_e) = (false, false); row_id += 1;
            }
            row_id += 16;
        }
    }
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
        
        // rel_x, rel_y, rel_z = z, x, y
        for rel_z in 0..16_u8 {
            for rel_y in 0..16_u8 {
                for rel_x in 0..16_u8 {
                    let block_pos = BlockPos::new(rel_y, rel_z, rel_x);

                    let down_face = self.get_block(block_pos).model.get_face(Down);
                    let up_face = self.get_offset_block(block_pos, Down).model.get_face(Up);

                    let compare = BlockFace::should_cull_pair(down_face, up_face);
                    'down: {
                        for face in self.get_extra_face::<{Down}>(block_pos) {
                            Run::add_face::<{Down}>(solid_staging_buffer, face, block_pos);
                        }
            
                        if compare.0 {
                            active_run_d = false;
                            break 'down
                        }

                        // /*
                        if active_run_d && same_row_d {
                            if run_d.match_right(down_face) {
                                run_d.merge_face(solid_staging_buffer, down_face);
                                break 'down
                            } else {
                                active_run_d = false;
                            }
                        }
                        // /*
                        if !active_run_d {
                            run_d = &mut row_d[rel_x as usize];
                            if run_d.row + 1 == row_id && run_d.match_top_left(down_face) {
                                same_row_d = false;
                                active_run_d = true;
                            }
                        }
                        if active_run_d {
                            if run_d.end == rel_x {
                                if run_d.match_top_right(down_face) {
                                    run_d.pull(solid_staging_buffer, down_face, rel_x, rel_y, rel_z);
                                    active_run_d = false;
                                }
                                else {
                                    run_d.pull_partial(solid_staging_buffer, down_face, rel_x, rel_y, rel_z);
                                    same_row_d = true;
                                }
                            }
                            else {
                                let next_block_pos = block_pos + BlockPos::new(0, 0, 1);
                                let next_down_face = self.get_block(next_block_pos).model.get_face(Down);
                                let next_up_face = self.get_offset_block(next_block_pos, Down).model.get_face(Up);
                                let compare = BlockFace::should_cull_pair(next_down_face, next_up_face);

                                if compare.0 || !Run::match_faces(down_face, next_down_face) {
                                    run_d.pull_partial(solid_staging_buffer, down_face, rel_x, rel_y, rel_z);
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
                        run_d.begin::<{Down}>(solid_staging_buffer, down_face, block_pos, rel_x, row_id);
                    }
                    'up: {
                        // break 'up;

                        for face in self.get_opposing_extra_face::<{Down}>(block_pos) {
                            Run::add_face::<{Up}>(solid_staging_buffer, face, block_pos);
                        }

                        if compare.1 {
                            active_run_u = false;
                            break 'up
                        }
                        // /*
                        if active_run_u && same_row_u {
                            if run_u.match_right(up_face) {
                                run_u.merge_face(solid_staging_buffer, up_face);
                                break 'up
                            } else {
                                active_run_u = false;
                            }
                        }
                        // /* 
                        if !active_run_u {
                            run_u = &mut row_u[rel_x as usize];
                            if run_u.row + 1 == row_id && run_u.match_top_left(up_face) {
                                same_row_u = false;
                                active_run_u = true;
                            }
                        }

                        if active_run_u {
                            if run_u.end == rel_x {
                                if run_u.match_top_right(up_face) {
                                    run_u.pull(solid_staging_buffer, up_face, rel_x, rel_y, rel_z);
                                    active_run_u = false;
                                }
                                else {
                                    run_u.pull_partial(solid_staging_buffer, up_face, rel_x, rel_y, rel_z);
                                    same_row_u = true;
                                }
                            }
                            else {
                                let next_block_pos = block_pos + BlockPos::new(0, 0, 1);
                                let next_down_face = self.get_block(next_block_pos).model.get_face(Down);
                                let next_up_face = self.get_offset_block(next_block_pos, Down).model.get_face(Up);
                                let compare = BlockFace::should_cull_pair(next_down_face, next_up_face);

                                if compare.1 || !Run::match_faces(up_face, next_up_face) {
                                    run_u.pull_partial(solid_staging_buffer, up_face, rel_x, rel_y, rel_z);
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
                        run_u.begin::<{Up}>(solid_staging_buffer, up_face, block_pos, rel_x, row_id);
                    }
                }
                (active_run_d, active_run_u) = (false, false); row_id += 1;
            }
            row_id += 16;
        }
    }
    
    pub fn mesh_north_south_no_merge(&mut self, solid_staging_buffer: &mut StagingBuffer) {
        for z in 0..16_u8 {
            for y in 0..16_u8 {
                for x in 0..16_u8 {
                    let block_pos = BlockPos::new(x, y, z);

                    let block = self.blocks[block_pos.index];

                    let north_face = self.get_block(block_pos).model.get_face(North);
                    let south_face = self.get_offset_block(block_pos, North).model.get_face(South);
                    let compare = BlockFace::should_cull_pair(north_face, south_face);

                    if !compare.0 {
                        solid_staging_buffer.put_face(north_face, block_pos);
                    }
                    if !compare.1  && z != 0 {
                        solid_staging_buffer.put_face(south_face, block_pos - BlockPos::new(0, 0, 1));
                    }
                }
            }
        }
        for y in 0..16_u8 {
            for x in 0..16_u8 {
                let block_pos = BlockPos::new(x, y, 15);

                let north_face = self.get_offset_block(block_pos, South).model.get_face(North);
                let south_face = self.get_block(block_pos).model.get_face(South);
                let compare = BlockFace::should_cull_pair(north_face, south_face);
                
                if !compare.1 {
                    solid_staging_buffer.put_face(south_face, block_pos);
                }
            }
        }
    }
    pub fn mesh_east_west_no_merge(&mut self, solid_staging_buffer: &mut StagingBuffer) {
        for x in 0..16_u8 {
            for y in 0..16_u8 {
                for z in 0..16_u8 {
                    let block_pos = BlockPos::new(x, y, z);

                    let west_face = self.get_block(block_pos).model.get_face(West);
                    let east_face = self.get_offset_block(block_pos, West).model.get_face(East);

                    let compare = BlockFace::should_cull_pair(west_face, east_face);
                    
                    if !compare.0 {
                        solid_staging_buffer.put_face(west_face, block_pos);
                    }
                    if !compare.1 && x != 0 {
                        solid_staging_buffer.put_face(east_face, block_pos - BlockPos::new(1, 0, 0));
                    }
                }
            }
        }
        for y in 0..16_u8 {
            for z in 0..16_u8 {
                let block_pos = BlockPos::new(15, y, z);

                let west_face = self.get_offset_block(block_pos, East).model.get_face(West);
                let east_face = self.get_block(block_pos).model.get_face(East);
                let compare = BlockFace::should_cull_pair(west_face, east_face);
                
                if !compare.1 {
                    solid_staging_buffer.put_face(east_face, block_pos);
                }
            }
        }
    }
    pub fn mesh_down_up_no_merge(&mut self, solid_staging_buffer: &mut StagingBuffer) {
        for x in 0..16_u8 {
            for y in 0..16_u8 {
                for z in 0..16_u8 {
                    let block_pos = BlockPos::new(x, y, z);

                    let down_face = self.get_block(block_pos).model.get_face(Down);
                    let up_face = self.get_offset_block(block_pos, Down).model.get_face(Up);

                    let compare = BlockFace::should_cull_pair(down_face, up_face);

                    let offset = Section::INDICES_YXZ[block_pos.index] as u64;

                    if !compare.0 {
                        solid_staging_buffer.put_face(down_face, block_pos);
                    }
                    if !compare.1 && y != 0 {
                        solid_staging_buffer.put_face(up_face, block_pos - BlockPos::new(0, 1, 0));
                    }
                }
            }
        }
        for x in 0..16_u8 {
            for z in 0..16_u8 {
                let block_pos = BlockPos::new(x, 15, z);

                let down_face = self.get_offset_block(block_pos, Up).model.get_face(Down);
                let up_face = self.get_block(block_pos).model.get_face(Up);
                let compare = BlockFace::should_cull_pair(down_face, up_face);
                
                if !compare.1 {
                    solid_staging_buffer.put_face(up_face, block_pos);
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

    pub fn mesh_trans(&mut self, trans_staging_buffer: &mut StagingBuffer, geometry_buffer_allocator: &mut BufferAllocator) {
        for x in 0..16_u8 {
            for y in 0..16_u8 {
                for z in 0..16_u8 {
                    let block_pos = BlockPos::new(x, y, z);

                    let block = self.get_block(block_pos);
                    let opposing_block_south = self.get_offset_block(block_pos, North);
                    let opposing_block_west = self.get_offset_block(block_pos, East);
                    let opposing_block_down = self.get_offset_block(block_pos, Down);

                    'north: for face in block.model.trans_north {
                        for opposing_face in opposing_block_south.model.trans_south {
                            if face.culled_by::<{North}>(opposing_face) { continue 'north; }
                        }
                        Run::add_face::<{North}>(trans_staging_buffer, &face, block_pos);
                    }
                    'east: for face in block.model.trans_east {
                        for opposing_face in opposing_block_west.model.trans_west {
                            if face.culled_by::<{East}>(opposing_face) { continue 'east; }
                        }
                        Run::add_face::<{East}>(trans_staging_buffer, &face, block_pos);
                    }
                    'down: for face in block.model.trans_down {
                        for opposing_face in opposing_block_down.model.trans_up {
                            if face.culled_by::<{Down}>(opposing_face) { continue 'down; }
                        }
                        Run::add_face::<{Down}>(trans_staging_buffer, &face, block_pos);
                    }
                    'south: for face in opposing_block_south.model.trans_south {
                        for opposing_face in block.model.trans_north {
                            if face.culled_by::<{South}>(opposing_face) { continue 'south; }
                        }
                        Run::add_face::<{South}>(trans_staging_buffer, &face, block_pos);
                    }
                    'west: for face in opposing_block_west.model.trans_west {
                        for opposing_face in block.model.trans_east {
                            if face.culled_by::<{West}>(opposing_face) { continue 'west; }
                        }
                        Run::add_face::<{West}>(trans_staging_buffer, &face, block_pos);
                    }
                    'up: for face in opposing_block_down.model.trans_up {
                        for opposing_face in block.model.trans_down {
                            if face.culled_by::<{Up}>(opposing_face) { continue 'up; }
                        }
                        Run::add_face::<{Up}>(trans_staging_buffer, &face, block_pos);
                    }
                }
            }
        }
        trans_staging_buffer.format_quads_fixed();
        self.trans_quad_count = trans_staging_buffer.idx as u32 / 8;
        self.trans_segment = geometry_buffer_allocator.alloc(trans_staging_buffer.idx + 4 * mem::size_of::<u32>());
    }
    pub fn mesh_solid(&mut self, solid_staging_buffer: &mut StagingBuffer, geometry_buffer_allocator: &mut BufferAllocator) { unsafe {
        // self.mesh_north_south(solid_staging_buffer);
        // self.mesh_east_west(solid_staging_buffer);
        // self.mesh_down_up(solid_staging_buffer);
        self.mesh_north_south_no_merge(solid_staging_buffer);
        self.mesh_east_west_no_merge(solid_staging_buffer);
        self.mesh_down_up_no_merge(solid_staging_buffer);

        self.mesh_unaligned(solid_staging_buffer);

        solid_staging_buffer.format_quads_fixed();
        self.solid_quad_count = solid_staging_buffer.idx as u32 / 8;
        self.solid_segment = geometry_buffer_allocator.alloc(solid_staging_buffer.idx + 4 * mem::size_of::<u32>());
    } }

    pub fn mesh_solid_light(&mut self, solid_staging_buffer: &mut StagingBuffer, light_staging_buffer: &mut StagingBuffer, light_buffer_allocator: &mut BufferAllocator) { unsafe {
        const BYTES_PER_QUAD: usize = 8;

        // bytes reserved to index a light chunk for each quad
        // need 1 u32 per quad
        let reserved_bytes = solid_staging_buffer.idx / BYTES_PER_QUAD * mem::size_of::<u32>();
        light_staging_buffer.idx = reserved_bytes;

        for (i, quad) in solid_staging_buffer.iter().map(|quad| mem::transmute::<&[u8; 8], &GpuQuad>(quad)).enumerate() {
            // insert the index offset of the light section
            light_staging_buffer.set_u32(i * mem::size_of::<u32>(), (light_staging_buffer.idx / mem::size_of::<u32>()) as u32);
            
            match quad.normal {
                North => {
                    let start_x = quad.x / 16;
                    let end_x = (quad.x + quad.width) / 16;
                    
                    let start_y = quad.y / 16;
                    let end_y = (quad.y + quad.height) / 16;
                    
                    let z = quad.z / 16;
                    
                    for y in start_y..=end_y {
                        for x in start_x..=end_x {
                            light_staging_buffer.put_u32(self.get_offset_light(BlockPos::new(x, y, z), North) as u32);
                        }
                    }
                }
                South => {
                    let start_x = quad.x / 16;
                    let end_x = (quad.x + quad.width) / 16;
    
                    let start_y = quad.y / 16;
                    let end_y = (quad.y + quad.height) / 16;

                    let z = quad.z / 16;
                    
                    for y in start_y..=end_y {
                        for x in start_x..=end_x {
                            light_staging_buffer.put_u32(self.get_offset_light(BlockPos::new(x, y, z), South) as u32);
                        }
                    }
                }
                East => {
                    let x = quad.x / 16;
    
                    let start_y = quad.y / 16;
                    let end_y = (quad.y + quad.height) / 16;
    
                    let start_z = quad.z / 16;
                    let end_z = (quad.z + quad.width) / 16;
                    
                    for y in start_y..=end_y {
                        for z in start_z..=end_z {
                            light_staging_buffer.put_u32(self.get_offset_light(BlockPos::new(x, y, z), East) as u32);
                        }
                    }
                }
                West => {
                    // if this from the neighboring section, x wraps to 0, which is the only way for it to be 0
                    let x = quad.x / 16;

                    let start_y = quad.y / 16;
                    let end_y = (quad.y + quad.height) / 16;
    
                    let start_z = quad.z / 16;
                    let end_z = (quad.z + quad.width) / 16;
                    
                    for y in start_y..=end_y {
                        for z in start_z..=end_z {
                            light_staging_buffer.put_u32(self.get_offset_light(BlockPos::new(x, y, z), West) as u32);
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
                            light_staging_buffer.put_u32(self.get_offset_light(BlockPos::new(x, y, z), Down) as u32);
                        }
                    }
                }
                Up => {
                    let start_x = quad.x / 16;
                    let end_x = (quad.x + quad.height) / 16;

                    // if this from the neighboring section, y wraps to 0, which is the only way for it to be 0
                    let y = quad.y / 16;
    
                    let start_z = quad.z / 16;
                    let end_z = (quad.z + quad.width) / 16;
                    
                    for x in start_x..=end_x {
                        for z in start_z..=end_z {
                            light_staging_buffer.put_u32(self.get_offset_light(BlockPos::new(x, y, z), Up) as u32);
                        }
                    }
                }
                _ => {
                    let x = quad.x / 16;
                    let y = quad.y / 16;
                    let z = quad.z / 16;

                    light_staging_buffer.put_u32(self.get_light(BlockPos::new(x, y, z)) as u32);
                }
            }
        }
        
        self.solid_light_segment = light_buffer_allocator.alloc(light_staging_buffer.idx);
    } }
    pub fn mesh_trans_light(&mut self, trans_staging_buffer: &mut StagingBuffer, light_staging_buffer: &mut StagingBuffer, light_buffer_allocator: &mut BufferAllocator) { unsafe {
        const BYTES_PER_QUAD: usize = 8;

        // bytes reserved to index a light chunk for each quad
        // need 1 u32 per quad
        let reserved_bytes = trans_staging_buffer.idx / BYTES_PER_QUAD * mem::size_of::<u32>();
        light_staging_buffer.idx = reserved_bytes;

        for (i, quad) in trans_staging_buffer.iter().map(|quad| mem::transmute::<&[u8; 8], &GpuQuad>(quad)).enumerate() {
            // insert the index offset of the light section
            light_staging_buffer.set_u32(i * mem::size_of::<u32>(), (light_staging_buffer.idx / mem::size_of::<u32>()) as u32);
            
            match quad.normal {
                North => {
                    let start_x = quad.x / 16;
                    let end_x = (quad.x + quad.width) / 16;
                    
                    let start_y = quad.y / 16;
                    let end_y = (quad.y + quad.height) / 16;
                    
                    let z = quad.z / 16;
                    
                    for y in start_y..=end_y {
                        for x in start_x..=end_x {
                            light_staging_buffer.put_u32(self.get_offset_light(BlockPos::new(x, y, z), North) as u32);
                        }
                    }
                }
                South => {
                    let start_x = quad.x / 16;
                    let end_x = (quad.x + quad.width) / 16;
    
                    let start_y = quad.y / 16;
                    let end_y = (quad.y + quad.height) / 16;

                    // if this from the neighboring section, z wraps to 15, which is the only way for it to be 15
                    let z = (quad.z - 16) / 16;
                    
                    for y in start_y..=end_y {
                        for x in start_x..=end_x {
                            light_staging_buffer.put_u32(self.get_offset_light(BlockPos::new(x, y, z), South) as u32);
                        }
                    }
                }
                East => {
                    let x = (quad.x - 16) / 16;
    
                    let start_y = quad.y / 16;
                    let end_y = (quad.y + quad.height) / 16;
    
                    let start_z = quad.z / 16;
                    let end_z = (quad.z + quad.width) / 16;
                    
                    for y in start_y..=end_y {
                        for z in start_z..=end_z {
                            light_staging_buffer.put_u32(self.get_offset_light(BlockPos::new(x, y, z), East) as u32);
                        }
                    }
                }
                West => {
                    // if this from the neighboring section, x wraps to 15, which is the only way for it to be 15
                    let x = quad.x / 16;

                    let start_y = quad.y / 16;
                    let end_y = (quad.y + quad.height) / 16;
    
                    let start_z = quad.z / 16;
                    let end_z = (quad.z + quad.width) / 16;
                    
                    for y in start_y..=end_y {
                        for z in start_z..=end_z {
                            light_staging_buffer.put_u32(self.get_offset_light(BlockPos::new(x, y, z), West) as u32);
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
                            light_staging_buffer.put_u32(self.get_offset_light(BlockPos::new(x, y, z), Down) as u32);
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
                                if let Some(down) = self.get_neighbor(Down) {
                                    light_staging_buffer.put_u32(down.get_offset_light(BlockPos::new(x, y, z), Up) as u32);
                                }
                                else {
                                    light_staging_buffer.put_u32(rand::random::<u32>() & 15);
                                }
                            }
                            light_staging_buffer.put_u32(self.get_offset_light(BlockPos::new(x, y, z), Up) as u32);
                        }
                    }
                }
                _ => {
                    let x = quad.x / 16;
                    let y = quad.y / 16;
                    let z = quad.z / 16;

                    light_staging_buffer.put_u32(self.get_light(BlockPos::new(x, y, z)) as u32);
                }
            }
        }

        
        self.trans_light_segment = light_buffer_allocator.alloc(light_staging_buffer.idx);
    } }

    pub fn mesh(&mut self, solid_staging_buffer: &mut StagingBuffer, trans_staging_buffer: &mut StagingBuffer, geometry_buffer_allocator: &mut BufferAllocator, light_staging_buffer: &mut StagingBuffer, light_buffer_allocator: &mut BufferAllocator) {
        self.dirty = false;
        geometry_buffer_allocator.free(self.solid_segment.take());
        geometry_buffer_allocator.free(self.trans_segment.take());
        light_buffer_allocator.free(self.solid_light_segment.take());
        light_buffer_allocator.free(self.trans_light_segment.take());

        self.mesh_solid(solid_staging_buffer, geometry_buffer_allocator);
        self.mesh_solid_light(solid_staging_buffer, light_staging_buffer, light_buffer_allocator);
        if let (Some(solid_segment), Some(solid_light_segment)) = (&self.solid_segment, &self.solid_light_segment) {
            geometry_buffer_allocator.buffer_sub_data(solid_segment, solid_staging_buffer.idx, 4 * mem::size_of::<u32>(), &raw const solid_staging_buffer.buffer.0 as *const c_void);
            let offset = solid_light_segment.offset / mem::size_of::<u32>() as u32;
            geometry_buffer_allocator.buffer_sub_data(solid_segment, 3 * mem::size_of::<u32>(), 0, &raw const self.section_pos as *const c_void);
            geometry_buffer_allocator.buffer_sub_data(solid_segment, mem::size_of::<u32>(), 3 * mem::size_of::<u32>(), &raw const offset as *const c_void);

            light_buffer_allocator.buffer_sub_data(solid_light_segment, light_staging_buffer.idx, 0, &raw const light_staging_buffer.buffer.0 as *const c_void);
        }
        solid_staging_buffer.reset();
        light_staging_buffer.reset();

        self.mesh_trans(trans_staging_buffer, geometry_buffer_allocator);
        self.mesh_trans_light(trans_staging_buffer, light_staging_buffer, light_buffer_allocator);
        if let (Some(trans_segment), Some(trans_light_segment)) = (&self.trans_segment, &self.trans_light_segment) {
            geometry_buffer_allocator.buffer_sub_data(trans_segment, 3 * mem::size_of::<u32>(), 0, &raw const self.section_pos as *const c_void);
            geometry_buffer_allocator.buffer_sub_data(trans_segment, trans_staging_buffer.idx, 4 * mem::size_of::<u32>(), &raw const trans_staging_buffer.buffer.0 as *const c_void);
            let offset = trans_light_segment.offset / mem::size_of::<u32>() as u32;
            geometry_buffer_allocator.buffer_sub_data(trans_segment, mem::size_of::<u32>(), 3 * mem::size_of::<u32>(), &raw const offset as *const c_void);

            light_buffer_allocator.buffer_sub_data(trans_light_segment, light_staging_buffer.idx, 0, &raw const light_staging_buffer.buffer.0 as *const c_void);
        }
        trans_staging_buffer.reset();
        light_staging_buffer.reset();
    }
    
    pub fn get_local_bounding_box(&self, camera: &Camera) -> BoundingBox {
        return BoundingBox {
            min: Vector3 {
                x: (self.section_pos.x * 16) as f32 - camera.frustum_pos.x,
                y: (self.section_pos.y * 16) as f32 - camera.frustum_pos.y,
                z: (self.section_pos.z * 16) as f32 - camera.frustum_pos.z,
            },
            max: Vector3 {
                x: (self.section_pos.x * 16) as f32 - camera.frustum_pos.x + 16.0,
                y: (self.section_pos.y * 16) as f32 - camera.frustum_pos.y + 16.0,
                z: (self.section_pos.z * 16) as f32 - camera.frustum_pos.z + 16.0,
            }
        };
    }

    pub fn get_bounding_box(&self, camera: &Camera) -> BoundingBox {
        return BoundingBox {
            min: Vector3 {
                x: (self.section_pos.x * 16) as f32,
                y: (self.section_pos.y * 16) as f32,
                z: (self.section_pos.z * 16) as f32,
            },
            max: Vector3 {
                x: (self.section_pos.x * 16) as f32 + 16.0,
                y: (self.section_pos.y * 16) as f32 + 16.0,
                z: (self.section_pos.z * 16) as f32 + 16.0,
            }
        };
    }

    pub fn new() -> Section { unsafe {
        return mem::zeroed();
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

/*
1. Frustum cull sections
2. Render sections based on previous frame's visibility results (sections that were previously visible are not rendered)
3. Rasterize minimum enclosing AABBs of sections not culled by frustum test on current depth buffer
4. Write to visibility results - if any pixel of a section's AABB was drawn, it is visible
5. Render sections that weren't visible last frame but passed rasterization test
*/