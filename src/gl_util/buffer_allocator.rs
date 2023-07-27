use std::num::NonZeroUsize;

use super::buffer::Buffer;

#[derive(Debug)]
pub struct BufferPoolAllocator<const S: usize, const P: usize> {
    pub buffer: Buffer,
    pub staging_buffer: Buffer,
    pub pages: Box<[bool; S]>,
    pub furthest: usize,
}

impl <const S: usize, const P: usize> BufferPoolAllocator<S, P> {
    pub fn new() -> BufferPoolAllocator<S, P> { unsafe {
        let buffer = Buffer::create();
        buffer.storage((P * S) as isize, gl::DYNAMIC_STORAGE_BIT);
        let staging_buffer = Buffer::create();
        staging_buffer.storage(1024 * 1024, gl::DYNAMIC_STORAGE_BIT);
        return BufferPoolAllocator {
            buffer: buffer,
            staging_buffer: staging_buffer,
            pages: Box::<[bool; S]>::new_zeroed().assume_init(),
            furthest: 0
        }
    } }
    // size given in bytes
    pub fn allocate(&mut self, size: usize) -> Option<Page<P>> { unsafe {
        if size == 0 { return None; }
        let size = size.div_ceil(P);

        let mut run = 0;
        let mut start = 0;
        // hack to make meshing not take O(n^2)
        for i in self.furthest..S {
            if !self.pages[i] {
                if run == 0 { start = i; }
                run += 1;
            }
            if run == size {
                for j in start..(start + size) {
                    self.pages[j] = true;
                }
                if start + size > self.furthest {
                    self.furthest = start + size;
                }
                return Some(Page {
                    start: start,
                    size: NonZeroUsize::new_unchecked(size),
                });
            }
        }
        return None;
    } }
    pub fn deallocate(&mut self, page: Option<Page<P>>) {
        if let Some(page) = page {
            for i in page.start..(page.start + page.size.get()) {
                self.pages[i] = false;
            }
        }
    }

    pub fn upload<T>(&mut self, page: &Page<P>, data: &[T], length: usize) { unsafe {
        if length > page.size.get() * P {
            panic!("exceeded allocation size");
        }
        self.staging_buffer.upload_slice(data, 0, length as isize);
        
        gl::CopyNamedBufferSubData(self.staging_buffer.id, self.buffer.id, 0, (page.start * P) as isize, length as isize);
        // self.buffer.upload_slice(data, (page.start * P) as isize, length);
    } }

    pub fn upload_offset<T>(&mut self, page: &Page<P>, data: &[T], length: usize, offset: usize) { unsafe {
        if length + offset > page.size.get() * P {
            panic!("exceeded allocation size");
        }
        self.staging_buffer.upload_slice(data, 0, length as isize);
        
        gl::CopyNamedBufferSubData(self.staging_buffer.id, self.buffer.id, 0, (page.start * P + offset) as isize, length as isize);
        // self.buffer.upload_slice(data, (page.start * P) as isize, length);
    } }

    pub const fn block_size(&self) -> usize {
        return P;
    }
}

#[derive(Debug, Clone)]
pub struct Page<const P: usize> {
    pub start: usize,
    pub size: NonZeroUsize,
}

impl <const P: usize> Page<P> {
    // returns block size in bytes
    pub const fn block_size(&self) -> usize {
        return P;
    }
}