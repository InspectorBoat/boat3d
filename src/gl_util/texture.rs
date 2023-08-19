use std::ffi::c_void;

use super::gl_wrapper;

#[derive(Debug)]
pub struct Texture {
    pub id: u32
}

impl Texture {
    pub fn create(target: u32) -> Texture { unsafe {
        let mut id: u32 = 0;
        gl_wrapper::CreateTextures(target, 1, &raw mut id);
        return Texture { id: id };
    } }
    pub fn storage_2d(&self, levels: i32, internalformat: u32, width: i32, height: i32) { unsafe {
        gl_wrapper::TextureStorage2D(self.id, levels, internalformat, width, height);
    } }
    pub fn storage_3d(&self, levels: i32, internalformat: u32, width: i32, height: i32, depth: i32) { unsafe {
        gl_wrapper::TextureStorage3D(self.id, levels, internalformat, width, height, depth);
    } }
    pub fn sub_image_2d(&self, level: i32, xoffset: i32, yoffset: i32, width: i32, height: i32, format: u32, type_: u32, pixels: *const c_void) { unsafe {
        gl_wrapper::TextureSubImage2D(self.id, level, xoffset, yoffset, width, height, format, type_, pixels);
    } }
    pub fn sub_image_3d(&self, level: i32, xoffset: i32, yoffset: i32, zoffset: i32, width: i32, height: i32, depth: i32, format: u32, type_: u32, pixels: *const c_void) { unsafe {
        gl_wrapper::TextureSubImage3D(self.id, level, xoffset, yoffset, zoffset, width, height, depth, format, type_, pixels);
    } }
    pub fn parameter_signed_integer(&self, pname: u32, param: i32) { unsafe {
        gl_wrapper::TextureParameteri(self.id, pname, param);
    } }
    pub fn bind_unit(&self, unit: u32) { unsafe {
        gl_wrapper::BindTextureUnit(unit, self.id);
    } }
    pub fn bind(&self, target: u32) { unsafe {
        gl_wrapper::BindTexture(target, self.id);
    } }
    pub fn kill(self) { unsafe {
        gl_wrapper::DeleteTextures(1, &raw const self.id);
    } }
}

impl Drop for Texture {
    fn drop(&mut self) {
        // panic!();
    }
}

