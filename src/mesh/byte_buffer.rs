use std::{ops::{IndexMut, Index, Add}, hint::{unreachable_unchecked, black_box}, mem};
use crate::{block::blockface::BlockFace, world::section::Section, mesh::{buffer_quad::BufferQuad, gpu_quad::GpuQuad}};
#[repr(C, align(8))]
#[derive(Debug)]
pub struct StagingBuffer {
    pub idx: usize,
    pub buffer: Box<Buffer>,
}

#[repr(C, align(8))]
#[derive(Debug)]
pub struct Buffer(pub [u8; 262144]);

impl StagingBuffer {
    pub fn put(&mut self, val: u8) { unsafe {
        *self.buffer.0.get_unchecked_mut(self.idx) = val;
        self.idx += 1;
    } }
    pub fn put_u16(&mut self, val: u16) { unsafe {
        let loc = self.buffer.0.as_mut_ptr().byte_add(self.idx) as *mut u16;
        *loc = val;
        self.idx += 2;
    } }
    pub fn put_u32(&mut self, val: u32) { unsafe {
        let loc = self.buffer.0.as_mut_ptr().byte_add(self.idx) as *mut u32;
        *loc = val;
        self.idx += 4;
    } }
    pub fn put_u64(&mut self, val: u64) { unsafe {
        let loc = self.buffer.0.as_mut_ptr().byte_add(self.idx) as *mut u64;
        *loc = val;
        self.idx += 8;
    } }
    pub fn set_u32(&mut self, index: usize, val: u32) { unsafe {
        let loc = self.buffer.0.as_mut_ptr().byte_add(index) as *mut u32;
        *loc = val;
    } }
    pub fn get_u64<T: TryInto<usize>>(&self, pos: T) -> u64 { unsafe {
        let loc = self.buffer.0.as_ptr().byte_add(pos.try_into().unwrap_unchecked()) as *mut u64;
        return *loc;
    } }
    pub fn new() -> StagingBuffer {
        return StagingBuffer {
            idx: 0,
            buffer: Box::new(Buffer([0; 262144])),
        };
    }
    pub fn put_face(&mut self, face: &BlockFace, pos: usize) { unsafe {
        let loc = self.buffer.0.as_mut_ptr().byte_add(self.idx) as *mut u64;
        *loc = face.as_u64() + Section::INDICES_ZYX[pos] as u64;
        self.idx += 8;
    } }
    
    pub fn format_quads(&mut self) { unsafe {
        for buffer_quad in self.iter_mut().map(|quad| mem::transmute::<&mut [u8; 8], &mut BufferQuad>(quad)) {
            let gpu_quad = GpuQuad {
                rel_x: buffer_quad.get_rel_x(),
                rel_y: buffer_quad.get_rel_y(),
                rel_z: buffer_quad.get_rel_z(),
                normal: buffer_quad.get_normal(),
                width: buffer_quad.get_width(),
                height: buffer_quad.get_height(),
                texture: buffer_quad.get_texture(),
            };
            *buffer_quad = mem::transmute::<GpuQuad, BufferQuad>(gpu_quad);
        }
    } }
    pub fn reset(&mut self) {
        self.idx = 0;
    }
    pub fn iter(&self) -> impl Iterator<Item = &[u8; 8]> { unsafe {
        return self.buffer.0.as_chunks_unchecked::<8>().iter().take(self.idx / 8);
    } }
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut [u8; 8]> { unsafe {
        return self.buffer.0.as_chunks_unchecked_mut::<8>().iter_mut().take(self.idx / 8);
    } }
}

impl<T: TryInto<usize>> Index<T> for StagingBuffer {
    type Output = u8;

    fn index(&self, index: T) -> &Self::Output { unsafe {
        // return &self.arr[index.into()];
        return self.buffer.0.get_unchecked(index.try_into().unwrap_unchecked());
    } }
}

impl<T: TryInto<usize>> IndexMut<T> for StagingBuffer {
    fn index_mut(&mut self, index: T) -> &mut Self::Output { unsafe {
        // return &mut self.arr[index.into()];
        return self.buffer.0.get_unchecked_mut(index.try_into().unwrap_unchecked());
    } }
}