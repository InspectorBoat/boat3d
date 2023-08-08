use super::gl_wrapper;

#[derive(Debug)]
pub struct Texture {
    pub id: u32
}

impl Texture {
    pub fn create() -> Texture { unsafe {
        let mut id: u32 = 0;
        gl_wrapper::GenTextures(1, &mut id);
        return Texture { id: id };
    } }
    pub fn bind(&self, target: u32) { unsafe {
        gl_wrapper::BindTexture(target, self.id);
    } }
    pub fn kill(self) { unsafe {
        gl_wrapper::DeleteTextures(1, &self.id);
    } }
}

impl Drop for Texture {
    fn drop(&mut self) {
        // panic!();
    }
}

