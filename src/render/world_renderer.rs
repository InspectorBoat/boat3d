use std::{ffi::c_void, mem};

use gl::types::GLsync;

use crate::gl_util::{framebuffer::FrameBuffer, texture::Texture, renderbuffer::RenderBuffer, program::Program, gl_helper::WindowStatus, buffer::Buffer, shader::Shader};

#[derive(Debug)]
pub struct WorldRenderer {
    pub framebuffer: Option<FrameBuffer>,
    pub texture_attachment: Option<Texture>,
    pub renderbuffer_attachment: Option<RenderBuffer>,
    pub index_buffer: Option<Buffer>,
    pub geometry_program: Option<Program>,
    pub post_program: Option<Program>,
    pub screen_buffer: Option<Buffer>,
    pub block_texture: Option<Texture>,
    pub counts: Vec<i32>,
    pub indices: Vec<*const c_void>,
    pub base_vertices: Vec<i32>,
    pub fences: Vec<GLsync>,
}

impl WorldRenderer {
    pub fn new() -> WorldRenderer {
        return WorldRenderer {
            framebuffer: None,
            texture_attachment: None,
            renderbuffer_attachment: None,
            index_buffer: None,
            geometry_program: None,
            post_program: None,
            screen_buffer: None,
            block_texture: None,
            counts: Vec::new(),
            indices: Vec::new(),
            base_vertices: Vec::new(),
            fences: Vec::new(),
        };
    }
    
    pub fn make_block_texture(&mut self) { unsafe {
        gl::ActiveTexture(gl::TEXTURE0);
        let block_texture = Texture::create();
        block_texture.bind(gl::TEXTURE_2D_ARRAY);
        let mut image: [u8; 16 * 16 * 4 * 64] = [0; 16 * 16 * 4 * 64].map(|_| rand::random::<u8>() & 127);
        image[0] = 127;
        image[1] = 127;
        image[2] = 127;
        image[3] = 0;

        gl::TexImage3D(gl::TEXTURE_2D_ARRAY, 0, gl::RGBA as i32, 16, 16, 64, 0, gl::RGBA, gl::BYTE, &raw const image as *const c_void);
        gl::TexParameteri(gl::TEXTURE_2D_ARRAY, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
        gl::TexParameteri(gl::TEXTURE_2D_ARRAY, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
        // set texture filtering parameters
        gl::TexParameteri(gl::TEXTURE_2D_ARRAY, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
        gl::TexParameteri(gl::TEXTURE_2D_ARRAY, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
        self.block_texture = Some(block_texture);
    } }
    
    pub fn make_framebuffer(&mut self, status: &WindowStatus) { unsafe {
        if let Some(framebuffer) = self.framebuffer.take() {
            framebuffer.kill();
        }
        if let Some(texture_attachment) = self.texture_attachment.take() {
            texture_attachment.kill();
        }
        if let Some(renderbuffer_attachment) = self.renderbuffer_attachment.take() {
            renderbuffer_attachment.kill();
        }

        let framebuffer = FrameBuffer::create();
        framebuffer.bind(gl::FRAMEBUFFER);
    
        let texture_attachment = Texture::create();
        gl::ActiveTexture(gl::TEXTURE0);
        texture_attachment.bind(gl::TEXTURE_2D);
        gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RGBA32UI as i32, status.width as i32, status.height as i32, 0, gl::RGBA_INTEGER, gl::INT, 0  as *const c_void);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
        FrameBuffer::texture2d_attachment(gl::FRAMEBUFFER, gl::COLOR_ATTACHMENT0, gl::TEXTURE_2D, &texture_attachment, 0);
    
        let renderbuffer_attachment = RenderBuffer::create();
        renderbuffer_attachment.bind(gl::RENDERBUFFER);
        gl::RenderbufferStorage(gl::RENDERBUFFER, gl::DEPTH_COMPONENT32F, status.width as i32, status.height as i32);  
        FrameBuffer::renderbuffer_attachment(gl::FRAMEBUFFER, gl::DEPTH_ATTACHMENT, gl::RENDERBUFFER, &renderbuffer_attachment);
        
        let attachments: [u32; 1] = [gl::COLOR_ATTACHMENT0];
        gl::DrawBuffers(1, &raw const attachments as *const u32);

        self.framebuffer = Some(framebuffer);
        self.texture_attachment = Some(texture_attachment);
        self.renderbuffer_attachment = Some(renderbuffer_attachment);
    } }

    pub fn make_index_buffer(&mut self) { unsafe {
        gl::Enable(gl::PRIMITIVE_RESTART);
        gl::PrimitiveRestartIndex(u32::MAX as u32);
        let mut index_array = Vec::<u32>::with_capacity(1024 * 1024 / 4);
        let mut j = 0;
        for i in 0..(1024 * 1024 / 4) {
            if i % 5 == 4 {
                index_array.push(u32::MAX);
            }
            else {
                index_array.push(j);
                j += 1;
            }
        }
        let index_buffer = Buffer::create();
        index_buffer.bind_target(gl::ELEMENT_ARRAY_BUFFER);
        index_buffer.storage(1024 * 1024, gl::DYNAMIC_STORAGE_BIT);
        index_buffer.upload_slice(&index_array.as_slice(), 0, 1024 * 1024);
        self.index_buffer = Some(index_buffer);    
    } }

    pub fn make_shader_programs(&mut self) { unsafe {
        let geometry_program = Program::create(
            Shader::create(gl::VERTEX_SHADER, include_str!("../shader/geometry.glsl.vert")),
            Shader::create(gl::FRAGMENT_SHADER, include_str!("../shader/geometry.glsl.frag"))
        );
        let post_program = Program::create(
            Shader::create(gl::VERTEX_SHADER, include_str!("../shader/post.glsl.vert")),
            Shader::create(gl::FRAGMENT_SHADER, include_str!("../shader/post.glsl.frag"))
        );
        post_program.uniform1i(0, 0);
        post_program.uniform1i(1, 1);
        geometry_program.uniform1i(1, 1);
    
        self.geometry_program = Some(geometry_program);
        self.post_program = Some(post_program);
    } }

    pub fn make_screen_buffer(&mut self) { unsafe {
        let screen_vertices: [f32; 24] = [
            -1.0,  1.0,  0.0, 1.0,
            -1.0, -1.0,  0.0, 0.0,
             1.0, -1.0,  1.0, 0.0,
    
            -1.0,  1.0,  0.0, 1.0,
             1.0, -1.0,  1.0, 0.0,
             1.0,  1.0,  1.0, 1.0,
        ];
    
        let screen_buffer = Buffer::create();
        screen_buffer.upload(&screen_vertices, mem::size_of::<[f32; 24]>() as isize, gl::STATIC_DRAW);
        screen_buffer.bind_target(gl::ARRAY_BUFFER);
        gl::EnableVertexAttribArray(0);
        gl::VertexAttribPointer(0, 2, gl::FLOAT, gl::FALSE, 4 * mem::size_of::<f32>() as i32, 0 as *const c_void);
        gl::EnableVertexAttribArray(1);
        gl::VertexAttribPointer(1, 2, gl::FLOAT, gl::FALSE, 4 * mem::size_of::<f32>() as i32, (2 * mem::size_of::<f32>()) as *const c_void);
        self.screen_buffer = Some(screen_buffer);
    } }
}