use std::{ops::{IndexMut, Index, Add}, hint::{unreachable_unchecked, black_box}, mem};
use crate::{block::blockface::{BlockFace, Normal, GpuQuad}, world::chunk::Chunk};
#[repr(C, align(8))]
pub struct StagingBuffer {
    pub index: usize,
    pub buffer: Box<Buffer>,
}

#[repr(C, align(8))]
pub struct Buffer(pub [u8; 262144]);

impl StagingBuffer {
    pub fn put(&mut self, val: u8) { unsafe {
        *self.buffer.0.get_unchecked_mut(self.index) = val;
        self.index += 1;
    } }
    pub fn put_u16(&mut self, val: u16) { unsafe {
        let loc = self.buffer.0.as_mut_ptr().byte_add(self.index) as *mut u16;
        *loc = val;
        self.index += 2;
    } }
    pub fn put_u32(&mut self, val: u32) { unsafe {
        let loc = self.buffer.0.as_mut_ptr().byte_add(self.index) as *mut u32;
        *loc = val;
        self.index += 4;
    } }
    pub fn put_u64(&mut self, val: u64) { unsafe {
        let loc = self.buffer.0.as_mut_ptr().byte_add(self.index) as *mut u64;
        *loc = val;
        self.index += 8;
    } }
    pub fn set_u32(&mut self, index: usize, val: u32) { unsafe {
        let loc = self.buffer.0.as_mut_ptr().byte_add(index) as *mut u32;
        *loc = val;
    } }
    pub fn get_u64<T: Into<usize>>(&self, pos: T) -> u64 { unsafe {
        let loc = self.buffer.0.as_ptr().byte_add(pos.into()) as *mut u64;
        return *loc;
    } }
    pub fn new() -> StagingBuffer {
        return StagingBuffer {
            index: 0,
            buffer: Box::new(Buffer([0; 262144])),
        };
    }
    pub fn put_face(&mut self, face: &BlockFace, pos: usize) { unsafe {
        let loc = self.buffer.0.as_mut_ptr().byte_add(self.index) as *mut u64;
        *loc = face.as_u64() + Chunk::INDICES_ZYX[pos] as u64;
        self.index += 8;
    } }
    
    /** 
     *   0f0   00f
     *   ---------
     * 0 ure ~ lef
     * 1 ven ~ bot
     * 2 dep ~ rig
     * 3 wid ~ top
     * 4 hei ~ dep
     * 5 nor ~ nor
     * 6 tex ~ tex
     * 7 tex ~ tex
     * 
     * 
     * Converts a quad from left-bottom-right-top format to u-v-d-w-h format
     */
    #[allow(unused_parens)]
    pub fn format_quads(&mut self) { unsafe {
        for face in self.iter_mut() {
            let ure = (face[0]);
            let ven = (face[1]);
            let dep = (face[2] & 0xf0) | (0x0 + (face[4] & 0x0f));
            let nor = (face[5]);
            let wid = (face[3] & 0xf0) | (0xf - (face[2] & 0x0f) - (face[0] & 0x0f));
            let hei = (face[4] & 0xf0) | (0xf - (face[3] & 0x0f) - (face[1] & 0x0f));
            // let tex = (face[6], face[7]);
            // face[0] = ure;
            // face[1] = ven;
            face[2] = dep;
            face[3] = nor;
            face[4] = wid;
            face[5] = hei;
            // face[6] = tex.0;
            // face[7] = tex.1;
        }
    } }
    pub fn reset(&mut self) {
        self.index = 0;
    }
    pub fn iter(&self) -> impl Iterator<Item = &[u8; 8]> { unsafe {
        return self.buffer.0.as_chunks_unchecked::<8>().iter().take(self.index / 8);
    } }
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut [u8; 8]> { unsafe {
        return self.buffer.0.as_chunks_unchecked_mut::<8>().iter_mut().take(self.index / 8);
    } }
}

impl<T: Into<usize>> Index<T> for StagingBuffer {
    type Output = u8;

    fn index(&self, index: T) -> &Self::Output { unsafe {
        // return &self.arr[index.into()];
        return self.buffer.0.get_unchecked(index.into());
    } }
}

impl<T: Into<usize>> IndexMut<T> for StagingBuffer {
    fn index_mut(&mut self, index: T) -> &mut Self::Output { unsafe {
        // return &mut self.arr[index.into()];
        return self.buffer.0.get_unchecked_mut(index.into());
    } }
}