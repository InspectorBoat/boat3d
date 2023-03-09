use std::ops::Deref;
use std::{ops::{IndexMut, Index}, ptr};

pub struct ByteBuffer {
    pub pos: usize,
    pub arr: Box<[u8; 262144]>
}

impl ByteBuffer {
    pub fn put(&mut self, val: u8) {
        // self.arr[self.pos] = val;
        unsafe {
            *self.arr.get_unchecked_mut(self.pos) = val;
        }
        self.pos += 1;
    }
    pub fn put_u64(&mut self, val: u64) {
        unsafe {
            let loc = self.arr.as_mut_ptr().byte_add(self.pos) as *mut u64;
            *loc = val;
        }
        self.pos += 8;
    }
    pub fn new() -> ByteBuffer {
        // let chunks: Vec<u8> = Vec::with_capacity(262144);
        
        // let chunks = chunks.leak() as *mut [u8] as *mut [u8; 262144];

        // let chunks = unsafe { *chunks  };

        return ByteBuffer {
            pos: 0,
            arr: Box::new([0; 262144])
        }
    }
    /**
     * u - left
     * v - bottom
     * d - right
     * w - top
     * h - depth
     * normal
     * texture
     */
    pub fn convert(&mut self) {
        for i in (0..self.pos).step_by(8) {
            let u = self.arr[i];
            let v = self.arr[i + 1];
            let depth =  (self.arr[i + 2] & 0xf0) | (self.arr[i + 4] & 0x0f);
            let normal = self.arr[6];
            let width =  (self.arr[i + 3] & 0xf0) | (15 - (self.arr[i + 2] & 0x0f));
            let height = (self.arr[i + 4] & 0xf0) | (15 - (self.arr[i + 3] & 0x0f));
            self.arr[i + 0] = u;
            self.arr[i + 1] = v;
            self.arr[i + 2] = depth;
            self.arr[i + 3] = normal;
            self.arr[i + 4] = width;
            self.arr[i + 5] = height;
        }
    }
}

impl Index<usize> for ByteBuffer {
    type Output = u8;

    fn index(&self, index: usize) -> &Self::Output {
        // return &self.arr[index];
        return unsafe { self.arr.get_unchecked(index) }
    }
}

impl IndexMut<usize> for ByteBuffer {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        // return &mut self.arr[index];
        return unsafe { self.arr.get_unchecked_mut(index) }
    }
}