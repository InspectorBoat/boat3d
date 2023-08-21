use std::mem;

use super::{gl_wrapper, gl_helper::{PANIC_ON_DROP, LOG_ON_DROP}};

#[derive(Debug)]
pub struct RenderBuffer {
    pub id: u32
}

impl RenderBuffer {
    pub fn create() -> RenderBuffer { unsafe {
        let mut id: u32 = 0;
        gl_wrapper::GenRenderbuffers(1, &mut id);
        return RenderBuffer {
            id: id
        };
    } }
    pub fn storage(&self, internalformat: u32, width: i32, height: i32) { unsafe {
        gl_wrapper::NamedRenderbufferStorage(self.id, internalformat, width, height)
    } }
    pub fn bind(&self, target: u32) { unsafe {
        gl_wrapper::BindRenderbuffer(gl_wrapper::RENDERBUFFER, self.id);
    } }
    pub fn kill(self) { unsafe {
        gl_wrapper::DeleteRenderbuffers(1, &self.id);
        mem::forget(self);
    } }
}

impl Drop for RenderBuffer {
    fn drop(&mut self) {
        if LOG_ON_DROP {
            println!("dropped renderbuffer {}", self.id);
        }
        if PANIC_ON_DROP {
            panic!();
        }
    }
}