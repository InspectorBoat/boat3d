use std::{ffi::c_void, mem};

#[derive(Debug)]
pub struct Buffer {
    pub id: u32
}

impl Buffer {
    pub fn create() -> Buffer { unsafe {
        let mut id: u32 = 0;
        gl::CreateBuffers(1, &mut id);
        return Buffer {
            id: id
        };
    } }
    pub fn generate() -> Buffer { unsafe {
        let mut id: u32 = 0;
        gl::GenBuffers(1, &mut id);
        return Buffer { id };
    } }
    pub fn bind_target(&self, target: u32) { unsafe {
        gl::BindBuffer(target, self.id);
    } }
    pub fn bind_indexed_target_base(&self, target: u32, index: u32) { unsafe {
            gl::BindBufferBase(target, index, self.id);
    } }   
    pub fn bind_indexed_target(&self, target: u32, index: u32, offset: isize, length: isize) { unsafe {
            gl::BindBufferRange(target, index, self.id, offset, length);
    } }
    pub fn valid(&self) -> bool { unsafe {
        return gl::IsBuffer(self.id) == gl::TRUE;
    } }
    pub fn kill(self) { unsafe {
        gl::DeleteBuffers(1, &self.id);
    } }
    pub fn storage(&self, length: isize, flags: u32) { unsafe {
        gl::NamedBufferStorage(
            self.id,
            length,
            0 as *const c_void,
            flags
        );
    } }
    pub fn upload<T>(&self, data: &[T], length: isize, usage: gl::types::GLenum) { unsafe {
        gl::NamedBufferData(
            self.id,
            length,
            data.as_ptr() as *const c_void, usage
        );
    } }
    pub fn upload_slice<T>(&self, data: &[T], buffer_start: isize, length: isize) { unsafe {
        gl::NamedBufferSubData(
            self.id,
            buffer_start,
            length,
            data.as_ptr() as *const c_void
        );
    } }
    // length and offset in bytes
    pub fn get_sub_data<T: Sized + Clone + Default>(&self, offset: isize, length: isize) -> Vec<T> { unsafe {
        let mut data = vec![Default::default(); length as usize / mem::size_of::<T>()];
        gl::GetNamedBufferSubData(
            self.id,
            offset,
            length,
            data.as_mut_ptr() as *mut c_void
        );
        return data;
    } }
    pub fn unbind(target: u32) { unsafe {
        gl::BindBuffer(gl::NONE, target);
    } }
    pub fn fake() -> Buffer {
        return Buffer { id: u32::MAX };
    }
}

impl Drop for Buffer {
    fn drop(&mut self) {
        // if self.valid() {
            // panic!();
        // }
    }
}

