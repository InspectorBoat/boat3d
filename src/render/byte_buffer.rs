use std::{ops::{IndexMut, Index}, mem::{self, ManuallyDrop}};
use crate::{block::{blockface::BlockFace, normal::Normal::*}, world::{section::Section, blockpos::BlockPos}, render::{buffer_quad::BufferQuad, gpu_quad::GpuQuad}, gl_util::{buffer::Buffer, gl_wrapper}};
#[repr(C, align(8))]
#[derive(Debug)]
pub struct StagingBuffer {
    pub idx: usize,
    pub buffer: Box<RawBuffer>,
}

#[repr(C, align(8))]
#[derive(Debug)]
pub struct RawBuffer(pub [u8; 262144]);

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
    pub fn put_i32(&mut self, val: i32) { unsafe {
        let loc = self.buffer.0.as_mut_ptr().byte_add(self.idx) as *mut i32;
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
    pub fn new() -> StagingBuffer { unsafe {
        return StagingBuffer {
            idx: 0,
            buffer: Box::new(RawBuffer([0; 262144]))
        };
    } }
    pub fn put_face(&mut self, face: &BlockFace, block_pos: BlockPos) { unsafe {
        let loc = self.buffer.0.as_mut_ptr().byte_add(self.idx) as *mut u64;

        *loc = face.as_u64() + ((block_pos.x() << 4) | (block_pos.y() << 12) | (block_pos.z() << 20)) as u64;
        self.idx += 8;
    } }
    pub fn format_quads_relative(&mut self) { unsafe {
        for buffer_quad in self.iter_mut().map(|quad| mem::transmute::<&mut [u8; 8], &mut BufferQuad>(quad)) {
            let mut gpu_quad = GpuQuad {
                x: buffer_quad.rel_x(),
                y: buffer_quad.rel_y(),
                z: buffer_quad.rel_z(),
                normal: buffer_quad.normal(),
                width: buffer_quad.width(),
                height: buffer_quad.height(),
                texture: buffer_quad.texture(),
            };
            match gpu_quad.normal {
                North | South => {
                    (gpu_quad.x, gpu_quad.y, gpu_quad.z) = (gpu_quad.x, gpu_quad.y, gpu_quad.z);
                }
                East | West => {
                    (gpu_quad.x, gpu_quad.y, gpu_quad.z) = (gpu_quad.z, gpu_quad.y, gpu_quad.x);
                }
                Down | Up => {
                    (gpu_quad.x, gpu_quad.y, gpu_quad.z) = (gpu_quad.y, gpu_quad.z, gpu_quad.x);
                }
                _ => {}
            }

            *buffer_quad = mem::transmute::<GpuQuad, BufferQuad>(gpu_quad);
        }
    } }
    pub fn format_quads_absolute(&mut self) { unsafe {
        for buffer_quad in self.iter_mut().map(|quad| mem::transmute::<&mut [u8; 8], &mut BufferQuad>(quad)) {
            let mut left = buffer_quad.left();
            let mut bottom = buffer_quad.bottom();
            let mut depth = buffer_quad.depth();

            match buffer_quad.normal() {
                North | South => {
                    (left, bottom, depth) = (left, bottom, depth);
                }
                East | West => {
                    (left, bottom, depth) = (depth, bottom, left);
                }
                Down | Up => {
                    (left, bottom, depth) = (bottom, depth, left);
                }
                _ => {}
            }

            let gpu_quad = GpuQuad {
                x: buffer_quad.block_x() * 16 + left,
                y: buffer_quad.block_y() * 16 + bottom,
                z: buffer_quad.block_z() * 16 + depth,
                normal: buffer_quad.normal(),
                width: buffer_quad.width(),
                height: buffer_quad.height(),
                texture: buffer_quad.texture(),
            };

            if buffer_quad.normal() == Up {
                println!("{buffer_quad:?} {gpu_quad:?}");
            }

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
        return self.buffer.0.get_unchecked(index.try_into().unwrap_unchecked());
    } }
}

impl<T: TryInto<usize>> IndexMut<T> for StagingBuffer {
    fn index_mut(&mut self, index: T) -> &mut Self::Output { unsafe {
        return self.buffer.0.get_unchecked_mut(index.try_into().unwrap_unchecked());
    } }
}