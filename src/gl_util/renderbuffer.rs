use super::gl_wrapper;

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
    pub fn bind(&self, target: u32) { unsafe {
        gl_wrapper::BindRenderbuffer(gl_wrapper::RENDERBUFFER, self.id);
    } }
    pub fn kill(self) { unsafe {
        gl_wrapper::DeleteRenderbuffers(1, &self.id);
    } }
}
