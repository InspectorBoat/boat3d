use std::{ffi::c_void, mem};

use super::{gl_wrapper, gl_helper::{PANIC_ON_DROP, LOG_ON_DROP}};

#[derive(Debug)]
pub struct Buffer {
    pub id: u32
}

impl Buffer {
    pub fn create() -> Buffer { unsafe {
        let mut id: u32 = 0;
        gl_wrapper::CreateBuffers(1, &mut id);
        return Buffer {
            id: id
        };
    } }
    pub fn generate() -> Buffer { unsafe {
        let mut id: u32 = 0;
        gl_wrapper::GenBuffers(1, &mut id);
        return Buffer { id };
    } }
    pub fn bind_target(&self, target: u32) { unsafe {
        gl_wrapper::BindBuffer(target, self.id);
    } }
    pub fn bind_indexed_target_base(&self, target: u32, index: u32) { unsafe {
        gl_wrapper::BindBufferBase(target, index, self.id);
    } }
    pub fn bind_indexed_target(&self, target: u32, index: u32, offset: isize, length: isize) { unsafe {
        gl_wrapper::BindBufferRange(target, index, self.id, offset, length);
    } }
    pub fn valid(&self) -> bool { unsafe {
        return gl_wrapper::IsBuffer(self.id) == gl_wrapper::TRUE;
    } }
    pub fn kill(self) { unsafe {
        gl_wrapper::DeleteBuffers(1, &raw const self.id);
        mem::forget(self);
    } } 
    pub fn storage(&self, length: isize, flags: u32) { unsafe {
        gl_wrapper::NamedBufferStorage(
            self.id,
            length,
            0 as *const c_void,
            flags
        );
    } }
    pub fn map_range(&self, offset: isize, length: isize, access: u32) -> *mut c_void { unsafe {
        return gl_wrapper::MapNamedBufferRange(self.id, offset, length, access);
    } }
    pub fn buffer_sub_data(&self, buffer_start: isize, length: isize, data: *const c_void) { unsafe {
        gl_wrapper::NamedBufferSubData(
            self.id,
            buffer_start,
            length,
            data
        );
    } }
    // length and offset in bytes
    pub fn get_sub_data<T: Sized + Clone + Default>(&self, offset: isize, length: isize) -> Vec<T> { unsafe {
        let mut data = vec![Default::default(); length as usize / mem::size_of::<T>()];
        gl_wrapper::GetNamedBufferSubData(
            self.id,
            offset,
            length,
            data.as_mut_ptr() as *mut c_void
        );
        return data;
    } }
    pub fn unbind(target: u32) { unsafe {
        gl_wrapper::BindBuffer(gl_wrapper::NONE, target);
    } }
    pub fn fake() -> Buffer {
        return Buffer { id: u32::MAX };
    }
}

impl Drop for Buffer {
    fn drop(&mut self) {
        if LOG_ON_DROP {
            println!("dropped buffer {}", self.id);
        }
        if PANIC_ON_DROP {
            panic!();
        }
    }
}