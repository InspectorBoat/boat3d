use std::hint;

use crate::{block::{blockface::BlockFace, normal::Normal::{self, *}}, render::{buffer_quad::BufferQuad, byte_buffer::StagingBuffer}};

use super::{section::Section, blockpos::BlockPos};

#[derive(Clone, Debug)]
#[repr(C)]
pub struct Run {
    pub lef: u8,
    pub bot: u8,
    pub rig: u8,
    pub top: u8,
    pub dep: u8,
    
    pub beg: u8,
    pub end: u8,
    
    pub tex: u16,
    
    pub idx: u32,
    
    pub row: u16,
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
        let buffer_quad = &mut *(&raw mut buffer[self.idx as u32] as *mut BufferQuad);
        buffer_quad.block_width_top += 0x10;
        buffer_quad.block_z_right &= 0xf0;
        buffer_quad.block_z_right |= face.rig;

        self.end += 1;
        self.rig = face.rig;
    } }
    /**
     * Pulls the run up after an incomplete merge
     * min_x, min_y, min_z, and texture are already guaranteed to match
     */
    pub fn pull_partial(&mut self, buffer: &mut StagingBuffer, face: &BlockFace, rel_x: u8, rel_y: u8, rel_z: u8, block_pos: BlockPos) { unsafe {
        let idx = buffer.idx as u32;
        // copy the quad over
        buffer.put_u64(buffer.get_u64(self.idx));
        let buffer_quad = &mut *(&raw mut buffer[idx] as *mut BufferQuad);
        buffer_quad.block_x_left = (block_pos.x() << 4) as u8;
        buffer_quad.block_y_bottom = (block_pos.y() << 4) as u8;
        buffer_quad.block_z_right = (block_pos.z() << 4) as u8 | face.rig;
        // buffer_quad.block_y_bottom = rel_y << 4;
        // buffer_quad.block_z_right = (rel_z << 4) | face.rig;
        buffer_quad.block_width_top = ((rel_x - self.beg) << 4) | face.top;
        buffer_quad.block_height_depth &= 0x0f;

        self.idx = idx;
        self.end = rel_x;
        self.row += 1;
    } }
    /**
     * Pulls the run up after a complete merge
     * Only possible change is top
     */
    pub fn pull(&mut self, buffer: &mut StagingBuffer, face: &BlockFace, rel_x: u8, rel_y: u8, rel_z: u8) { unsafe {
        let buffer_quad = &mut *(&raw mut buffer[self.idx] as *mut BufferQuad);
        buffer_quad.block_height_depth += 0x10;
        buffer_quad.block_width_top &= 0xf0;
        buffer_quad.block_width_top |= face.top;

        self.top = face.top;
        self.row += 1;
    } }
    /**
     * Begins a new run
     */
    pub fn begin<const N: Normal>(&mut self, buffer: &mut StagingBuffer, face: &BlockFace, pos: BlockPos, rel_x: u8, row: u16) {
        self.idx = buffer.idx as u32;
        buffer.put_face(face, pos);

        self.lef = face.lef;
        self.bot = face.bot;

        self.rig = face.rig;
        self.top = face.top;

        self.beg = rel_x;
        self.end = rel_x;

        self.tex = face.tex;

        self.row = row;
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
        }
    }
}

impl Default for Run {
    fn default() -> Self {
        return Run::new();
    }
}