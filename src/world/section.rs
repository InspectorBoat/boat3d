use std::arch::x86_64::*;
use std::arch::x86_64::*;
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
use super::blockpos::{BlockPos, RelBlockPos};
use super::camera::Camera;
use super::merge::{Run, Merger};
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
            _ => { panic!(); }
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
            _ => unsafe { panic!(); }
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
                    // actually belongs to chunk to the north
                    return self.get_light(block_pos.set_z(0));
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
                    // actually belongs to chunk to the west
                    return self.get_light(block_pos.set_x(0));
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
                    // actually belongs to chunk below
                    return self.get_light(block_pos.set_y(0));
                }
                return self.get_light(block_pos + BlockPos::new(0, 1, 0));
            }
            _ => unsafe { panic!(); }
        }
    } }

    pub fn has_extra_face<const N: Normal>(&self, block_pos: BlockPos) -> bool { unsafe {
        match N {
            North => { return self.get_block(block_pos).model.extra_north.len() > 0; }
            East => { return self.get_block(block_pos).model.extra_east.len() > 0; }
            Down => { return self.get_block(block_pos).model.extra_down.len() > 0; }
            _ => { panic!(); }
        }
    } }
    pub fn has_opposing_extra_face<const N: Normal>(&self, block_pos: BlockPos) -> bool { unsafe {
        match N {
            North => { return self.get_offset_block(block_pos, N).model.extra_south.len() > 0; }
            East => { return self.get_offset_block(block_pos, N).model.extra_west.len() > 0; }
            Down => { return self.get_offset_block(block_pos, N).model.extra_up.len() > 0; }
            _ => { panic!(); }
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
                    if x <= 15 && y <= 15 && z <= 15 {
                        self.blocks[pos.index] = 1;
                    }
                    self.light[pos.index] = rand::random::<u8>() & 15;
                    i += 1;
                }
                i += 1;
            }
            i += 1;
        }
        self.blocks[BlockPos::new(15, 8, 8).index] = 0;
        self.blocks[BlockPos::new(8, 15, 8).index] = 0;
        self.blocks[BlockPos::new(8, 8, 15).index] = 0;
        // self.blocks[BlockPos::new(0, 2, 2).index] = 0;
    }

    pub fn set_pos(&mut self, section_pos: Vector3<i32>) {
        self.section_pos = section_pos;
    }
    
    pub fn mesh_north_south_greedy(&self, mut solid_staging_buffer: &mut StagingBuffer) {
        let mut north_merger = Merger::new(&mut solid_staging_buffer);
        let mut south_merger = Merger::new(&mut solid_staging_buffer);
        
        // rel_x, rel_y, rel_z = x, y, z
        for rel_z in 0..16_u8 {
            for rel_y in 0..16_u8 {
                for rel_x in 0..16_u8 {
                    let block_pos = BlockPos::new(rel_x, rel_y, rel_z);
                    let rel_block_pos = RelBlockPos::new(rel_x, rel_y, rel_z);

                    let north_face = self.get_block(block_pos).model.get_face(North);
                    let south_face = self.get_offset_block(block_pos, North).model.get_face(South);

                    let compare = BlockFace::should_cull_pair(north_face, south_face);
                    
                    'north: {
                        if compare.0 {
                            north_merger.stop_merge();
                            break 'north;
                        }
                        if north_merger.merge_same_row(rel_block_pos, north_face) {
                            break 'north;
                        }
                        north_merger.try_begin_merge(rel_block_pos, north_face);

                        if north_merger.run != usize::MAX {
                            if north_merger.row[north_merger.run].ending == rel_x {
                                north_merger.end_merge(North, rel_block_pos, north_face, block_pos);
                            }
                            else {
                                north_merger.continue_merge(North, self, rel_block_pos, north_face, block_pos + BlockPos::new(1, 0, 0));
                            }
                            break 'north;
                        }
                        north_merger.begin(North, north_face, block_pos, rel_block_pos);
                    }
                    'south: {
                        if compare.1 || rel_z == 0 {
                            south_merger.stop_merge();
                            break 'south;
                        }
                        if south_merger.merge_same_row(rel_block_pos, south_face) {
                            break 'south;
                        }
                        south_merger.try_begin_merge(rel_block_pos, south_face);
                        if south_merger.run != usize::MAX {
                            if south_merger.row[south_merger.run].ending == rel_x {
                                south_merger.end_merge(South, rel_block_pos, south_face, block_pos);
                            }
                            else {
                                south_merger.continue_merge(South, self, rel_block_pos, south_face, block_pos + BlockPos::new(1, 0, 0));
                            }
                            break 'south;
                        }
                        south_merger.begin(South, south_face, block_pos - BlockPos::new(0, 0, 1), rel_block_pos);
                    }
                }
                north_merger.stop_merge();
                south_merger.stop_merge();
            }
        }
        for rel_y in 0..16_u8 {
            for rel_x in 0..16_u8 {
                let block_pos = BlockPos::new(rel_x, rel_y, 15);
                let rel_block_pos = RelBlockPos::new(rel_x, rel_y, 15);

                let south_face = self.get_block(block_pos).model.get_face(South);
                let north_face = self.get_offset_block(block_pos, South).model.get_face(North);

                let cull = south_face.culled_by(north_face, South);

                if cull {
                    south_merger.stop_merge();
                    continue;
                }
                if south_merger.merge_same_row(rel_block_pos, south_face) {
                    continue;
                }
                south_merger.try_begin_merge(rel_block_pos, south_face);
                
                if south_merger.run != usize::MAX {
                    if south_merger.row[south_merger.run].ending == rel_x {
                        south_merger.end_merge(South, rel_block_pos, south_face, block_pos);
                    }
                    else {
                        let next_south_face = self.get_block(block_pos + BlockPos::new(1, 0, 0)).model.get_face(South);
                        let next_north_face = self.get_offset_block(block_pos + BlockPos::new(1, 0, 0), South).model.get_face(North);
                
                        if next_south_face.culled_by(next_north_face, South) || !Run::match_faces(south_face, next_south_face) {
                            south_merger.row[south_merger.run].pull_partial(south_merger.staging_buffer.as_ptr(), south_face, rel_block_pos);
                            south_merger.run = usize::MAX;
                        }
                    }
                    continue;
                }
                south_merger.begin(South, south_face, block_pos, rel_block_pos);
            }
            south_merger.stop_merge();
        }
    }
    pub fn mesh_west_east_greedy(&self, mut solid_staging_buffer: &mut StagingBuffer) {
        let mut west_merger = Merger::new(&mut solid_staging_buffer);
        let mut east_merger = Merger::new(&mut solid_staging_buffer);

        // rel_x, rel_y, rel_z = z, y, x
        for rel_z in 0..16_u8 {
            for rel_y in 0..16_u8 {
                for rel_x in 0..16_u8 {
                    let block_pos = BlockPos::new(rel_z, rel_y, rel_x);
                    let rel_block_pos = RelBlockPos::new(rel_x, rel_y, rel_z);

                    let west_face = self.get_block(block_pos).model.get_face(West);
                    let east_face = self.get_offset_block(block_pos, West).model.get_face(East);

                    let compare = BlockFace::should_cull_pair(west_face, east_face);

                    'west: {
                        if compare.0 {
                            west_merger.stop_merge();
                            break 'west;
                        }
                        if west_merger.merge_same_row(rel_block_pos, west_face) {
                            break 'west;
                        }
                        west_merger.try_begin_merge(rel_block_pos, west_face);
                        if west_merger.run != usize::MAX {
                            if west_merger.row[west_merger.run].ending == rel_x {
                                west_merger.end_merge(West, rel_block_pos, west_face, block_pos);
                            }
                            else {
                                west_merger.continue_merge(West, self, rel_block_pos, west_face, block_pos + BlockPos::new(0, 0, 1));
                            }
                            break 'west;
                        }
                        west_merger.begin(West, west_face, block_pos, rel_block_pos);
                    }
                    'east: {
                        if compare.1 || rel_z == 0 {
                            east_merger.stop_merge();
                            break 'east;
                        }
                        if east_merger.merge_same_row(rel_block_pos, east_face) {
                            break 'east;
                        }
                        east_merger.try_begin_merge(rel_block_pos, east_face);
                        
                        if east_merger.run != usize::MAX {
                            if east_merger.row[east_merger.run].ending == rel_x {
                                east_merger.end_merge(East, rel_block_pos, east_face, block_pos);
                            }
                            else {
                                east_merger.continue_merge(East, self, rel_block_pos, east_face, block_pos + BlockPos::new(0, 0, 1));
                            }
                            break 'east;
                        }
                        east_merger.begin(East, east_face, block_pos - BlockPos::new(1, 0, 0), rel_block_pos);
                    }
                }
                west_merger.stop_merge();
                east_merger.stop_merge();
            }
        }
        for rel_y in 0..16_u8 {
            for rel_x in 0..16_u8 {
                let block_pos = BlockPos::new(15, rel_y, rel_x);
                let rel_block_pos = RelBlockPos::new(rel_x, rel_y, 15);

                let west_face = self.get_offset_block(block_pos, East).model.get_face(West);
                let east_face = self.get_block(block_pos).model.get_face(East);

                let cull = east_face.culled_by(west_face, East);

                if cull {
                    east_merger.stop_merge();
                    continue;
                }
                if east_merger.merge_same_row(rel_block_pos, east_face) {
                    continue;
                }
                east_merger.try_begin_merge(rel_block_pos, east_face);
                
                if east_merger.run != usize::MAX {
                    if east_merger.row[east_merger.run].ending == rel_x {
                        east_merger.end_merge(East, rel_block_pos, east_face, block_pos);
                    }
                    else {
                        let next_west_face = self.get_offset_block(block_pos + BlockPos::new(0, 0, 1), East).model.get_face(West);
                        let next_east_face = self.get_block(block_pos + BlockPos::new(0, 0, 1)).model.get_face(East);

                        if next_east_face.culled_by(west_face, East) || !Run::match_faces(east_face, next_east_face) {
                            east_merger.row[east_merger.run].pull_partial(east_merger.staging_buffer.as_ptr(), east_face, rel_block_pos);
                            east_merger.run = usize::MAX;
                        }
                    }
                    continue;
                }
                east_merger.begin(East, east_face, block_pos, rel_block_pos);
            }
            east_merger.stop_merge();
        }
    }
    pub fn mesh_down_up_greedy(&self, mut solid_staging_buffer: &mut StagingBuffer) {
        let mut down_merger = Merger::new(&mut solid_staging_buffer);
        let mut up_merger = Merger::new(&mut solid_staging_buffer);
        
        // rel_x, rel_y, rel_z = z, x, y
        for rel_z in 0..16_u8 {
            for rel_y in 0..16_u8 {
                for rel_x in 0..16_u8 {
                    let block_pos = BlockPos::new(rel_y, rel_z, rel_x);
                    let rel_block_pos = RelBlockPos::new(rel_x, rel_y, rel_z);

                    let down_face = self.get_block(block_pos).model.get_face(Down);
                    let up_face = self.get_offset_block(block_pos, Down).model.get_face(Up);

                    let compare = BlockFace::should_cull_pair(down_face, up_face);

                    'down: {
                        if compare.0 {
                            down_merger.stop_merge();
                            break 'down;
                        }
                        if down_merger.merge_same_row(rel_block_pos, down_face) {
                            break 'down;
                        }
                        down_merger.try_begin_merge(rel_block_pos, down_face);
                        if down_merger.run != usize::MAX {
                            if down_merger.row[down_merger.run].ending == rel_x {
                                down_merger.end_merge(Down, rel_block_pos, down_face, block_pos);
                            }
                            else {
                                down_merger.continue_merge(Down, self, rel_block_pos, down_face, block_pos + BlockPos::new(0, 0, 1));
                            }
                            break 'down;
                        }
                        down_merger.begin(Down, down_face, block_pos, rel_block_pos);
                    }
                    'up: {
                        if compare.1 || rel_z == 0 {
                            up_merger.stop_merge();
                            break 'up;
                        }
                        if up_merger.merge_same_row(rel_block_pos, up_face) {
                            break 'up;
                        }
                        up_merger.try_begin_merge(rel_block_pos, up_face);
                        
                        if up_merger.run != usize::MAX {
                            if up_merger.row[up_merger.run].ending == rel_x {
                                up_merger.end_merge(Up, rel_block_pos, up_face, block_pos);
                            }
                            else {
                                up_merger.continue_merge(Up, self, rel_block_pos, up_face, block_pos + BlockPos::new(0, 0, 1));
                            }
                            break 'up;
                        }
                        up_merger.begin(Up, up_face, block_pos - BlockPos::new(0, 1, 0), rel_block_pos);
                    }
                }
                down_merger.stop_merge();
                up_merger.stop_merge();
            }
        }
        for rel_y in 0..16_u8 {
            for rel_x in 0..16_u8 {
                let block_pos = BlockPos::new(rel_y, 15, rel_x);
                let rel_block_pos = RelBlockPos::new(rel_x, rel_y, 15);

                let down_face = self.get_offset_block(block_pos, Up).model.get_face(Down);
                let up_face = self.get_block(block_pos).model.get_face(Up);

                let cull = up_face.culled_by(down_face, Up);

                if cull {
                    up_merger.stop_merge();
                    continue;
                }
                if up_merger.merge_same_row(rel_block_pos, up_face) {
                    continue;
                }
                up_merger.try_begin_merge(rel_block_pos, up_face);
                
                if up_merger.run != usize::MAX {
                    if up_merger.row[up_merger.run].ending == rel_x {
                        up_merger.end_merge(East, rel_block_pos, up_face, block_pos);
                    }
                    else {
                        let next_down_face = self.get_offset_block(block_pos + BlockPos::new(0, 0, 1), Up).model.get_face(Down);
                        let next_up_face = self.get_block(block_pos + BlockPos::new(0, 0, 1)).model.get_face(Up);

                        if next_up_face.culled_by(down_face, Up) || !Run::match_faces(up_face, next_up_face) {
                            up_merger.row[up_merger.run].pull_partial(up_merger.staging_buffer.as_ptr(), up_face, rel_block_pos);
                            up_merger.run = usize::MAX;
                        }
                    }
                    continue;
                }
                up_merger.begin(Up, up_face, block_pos, rel_block_pos);
            }
            up_merger.stop_merge();
        }
    }

    pub fn mesh_north_south_no_merge(&mut self, solid_staging_buffer: &mut StagingBuffer) {
        for x in 0..16_u8 {
            for y in 0..16_u8 {
                for z in 0..16_u8 {
                    let block_pos = BlockPos::new(x, y, z);

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
        for x in 0..16_u8 {
            for y in 0..16_u8 {
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
                            if face.culled_by(opposing_face, North) { continue 'north; }
                        }
                        Run::add_face::<{North}>(trans_staging_buffer, &face, block_pos);
                    }
                    'east: for face in block.model.trans_east {
                        for opposing_face in opposing_block_west.model.trans_west {
                            if face.culled_by(opposing_face, East) { continue 'east; }
                        }
                        Run::add_face::<{East}>(trans_staging_buffer, &face, block_pos);
                    }
                    'down: for face in block.model.trans_down {
                        for opposing_face in opposing_block_down.model.trans_up {
                            if face.culled_by(opposing_face, Down) { continue 'down; }
                        }
                        Run::add_face::<{Down}>(trans_staging_buffer, &face, block_pos);
                    }
                    'south: for face in opposing_block_south.model.trans_south {
                        for opposing_face in block.model.trans_north {
                            if face.culled_by(opposing_face, South) { continue 'south; }
                        }
                        Run::add_face::<{South}>(trans_staging_buffer, &face, block_pos);
                    }
                    'west: for face in opposing_block_west.model.trans_west {
                        for opposing_face in block.model.trans_east {
                            if face.culled_by(opposing_face, West) { continue 'west; }
                        }
                        Run::add_face::<{West}>(trans_staging_buffer, &face, block_pos);
                    }
                    'up: for face in opposing_block_down.model.trans_up {
                        for opposing_face in block.model.trans_down {
                            if face.culled_by(opposing_face, Up) { continue 'up; }
                        }
                        Run::add_face::<{Up}>(trans_staging_buffer, &face, block_pos);
                    }
                }
            }
        }
        trans_staging_buffer.format_quads_relative();
        self.trans_quad_count = trans_staging_buffer.idx as u32 / 8;
        self.trans_segment = geometry_buffer_allocator.alloc(trans_staging_buffer.idx + 4 * mem::size_of::<u32>());
    }
    pub fn mesh_solid(&mut self, solid_staging_buffer: &mut StagingBuffer, geometry_buffer_allocator: &mut BufferAllocator) { unsafe {
        self.mesh_north_south_greedy(solid_staging_buffer);
        self.mesh_west_east_greedy(solid_staging_buffer);
        self.mesh_down_up_greedy(solid_staging_buffer);
        // self.mesh_north_south_no_merge(solid_staging_buffer);
        // self.mesh_east_west_no_merge(solid_staging_buffer);
        // self.mesh_down_up_no_merge(solid_staging_buffer);

        self.mesh_unaligned(solid_staging_buffer);
        solid_staging_buffer.format_quads_relative();
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

                    // if this from the neighboring section, z wraps to 0, which is the only way for it to be 0
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
                    let y = (quad.y - 16) / 16;
    
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

#[inline]
pub fn begin(normal: Normal, run: &mut usize, row: &mut [Run; 16], solid_staging_buffer: &mut StagingBuffer, face: &BlockFace, block_pos: BlockPos, rel_block_pos: RelBlockPos) {
    *run = rel_block_pos.rel_x as usize;
    row[*run].begin(normal, solid_staging_buffer, face, block_pos, rel_block_pos);
}

#[inline]
pub fn try_begin_merge(run: &mut usize, row: &mut [Run; 16], rel_block_pos: RelBlockPos, face: &BlockFace) {
    if *run == usize::MAX {
        if row[rel_block_pos.rel_x as usize].row + 1 == rel_block_pos.rel_y && row[rel_block_pos.rel_x as usize].layer == rel_block_pos.rel_z && row[rel_block_pos.rel_x as usize].match_top_left(face) {
            *run = rel_block_pos.rel_x as usize;
        }
    }
}

#[inline]
pub fn merge_same_row(rel_block_pos: RelBlockPos, row: &mut [Run; 16], run: &mut usize, face: &BlockFace, solid_staging_buffer: &mut StagingBuffer) -> bool {
    if *run != usize::MAX && row[*run].row == rel_block_pos.rel_y {
        if row[*run].match_right(face) {
            row[*run].merge_face(solid_staging_buffer, face);
            return true;
        } else {
            *run = usize::MAX;
        }
    }
    return false;
}

#[inline]
pub fn end_merge(normal: Normal, row: &mut [Run; 16], run: &mut usize, mut rel_block_pos: RelBlockPos, face: &BlockFace, solid_staging_buffer: &mut StagingBuffer, block_pos: BlockPos) {
    if normal == South || normal == East || normal == Up {
        rel_block_pos.rel_z -= 1;
    }
    if row[*run].match_top_right(face) {
        row[*run].pull(solid_staging_buffer, face);
        *run = usize::MAX;
    }
    else {
        row[*run].pull_partial(solid_staging_buffer, face, rel_block_pos);
    }
}

#[inline]
pub fn continue_merge(normal: Normal, section: &Section, row: &mut [Run; 16], run: &mut usize, mut rel_block_pos: RelBlockPos, face: &BlockFace, solid_staging_buffer: &mut StagingBuffer, next_block_pos: BlockPos) { unsafe {
    let next_face_a: &BlockFace;
    let next_face_b: &BlockFace;
    match normal {
        North => {
            next_face_a = section.get_block(next_block_pos).model.get_face(North);
            next_face_b = section.get_offset_block(next_block_pos, North).model.get_face(South);
        }
        South => {
            next_face_b = section.get_block(next_block_pos).model.get_face(North);
            next_face_a = section.get_offset_block(next_block_pos, North).model.get_face(South);
        }
        West => {
            next_face_a = section.get_block(next_block_pos).model.get_face(West);
            next_face_b = section.get_offset_block(next_block_pos, West).model.get_face(East);
        }
        East => {
            next_face_b = section.get_block(next_block_pos).model.get_face(West);
            next_face_a = section.get_offset_block(next_block_pos, West).model.get_face(East);
        }
        Down => {
            next_face_a = section.get_block(next_block_pos).model.get_face(Down);
            next_face_b = section.get_offset_block(next_block_pos, Down).model.get_face(Up);
        }
        Up => {
            next_face_b = section.get_block(next_block_pos).model.get_face(Down);
            next_face_a = section.get_offset_block(next_block_pos, Down).model.get_face(Up);
        }
        _ => { hint::unreachable_unchecked(); }
    }

    if next_face_a.culled_by(next_face_b, normal) || !Run::match_faces(face, next_face_a) {
        if normal == South || normal == East || normal == Up {
            rel_block_pos.rel_z -= 1;
        }    
        row[*run].pull_partial(solid_staging_buffer, face, rel_block_pos);
        *run = usize::MAX;
    }
} }

/*
1. Frustum cull sections
2. Render sections based on previous frame's visibility results (sections that were previously visible are not rendered)
3. Rasterize minimum enclosing AABBs of sections not culled by frustum test on current depth buffer
4. Write to visibility results - if any pixel of a section's AABB was drawn, it is visible
5. Render sections that weren't visible last frame but passed rasterization test
*/