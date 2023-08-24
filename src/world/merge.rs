use std::{hint, arch::x86_64::{_pdep_u64, _pdep_u32}, ptr::NonNull};

use crate::{block::{blockface::BlockFace, normal::Normal::{self, *}}, render::{buffer_quad::BufferQuad, byte_buffer::StagingBuffer}};

use super::{section::Section, blockpos::{BlockPos, RelBlockPos}};

#[derive(Clone, Debug)]
#[repr(C)]
pub struct Run {
    pub left: u8,
    pub bottom: u8,
    pub right: u8,
    pub top: u8,
    pub depth: u8,
    
    pub beginning: u8,
    pub ending: u8,
    
    pub texture: u16,
    
    pub index: u32,
    
    pub row: u8,
    pub layer: u8,
}

impl Run {
    /**
     * Matches the top and bottom right corners of the run with the top and bottom left corners of the face
     * Used to immediately merge a face
     */
    pub fn match_right(&self, face: &BlockFace) -> bool { unsafe {
        return (
            self.texture == face.tex &&
            self.right == 0 &&
            face.lef == 0 &&
            self.bottom == face.bot &&
            self.top == face.top
        );
    } }
    /**
     * Matches the top left corner of the run with the bottom left corner of the face
     * Used to begin a merge
     */
    pub fn match_top_left(&self, face: &BlockFace) -> bool { unsafe {
        return (
            self.texture == face.tex &&
            self.top == 0 &&
            face.bot == 0 &&
            self.left == face.lef
        );
    } }
    /**
     * Matches the top right corner of the run with the bottom right corner of the face
     * Used to finalize a merge
     */
    pub fn match_top_right(&self, face: &BlockFace) -> bool { unsafe {
        return (
            face.bot == 0 &&
            self.right == face.rig
        );
    } }
    /**
     * Merges a face into the run horizontally
     * Extends the run's end position and updates the end x
     * End y is already guaranteed to match
     */
    pub fn merge_face(&mut self, buffer: *mut StagingBuffer, face: &BlockFace) { unsafe {
        let buffer_quad = &mut *(&raw mut (*buffer)[self.index as u32] as *mut BufferQuad);
        buffer_quad.block_width_top += 0x10;
        buffer_quad.block_z_right &= 0xf0;
        buffer_quad.block_z_right |= face.rig;

        self.ending += 1;
        self.right = face.rig;
    } }
    /**
     * Pulls the run up after an incomplete merge
     * min_x, min_y, min_z, and texture are already guaranteed to match
     */
    pub fn pull_partial(&mut self, buffer: *mut StagingBuffer, face: &BlockFace, rel_block_pos: RelBlockPos) { unsafe {
        let idx = (*buffer).idx as u32;
        // copy the quad over
        (*buffer).put_u64((*buffer).get_u64(self.index));
        let buffer_quad = &mut *(&raw mut (*buffer)[idx] as *mut BufferQuad);
        buffer_quad.block_y_bottom = rel_block_pos.rel_y << 4;
        buffer_quad.block_z_right = (rel_block_pos.rel_z << 4) | face.rig;
        buffer_quad.block_width_top = ((rel_block_pos.rel_x - self.beginning) << 4) | face.top;
        buffer_quad.block_height_depth &= 0x0f;

        self.index = idx;
        self.ending = rel_block_pos.rel_x;
        self.row += 1;
    } }
    /**
     * Pulls the run up after a complete merge
     * Only possible change is top
     */
    pub fn pull(&mut self, buffer: *mut StagingBuffer, face: &BlockFace) { unsafe {
        let buffer_quad = &mut *(&raw mut (*buffer)[self.index] as *mut BufferQuad);
        buffer_quad.block_height_depth += 0x10;
        buffer_quad.block_width_top &= 0xf0;
        buffer_quad.block_width_top |= face.top;

        self.top = face.top;
        self.row += 1;
    } }
    /**
     * Begins a new run
     */
    pub fn begin(&mut self, normal: Normal, buffer: *mut StagingBuffer, face: &BlockFace, pos: BlockPos, rel_block_pos: RelBlockPos) { unsafe {
        self.index = (*buffer).idx as u32;
        let offset: u64;
        match normal {
            North | South => {
                offset = Section::INDICES_ZYX[pos.index] as u64;
            }
            East | West => { 
                offset = Section::INDICES_XYZ[pos.index] as u64;
            }
            Down | Up => {
                offset = Section::INDICES_YXZ[pos.index] as u64;
            }
            _ => {
                unsafe { panic!(); }
            }
        }
        (*buffer).put_u64(face.as_u64() + offset);

        self.left = face.lef;
        self.bottom = face.bot;

        self.right = face.rig;
        self.top = face.top;

        self.beginning = rel_block_pos.rel_x;
        self.ending = rel_block_pos.rel_x;

        self.texture = face.tex;

        self.row = rel_block_pos.rel_y;
        self.layer = rel_block_pos.rel_z;
    } }
    
