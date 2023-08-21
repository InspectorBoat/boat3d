use std::{ffi::CString, mem};

use super::{gl_wrapper, gl_helper::{PANIC_ON_DROP, LOG_ON_DROP}};

#[derive(Debug)]
pub struct Shader {
    pub id: u32
}

impl Shader {
    pub fn create(r#type: u32, source: &str) -> Shader { unsafe {
        let id = gl_wrapper::CreateShader(r#type);
        let string = CString::new(source).unwrap();
        gl_wrapper::ShaderSource(id, 1, &string.as_ptr(), 0 as *const i32);
        gl_wrapper::CompileShader(id);

        let mut status = 0;
        gl_wrapper::GetShaderiv(id, gl_wrapper::COMPILE_STATUS, &mut status);
        if status != gl_wrapper::TRUE as i32 {
            let mut length: i32 = 4096;
            let mut log: Vec<u8> = vec![0; length as usize];
            gl_wrapper::GetShaderInfoLog(id, length, &mut length, log.as_mut_ptr() as *mut i8);
            
            panic!("Failed to compile {} shader: {}", if r#type == gl_wrapper::VERTEX_SHADER { "vertex" } else { "fragment" }, std::str::from_utf8(&log).unwrap())
        }
        return Shader { id: id };
    } }
    pub fn kill(self) { unsafe {
        gl_wrapper::DeleteShader(self.id);
        mem::forget(self);
    } }
}

impl Drop for Shader {
    fn drop(&mut self) {
        if LOG_ON_DROP {
            println!("dropped shader {}", self.id);
        }
        if PANIC_ON_DROP {
            panic!();
        }
    }
}