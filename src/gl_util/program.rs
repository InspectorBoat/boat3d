use super::{shader::Shader, gl_wrapper};

#[derive(Debug)]
pub struct Program {
    pub id: u32
}

impl Program {
    pub fn create(vertex_shader: Shader, fragment_shader: Shader) -> Program { unsafe {
        let id = gl_wrapper::CreateProgram();

        gl_wrapper::AttachShader(id, vertex_shader.id);
        gl_wrapper::AttachShader(id, fragment_shader.id);
        gl_wrapper::LinkProgram(id);

        let mut status = 0;
        gl_wrapper::GetProgramiv(id, gl_wrapper::LINK_STATUS, &mut status);
        if status != gl_wrapper::TRUE as i32 {
            let mut length: i32 = 4096;
            let mut log: Vec<u8> = vec![0; length as usize];
            gl_wrapper::GetProgramInfoLog(id, length, &mut length, log.as_mut_ptr() as *mut i8);
            panic!("Failed to link programs: {}", std::str::from_utf8(&log).unwrap());
        }
        return Program {
            id
        };
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
}

