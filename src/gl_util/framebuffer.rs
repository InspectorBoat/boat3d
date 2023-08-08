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
    pub fn texture2d_attachment(target: u32, attachment_location: u32, texture_type: u32, texture: &Texture, level: i32) { unsafe {
        gl_wrapper::FramebufferTexture2D(target, attachment_location, texture_type, texture.id, level);
    } }
    pub fn renderbuffer_attachment(target: u32, attachment_type: u32, renderbuffer_target: u32, renderbuffer: &RenderBuffer) { unsafe {
        gl_wrapper::FramebufferRenderbuffer(target, attachment_type, renderbuffer_target, renderbuffer.id)
    } }
    pub fn clear_bind(target: u32) { unsafe {
        gl_wrapper::BindFramebuffer(target, 0);
    } }
    pub fn clear_unsigned_integer_color_attachment(&self, attachment_location: i32, clear_color: *const u32) { unsafe {
        gl_wrapper::ClearNamedFramebufferuiv(self.id, gl_wrapper::COLOR, attachment_location, clear_color);
    } }
    pub fn kill(self) { unsafe {
        gl_wrapper::DeleteFramebuffers(1, &self.id);
    } }
}
