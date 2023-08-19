// Pasted from burger

use std::{num::NonZeroU32, collections::BTreeSet, marker::ConstParamTy, mem, cmp::Ordering, ffi::c_void};
use crate::render::byte_buffer::StagingBuffer;

use self::SortType::*;
use super::{buffer::Buffer, gl_wrapper};

#[derive(Debug)]
pub struct BufferAllocator {
    pub free_segments_by_offset: BTreeSet<BufferSegment<{Length}>>,
    pub free_segments_by_length: BTreeSet<BufferSegment<{Offset}>>,
    pub device_buffer: Buffer,
    pub used: u32,
}

impl BufferAllocator {
    pub fn new(size: usize) -> BufferAllocator { unsafe {
        let device_buffer = Buffer::create();
        device_buffer.storage(size as isize, gl_wrapper::DYNAMIC_STORAGE_BIT);

        let segment = BufferSegment { offset: 0, length: NonZeroU32::new_unchecked(size as u32) };
        
        return BufferAllocator {
            free_segments_by_offset: BTreeSet::from([segment]),
            free_segments_by_length: BTreeSet::from([segment.into()]),
            device_buffer: device_buffer,
            used: 0
        };
    } }
    pub fn alloc(&mut self, size: usize) -> Option<BufferSegment> { unsafe {
        let size = size as u32;
        if let Some(free_segment) = self.free_segments_by_length.iter().find(|segment| segment.length.get() >= size) {
            self.used += size;
            let free_segment = free_segment.clone();
            self.free_segments_by_length.remove(&free_segment);
            self.free_segments_by_offset.remove(&free_segment.into());
            if free_segment.length.get() == size {
                return Some(free_segment.into());
            } else {
                let new_segment = BufferSegment {
                    length: NonZeroU32::new_unchecked(free_segment.length.get() - size),
                    offset: free_segment.offset + size
                };
                self.free_segments_by_length.insert(new_segment);
                self.free_segments_by_offset.insert(new_segment.into());
                return Some(BufferSegment {
                    length: NonZeroU32::new_unchecked(size),
                    offset: free_segment.offset
                });
            }
        }
        return None;
    } }
    pub fn free(&mut self, segment: Option<BufferSegment>) { unsafe {
        if let Some(mut segment) = segment {
            self.used -= segment.length.get();
            let prev_segment_index: usize;
            if let Some((next_segment_index, next_segment)) = self.free_segments_by_offset.iter().enumerate().find(|(i, next_segment)| next_segment.offset > segment.offset) {
                let next_segment = next_segment.clone();
                prev_segment_index = next_segment_index - 1;
                if next_segment.offset == segment.offset + segment.length.get() {
                    segment.length = NonZeroU32::new_unchecked(segment.length.get() + next_segment.length.get());
                    self.free_segments_by_length.remove(&next_segment.into());
                    self.free_segments_by_offset.remove(&next_segment);
                }
            } else {
                prev_segment_index = self.free_segments_by_offset.len() - 1;
            }

            if let Some(prev_segment) = self.free_segments_by_offset.iter().nth(prev_segment_index) {
                let prev_segment = prev_segment.clone();
                if prev_segment.offset + prev_segment.length.get() == segment.offset {
                    segment.offset = prev_segment.offset;
                    segment.length = NonZeroU32::new_unchecked(prev_segment.length.get() + segment.length.get());
                    self.free_segments_by_length.remove(&prev_segment.into());
                    self.free_segments_by_offset.remove(&prev_segment);
                }
            }
            self.free_segments_by_length.insert(segment.into());
            self.free_segments_by_offset.insert(segment.into());
        }
    } }
    pub fn buffer_sub_data(&mut self, segment: &BufferSegment, length: usize, offset: usize, data: *const c_void) { unsafe {
        if length + offset > segment.length.get() as usize {
            panic!("exceeded allocation size");
        }
        // self.staging_buffer.buffer_sub_data(0, length as isize, data);
        
        // gl_wrapper::CopyNamedBufferSubData(self.staging_buffer.id, self.device_buffer.id, 0, (segment.offset as usize + offset) as isize, length as isize);
        self.device_buffer.buffer_sub_data(segment.offset as isize + offset as isize, length as isize, data);
    } }
}

