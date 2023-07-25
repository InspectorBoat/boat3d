use std::hint;

use crate::{block::blockface::{BlockFace, Normal::{self, *}}, util::byte_buffer::StagingBuffer};

use super::{section::Section, blockpos::BlockPos};

#[derive(Clone, Debug)]
#[allow(dead_code)]
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
    
    pub idx: u16,
    
    pub row: u16,
    pub pad: u16
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
    pub fn pull_partial(&mut self, buffer: &mut StagingBuffer, face: &BlockFace, rel_x: u8, rel_y: u8, rel_z: u8) {
        let ind = buffer.index as u16;
        buffer.put_u64(buffer.get_u64(self.idx));
        
        buffer[ind + 1] = rel_y << 4;
        buffer[ind + 2] = (rel_z << 4) | face.rig;
        buffer[ind + 3] = ((rel_x - self.beg) << 4) | face.top;
        buffer[ind + 4] &= 0x0f;
        self.idx = ind;
        self.end = rel_x;
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
    pub fn begin<const N: Normal>(&mut self, buffer: &mut StagingBuffer, face: &BlockFace, index: BlockPos, u: u8, row: u16) {
        self.idx = buffer.index as u16;
        let offset: u32;
        match N {
            South | North => {
                offset = Section::INDICES_ZYX[index.index];
            }
            West | East => {
                offset = Section::INDICES_XYZ[index.index];
            }
            Down | Up => {
                offset = Section::INDICES_YXZ[index.index];
            }
            _ => {
                unsafe { hint::unreachable_unchecked(); }
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
    pub fn add_face<const N: Normal>(buffer: &mut StagingBuffer, face: &BlockFace, index: BlockPos) { unsafe {
        let offset: u32;
        match N {
            South | North => {
                offset = Section::INDICES_ZYX[index.index];
            }
            West | East => {
                offset = Section::INDICES_XYZ[index.index];
            }
            Down | Up => {
                offset = Section::INDICES_YXZ[index.index];
            }
            _ => { hint::unreachable_unchecked(); }
        }
        buffer.put_u64(face.as_u64() + offset as u64);
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