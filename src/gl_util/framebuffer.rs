use std::ffi::c_void;

use super::{texture::Texture, renderbuffer::RenderBuffer, gl_wrapper};

#[derive(Debug)]
pub struct FrameBuffer {
    pub id: u32
}

impl FrameBuffer {
    pub fn create() -> FrameBuffer { unsafe {
        let mut id: u32 = 0;
        gl_wrapper::GenFramebuffers(1, &mut id);
        return FrameBuffer {
            id: id
        };
    } }
    pub fn bind(&self, target: u32) { unsafe {
        gl_wrapper::BindFramebuffer(target, self.id);
    } }
    pub fn texture2d_attachment(&self, attachment: u32, texture: &Texture, level: i32) { unsafe {
        gl_wrapper::NamedFramebufferTexture(self.id, attachment, texture.id, level)
    } }
    pub fn renderbuffer_attachment(&self, attachment: u32, renderbuffer_target: u32, renderbuffer: &RenderBuffer) { unsafe {
        gl_wrapper::NamedFramebufferRenderbuffer(self.id, attachment, renderbuffer_target, renderbuffer.id)
    } }
    pub fn clear_bind(target: u32) { unsafe {
        gl_wrapper::BindFramebuffer(target, 0);
    } }
    pub fn clear_unsigned_integer_color_attachment(&self, attachment: i32, value: *const u32) { unsafe {
        gl_wrapper::ClearNamedFramebufferuiv(self.id, gl_wrapper::COLOR, attachment, value);
    } }
    pub fn clear_signed_integer_color_attachment(&self, attachment: i32, value: *const i32) { unsafe {
        gl_wrapper::ClearNamedFramebufferiv(self.id, gl_wrapper::COLOR, attachment, value);
    } }
    pub fn clear_float_color_attachment(&self, attachment: i32, value: *const f32) { unsafe {
        gl_wrapper::ClearNamedFramebufferfv(self.id, gl_wrapper::COLOR, attachment, value);
    } }
    pub fn clear_depth_stencil_attachment(&self, depth: f32, stencil: i32) { unsafe {
        gl_wrapper::ClearNamedFramebufferfi(self.id, gl_wrapper::DEPTH_STENCIL, 0, depth, stencil);
    } }
    pub fn clear_depth_attachment(&self, depth: f32) { unsafe {
        gl_wrapper::ClearNamedFramebufferfv(self.id, gl_wrapper::DEPTH, 0, &raw const depth);
    } }
    pub fn clear_stencil_attachment(&self, stencil: i32) { unsafe {
        gl_wrapper::ClearNamedFramebufferiv(self.id, gl_wrapper::STENCIL, 0, &raw const stencil);
    } }
    pub fn drawbuffers(&self, n: i32, bufs: *const u32) { unsafe {
        gl_wrapper::NamedFramebufferDrawBuffers(self.id, n, bufs);
    } }
    pub fn kill(self) { unsafe {
        gl_wrapper::DeleteFramebuffers(1, &self.id);
    } }
}