    /**
     * Directly adds a face
     */
    pub fn add_face<const N: Normal>(buffer: *mut StagingBuffer, face: &BlockFace, pos: BlockPos) { unsafe {
        let offset: u32;
        match N {
            North | South => {
                offset = Section::INDICES_ZYX[pos.index];
            }
            East | West => {
                offset = Section::INDICES_XYZ[pos.index];
            }
            Down | Up => {
                offset = Section::INDICES_YXZ[pos.index];
            }
            _ => {
                offset = Section::INDICES_ZYX[pos.index];
            }
        }
        (*buffer).put_u64(face.as_u64() + offset as u64);
    } }
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
            left: 0,
            bottom: 0,
            right: 0,
            top: 0,
            beginning: 0,
            ending: 0,
            depth: 0,
            texture: 0,
            index: 0,
            row: u8::MAX,
            layer: u8::MAX,
        }
    }
}

impl Default for Run {
    fn default() -> Self {
        return Run::new();
    }
}

#[derive(Clone, Debug)]
#[repr(C)]
pub struct Merger {
    pub row: [Run; 16],
    pub run: usize,
    pub staging_buffer: NonNull<StagingBuffer>,
}

impl Merger {
    pub fn new(staging_buffer: &mut &mut StagingBuffer) -> Merger { unsafe {
        return Merger {
            row: Default::default(),
            run: usize::MAX,
            staging_buffer: NonNull::new_unchecked(&raw mut **staging_buffer),
        }
    } }
    #[inline]
    pub fn stop_merge(&mut self) {
        self.run = usize::MAX;
    }
    #[inline]
    pub fn try_begin_merge(&mut self, rel_block_pos: RelBlockPos, face: &BlockFace) {
        if self.run == usize::MAX {
            if self.row[rel_block_pos.rel_x as usize].row + 1 == rel_block_pos.rel_y &&
                self.row[rel_block_pos.rel_x as usize].layer == rel_block_pos.rel_z &&
                self.row[rel_block_pos.rel_x as usize].match_top_left(face)
            {
                self.run = rel_block_pos.rel_x as usize;
            }
        }
    }
    #[inline]
    pub fn merge_same_row(&mut self, rel_block_pos: RelBlockPos, face: &BlockFace) -> bool { unsafe {
        if self.run != usize::MAX && self.row[self.run].row == rel_block_pos.rel_y {
            if self.row[self.run].match_right(face) {
                self.row[self.run].merge_face(self.staging_buffer.as_ptr(), face);
                return true;
            } else {
                self.run = usize::MAX;
            }
        }
        return false;
    } }
    #[inline]
    pub fn end_merge(&mut self, normal: Normal, mut rel_block_pos: RelBlockPos, face: &BlockFace, block_pos: BlockPos) { unsafe {
        if normal == South || normal == East || normal == Up {
            rel_block_pos.rel_z -= 1;
        }
        if self.row[self.run].match_top_right(face) {
            self.row[self.run].pull(self.staging_buffer.as_ptr(), face);
            self.run = usize::MAX;
        }
        else {
            self.row[self.run].pull_partial(self.staging_buffer.as_ptr(), face, rel_block_pos);
        }
    } }
    #[inline]
    pub fn continue_merge(&mut self, normal: Normal, section: &Section, mut rel_block_pos: RelBlockPos, face: &BlockFace, next_block_pos: BlockPos) { unsafe {
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
            _ => { panic!(); }
        }
        if next_face_a.culled_by(next_face_b, normal) || !Run::match_faces(face, next_face_a) {
            if normal == South || normal == East || normal == Up {
                rel_block_pos.rel_z -= 1;
            }
            self.row[self.run].pull_partial(self.staging_buffer.as_ptr(), face, rel_block_pos);
            self.run = usize::MAX;
        }
    } }    
    #[inline]
    pub fn begin(&mut self, normal: Normal, face: &BlockFace, block_pos: BlockPos, rel_block_pos: RelBlockPos) { unsafe {
        self.run = rel_block_pos.rel_x as usize;
        self.row[self.run].begin(normal, self.staging_buffer.as_ptr(), face, block_pos, rel_block_pos);
    } }
}