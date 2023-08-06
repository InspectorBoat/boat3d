use super::shader::Shader;

#[derive(Debug)]
pub struct Program {
    pub id: u32
}

impl Program {
    pub fn create(vertex_shader: Shader, fragment_shader: Shader) -> Program { unsafe {
        let id = gl::CreateProgram();

        gl::AttachShader(id, vertex_shader.id);
        gl::AttachShader(id, fragment_shader.id);
        gl::LinkProgram(id);

        let mut status = 0;
        gl::GetProgramiv(id, gl::LINK_STATUS, &mut status);
        if status != gl::TRUE as i32 {
            let mut length: i32 = 4096;
            let mut log: Vec<u8> = vec![0; length as usize];
            gl::GetProgramInfoLog(id, length, &mut length, log.as_mut_ptr() as *mut i8);
            panic!("Failed to link programs: {}", std::str::from_utf8(&log).unwrap());
        }
        return Program {
            id
        };
    } }

    pub fn bind(&self) { unsafe {
        gl::UseProgram(self.id);
    } }

    pub fn uniform_1i(&self, location: i32, v0: i32) { unsafe {
        gl::ProgramUniform1i(self.id, location, v0);
    } }

    pub fn uniform_matrix_4fv(&self, location: i32, count: i32, transpose: bool, v0: *const f32) { unsafe {
        gl::ProgramUniformMatrix4fv(self.id, location, count, transpose as u8, v0);
    } }
}