#[derive(PartialEq, PartialOrd, Eq, Clone, Copy, Debug)]
pub struct BufferSegment<const S: SortType = Unsorted> {
    pub offset: u32,
    pub length: NonZeroU32
}

impl From<BufferSegment<{Offset}>> for BufferSegment<{Length}> {
    fn from(value: BufferSegment<{Offset}>) -> Self { unsafe {
        return mem::transmute(value);
    } }
}
impl From<BufferSegment<{Unsorted}>> for BufferSegment<{Length}> {
    fn from(value: BufferSegment<{Unsorted}>) -> Self { unsafe {
        return mem::transmute(value);
    } }
}
impl From<BufferSegment<{Length}>> for BufferSegment<{Offset}> {
    fn from(value: BufferSegment<{Length}>) -> Self { unsafe {
        return mem::transmute(value);
    } }
}
impl From<BufferSegment<{Unsorted}>> for BufferSegment<{Offset}> {
    fn from(value: BufferSegment<{Unsorted}>) -> Self { unsafe {
        return mem::transmute(value);
    } }
}
impl From<BufferSegment<{Length}>> for BufferSegment<{Unsorted}> {
    fn from(value: BufferSegment<{Length}>) -> Self { unsafe {
        return mem::transmute(value);
    } }
}
impl From<BufferSegment<{Offset}>> for BufferSegment<{Unsorted}> {
    fn from(value: BufferSegment<{Offset}>) -> Self { unsafe {
        return mem::transmute(value);
    } }
}

impl Ord for BufferSegment<{Offset}> {
    fn cmp(&self, other: &BufferSegment<{Offset}>) -> Ordering {
        return self.offset.cmp(&other.offset);
    }
}
impl Ord for BufferSegment<{Length}> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.length == other.length {
            return self.offset.cmp(&other.offset);
        }
        return self.length.cmp(&other.length);
    }
}

#[derive(ConstParamTy, PartialEq, Eq)]
pub enum SortType {
    Length,
    Offset,
    Unsorted
}
/*
GLusize capacityInBytes; // the number of bytes to allocate for the staging buffer

GLuint stagingBuffer; // the staging buffer which we'll write our vertex data into 
glCreateBuffers(1 /* number of buffers */, &stagingBuffer /* output variable */); // create the staging buffer
glNamedBufferStorage(stagingBuffer /* target buffer */, capacityInBytes /* size */, NULL /* pointer to initial data */, GL_MAP_COHERENT_BIT | GL_MAP_PERSISTENT_BIT | GL_MAP_WRITE_BIT | GL_CLIENT_STORAGE_BIT /* storage flags */); // allocate the backing storage for the staging buffer

// a pointer to a buffer of memory (on the CPU) which vertex data can be written directly into
void* stagingMemory = glMapNamedBufferRange(stagingBuffer /* target buffer */, 0 /* offset */, capacityInBytes /* length */, GL_MAP_WRITE_BIT | GL_MAP_PERSISTENT_BIT | GL_MAP_COHERENT_BIT | GL_MAP_INVALIDATE_BUFFER_BIT | GL_MAP_UNSYNCHRONIZED_BIT /* mapped flags */); // map the memory used for our staging buffer into the application address space

// prepare the staging buffer with our vertex data
GLusize numOfBytesWritten = /* application code */; // the number of bytes written into the staging buffer

// once we have written our data, copy it from the staging buffer (CPU) into the device buffer (GPU)
glCopyNamedBufferSubData(stagingBuffer /* source (read) buffer */, gpuBuffer /* destination (write) buffer */, 0 /* read offset */, 0 /* write offset */, numOfBytesWritten /* number of bytes to copy */);
*/