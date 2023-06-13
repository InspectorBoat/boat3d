use std::ops::{IndexMut, Index, Add};
use crate::{block::blockface::BlockFace, world::chunk::Chunk};
#[repr(C, align(8))]
pub struct ByteBuffer {
    pub ind: usize,
    pub arr: Box<[u8; 262144]>
}
impl ByteBuffer {
    pub fn put(&mut self, val: u8) {
        // self.arr[self.ind] = val;
        unsafe {
            *self.arr.get_unchecked_mut(self.ind) = val;
        }
        self.ind += 1;
    }
    pub fn put_u32(&mut self, val: u32) {
        unsafe {
            let loc = self.arr.as_mut_ptr().byte_add(self.ind) as *mut u32;
            *loc = val;
        }
        self.ind += 4;
    }
    pub fn put_u64(&mut self, val: u64) {
        unsafe {
            let loc = self.arr.as_mut_ptr().byte_add(self.ind) as *mut u64;
            *loc = val;
        }
        self.ind += 8;
    }
    pub fn get_u32<T: Into<usize>>(&self, pos: T) -> u32 {
        unsafe {
            let loc = self.arr.as_ptr().byte_add(pos.into()) as *mut u32;
            return *loc
        }
    }
    pub fn get_u64<T: Into<usize>>(&self, pos: T) -> u64 {
        unsafe {
            let loc = self.arr.as_ptr().byte_add(pos.into()) as *mut u64;
            return *loc
        }
    }
    pub fn new() -> ByteBuffer {
        return ByteBuffer {
            ind: 0,
            arr: Box::new([0; 262144])
        }
    }
    pub fn put_face(&mut self, face: &BlockFace, pos: usize) {
        unsafe {
            let loc = self.arr.as_mut_ptr().byte_add(self.ind) as *mut u64;
            *loc = face.as_u64() + Chunk::INDICES_ZYX[pos] as u64;
        }
        self.ind += 8;
    }
    
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
    pub fn format_quads(&mut self) {
        for i in (0..self.ind).step_by(8) {
            let face = &mut self.arr[i..i+8];
         // let ure = (arr[0]);
         // let ven = (arr[1]);
            let dep = (face[2] & 0xf0) | (0x0 + (face[4] & 0x0f));
            let nor = (face[5]);
            let wid = (face[3] & 0xf0) | (0xf - (face[2] & 0x0f) - (face[0] & 0x0f));
            let hei = (face[4] & 0xf0) | (0xf - (face[3] & 0x0f) - (face[1] & 0x0f));
         // let tex = (arr[6], arr[7]);
         // arr[0] = ure;
         // arr[1] = ven;
            face[2] = dep;
            face[3] = nor;
            face[4] = wid;
            face[5] = hei;
            // arr[6] = tex.0;
            // arr[7] = tex.1;
        }
    }
    pub fn reset(&mut self) {
        self.ind = 0;
    }
}

impl<T: Into<usize>> Index<T> for ByteBuffer {
    type Output = u8;

    fn index(&self, index: T) -> &Self::Output {
        // return &self.arr[index.into()];
        return unsafe { self.arr.get_unchecked(index.into()) }
    }
}

impl<T: Into<usize>> IndexMut<T> for ByteBuffer {
    fn index_mut(&mut self, index: T) -> &mut Self::Output {
        // return &mut self.arr[index.into()];
        return unsafe { self.arr.get_unchecked_mut(index.into()) }
    }
}