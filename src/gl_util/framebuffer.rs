use super::{texture::Texture, renderbuffer::RenderBuffer};

#[derive(Debug)]
pub struct FrameBuffer {
    pub id: u32
}

impl FrameBuffer {
    pub fn create() -> FrameBuffer { unsafe {
        let mut id: u32 = 0;
        gl::GenFramebuffers(1, &mut id);
        return FrameBuffer {
            id: id
        };
    } }
    pub fn bind(&self, target: u32) { unsafe {
        gl::BindFramebuffer(target, self.id);
    } }
    pub fn texture2d_attachment(target: u32, attachment_location: u32, texture_type: u32, texture: &Texture, level: i32) { unsafe {
        gl::FramebufferTexture2D(target, attachment_location, texture_type, texture.id, level);
    } }
    pub fn renderbuffer_attachment(target: u32, attachment_type: u32, renderbuffer_target: u32, renderbuffer: &RenderBuffer) { unsafe {
        gl::FramebufferRenderbuffer(target, attachment_type, renderbuffer_target, renderbuffer.id)
    } }
    pub fn clear_bind(target: u32) { unsafe {
        gl::BindFramebuffer(target, 0);
    } }
    pub fn clear_unsigned_integer_color_attachment(&self, attachment_location: i32, clear_color: *const u32) { unsafe {
        gl::ClearNamedFramebufferuiv(self.id, gl::COLOR, attachment_location, clear_color);
    } }
    pub fn kill(self) { unsafe {
        gl::DeleteFramebuffers(1, &self.id);
    } }
}
