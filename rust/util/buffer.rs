use std::{ops::{IndexMut, Index}, ptr};

pub struct ByteBuffer {
    pub pos: usize,
    pub arr: [u8; 65536]
}

impl ByteBuffer {
    pub fn put(&mut self, val: u8) {
        self.arr[self.pos] = val;
        self.pos += 1;
    }
    pub fn put_u64(&mut self, val: u64) {
        unsafe {
            let loc = ptr::addr_of_mut!(self.arr).byte_add(self.pos) as *mut u64;
            *loc = val;
        }
        self.pos += 8;
    }
    pub fn new() -> ByteBuffer {
        return ByteBuffer {
            pos: 0,
            arr: [0; 65536]
        }
    }
}

impl Index<usize> for ByteBuffer {
    type Output = u8;

    fn index(&self, index: usize) -> &Self::Output {
        return &self.arr[index];
    }
}

impl IndexMut<usize> for ByteBuffer {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        return &mut self.arr[index];
    }
}