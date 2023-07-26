#[derive(Debug)]
pub struct RenderBuffer {
    pub id: u32
}

impl RenderBuffer {
    pub fn create() -> RenderBuffer { unsafe {
        let mut id: u32 = 0;
        gl::GenRenderbuffers(1, &mut id);
        return RenderBuffer {
            id: id
        };
    } }
    pub fn bind(&self, target: u32) { unsafe {
        gl::BindRenderbuffer(gl::RENDERBUFFER, self.id);
    } }
    pub fn kill(self) { unsafe {
        gl::DeleteRenderbuffers(1, &self.id);
    } }
}
