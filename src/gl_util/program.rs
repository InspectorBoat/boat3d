use std::mem::{ManuallyDrop, self};

use super::{shader::Shader, gl_wrapper, gl_helper::{PANIC_ON_DROP, LOG_ON_DROP}};

#[derive(Debug)]
pub struct Program {
    pub id: u32
}

impl Program {
    pub fn create_with(vertex_shader: Shader, fragment_shader: Shader) -> Program { unsafe {
        let program = Program::create();
        
        program.attach_shader(&vertex_shader);
        program.attach_shader(&fragment_shader);
        program.link();
        program.detach_shader(&vertex_shader);
        program.detach_shader(&fragment_shader);
        vertex_shader.kill();
        fragment_shader.kill();

        let mut status = 0;
        gl_wrapper::GetProgramiv(program.id, gl_wrapper::LINK_STATUS, &raw mut status);
        if status != gl_wrapper::TRUE as i32 {
            let mut length: i32 = 4096;
            let mut log: [u8; 4096] = [0; 4096];
            gl_wrapper::GetProgramInfoLog(program.id, length, &mut length, &raw mut log as *mut i8);
            panic!("Failed to link programs: {}", std::str::from_utf8(&log).unwrap());
        }
        return program;
    } }

    pub fn create() -> Program { unsafe {
        return Program {
            id: gl_wrapper::CreateProgram()
        };
    } }

    pub fn attach_shader(&self, shader: &Shader) { unsafe {
        gl_wrapper::AttachShader(self.id, shader.id);
    } }

    pub fn link(&self) { unsafe {
        gl_wrapper::LinkProgram(self.id);
    } }

    pub fn detach_shader(&self, shader: &Shader) { unsafe {
        gl_wrapper::DetachShader(self.id, shader.id);
    } }

    pub fn bind(&self) { unsafe {
        gl_wrapper::UseProgram(self.id);
    } }

    pub fn uniform_1i(&self, location: i32, v0: i32) { unsafe {
        gl_wrapper::ProgramUniform1i(self.id, location, v0);
    } }

    pub fn uniform_matrix_4fv(&self, location: i32, count: i32, transpose: bool, v0: *const f32) { unsafe {
        gl_wrapper::ProgramUniformMatrix4fv(self.id, location, count, transpose as u8, v0);
    } }

    pub fn kill(self) { unsafe {
        gl_wrapper::DeleteProgram(self.id);
        mem::forget(self);
    } }
}

impl Drop for Program {
    fn drop(&mut self) {
        if LOG_ON_DROP {
            println!("dropped program {}", self.id);
        }
        if PANIC_ON_DROP {
            panic!();
        }
    }
}