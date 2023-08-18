use std::{ffi::{c_void, c_uint}, mem, cell::UnsafeCell, ops::Mul, hint, simd::{Simd, SimdFloat}};

use cgmath::{Vector4, Matrix4, Matrix, InnerSpace};
use gl::types::GLsync;

use crate::{gl_util::{framebuffer::{FrameBuffer, self}, texture::Texture, renderbuffer::RenderBuffer, program::Program, gl_helper::{WindowStatus, log_if_error, log_error}, buffer::Buffer, shader::Shader, gl_wrapper}, world::{world::World, camera}, cull::{frustum_cull::frustum_cull, rasterizer::Rasterizer}, cull::frustum::{LocalFrustum, BoundsCheckResult}};

#[derive(Debug)]
pub struct WorldRenderer {
    pub framebuffer: Option<FrameBuffer>,
    pub texture_attachment: Option<Texture>,
    pub renderbuffer_attachment: Option<RenderBuffer>,
    pub index_buffer: Option<Buffer>,
    pub solid_program: Option<Program>,
    pub post_program: Option<Program>,
    pub trans_program: Option<Program>,
    pub block_texture: Option<Texture>,
    
    pub indices: UnsafeCell<Vec<*const c_void>>,
    pub solid_indirect_buffer: UnsafeCell<Vec<IndirectCommand>>,
    pub trans_indirect_buffer: UnsafeCell<Vec<IndirectCommand>>,
    pub rasterizer: UnsafeCell<Rasterizer>,
}

impl WorldRenderer {
    pub fn new() -> WorldRenderer {
        return WorldRenderer {
            framebuffer: None,
            texture_attachment: None,
            renderbuffer_attachment: None,
            index_buffer: None,
            solid_program: None,
            post_program: None,
            trans_program: None,
            block_texture: None,

            indices: UnsafeCell::new(Vec::new()),
            solid_indirect_buffer: UnsafeCell::new(Vec::new()),
            trans_indirect_buffer: UnsafeCell::new(Vec::new()),
            
            rasterizer: UnsafeCell::new(Rasterizer::new(600, 600)),
        };
    }
    
    pub fn init(&mut self, status: &WindowStatus) {
        self.make_block_texture();
        self.make_framebuffer(status);
        self.make_index_buffer();
        self.make_shader_programs();
    }

    pub fn make_block_texture(&mut self) { unsafe {
        gl_wrapper::ActiveTexture(gl_wrapper::TEXTURE0);
        let block_texture = Texture::create(gl_wrapper::TEXTURE_2D_ARRAY);
        let mut image: [u8; 16 * 16 * 4 * 64] = [0; 16 * 16 * 4 * 64].map(|_| rand::random::<u8>());
        image[0] = 255;
        image[1] = 255;
        image[2] = 255;
        image[3] = 0;
        block_texture.storage3d(1, gl_wrapper::RGBA8, 4, 4, 64);
        block_texture.sub_image_3d(0, 0, 0, 0, 4, 4, 64, gl_wrapper::RGBA, gl_wrapper::UNSIGNED_BYTE, &raw const image as *const c_void);
        block_texture.parameteri(gl_wrapper::TEXTURE_WRAP_S, gl_wrapper::REPEAT as i32);
        block_texture.parameteri(gl_wrapper::TEXTURE_WRAP_T, gl_wrapper::REPEAT as i32);
        block_texture.parameteri(gl_wrapper::TEXTURE_MIN_FILTER, gl_wrapper::NEAREST as i32);
        block_texture.parameteri(gl_wrapper::TEXTURE_MAG_FILTER, gl_wrapper::NEAREST as i32);
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
        framebuffer.bind(gl_wrapper::FRAMEBUFFER);
        
        let texture_attachment = Texture::create(gl_wrapper::TEXTURE_2D);
        texture_attachment.storage2d(1, gl_wrapper::RGBA32UI, status.width as i32, status.height as i32);
        texture_attachment.parameteri(gl_wrapper::TEXTURE_MIN_FILTER, gl_wrapper::NEAREST as i32);
        texture_attachment.parameteri(gl_wrapper::TEXTURE_MAG_FILTER, gl_wrapper::NEAREST as i32);
        framebuffer.texture2d_attachment(gl_wrapper::COLOR_ATTACHMENT0, &texture_attachment, 0);

        let renderbuffer_attachment = RenderBuffer::create();
        renderbuffer_attachment.bind(gl_wrapper::RENDERBUFFER);
        renderbuffer_attachment.storage(gl_wrapper::DEPTH24_STENCIL8, status.width, status.height);
        framebuffer.renderbuffer_attachment(gl_wrapper::DEPTH_ATTACHMENT, gl_wrapper::RENDERBUFFER, &renderbuffer_attachment);
        
        let attachments = [gl_wrapper::COLOR_ATTACHMENT0];
        framebuffer.drawbuffers(1, &raw const attachments as *const u32);

        self.framebuffer = Some(framebuffer);
        self.texture_attachment = Some(texture_attachment);
        self.renderbuffer_attachment = Some(renderbuffer_attachment);
    } }

    pub fn make_index_buffer(&mut self) { unsafe {
        gl_wrapper::Enable(gl_wrapper::PRIMITIVE_RESTART);
        gl_wrapper::PrimitiveRestartIndex(u32::MAX as u32);
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
        index_buffer.bind_target(gl_wrapper::ELEMENT_ARRAY_BUFFER);
        index_buffer.storage(1024 * 1024, gl_wrapper::DYNAMIC_STORAGE_BIT);
        index_buffer.upload_slice(&index_array.as_slice(), 0, 1024 * 1024);
        self.index_buffer = Some(index_buffer);    
    } }

    pub fn make_shader_programs(&mut self) { unsafe {
        let solid_program = Program::create(
            Shader::create(gl_wrapper::VERTEX_SHADER, include_str!("../shader/solid.glsl.vert")),
            Shader::create(gl_wrapper::FRAGMENT_SHADER, include_str!("../shader/solid.glsl.frag"))
        );
        let post_program = Program::create(
            Shader::create(gl_wrapper::VERTEX_SHADER, include_str!("../shader/post.glsl.vert")),
            Shader::create(gl_wrapper::FRAGMENT_SHADER, include_str!("../shader/post.glsl.frag"))
        );
        let trans_program = Program::create(
            Shader::create(gl_wrapper::VERTEX_SHADER, include_str!("../shader/trans.glsl.vert")),
            Shader::create(gl_wrapper::FRAGMENT_SHADER, include_str!("../shader/trans.glsl.frag"))
        );
        // texture attachment
        post_program.uniform_1i(0, 0);
        // block texture
        post_program.uniform_1i(1, 1);

        // block texture
        solid_program.uniform_1i(1, 1);

        // block texture
        trans_program.uniform_1i(1, 1);
    
        self.solid_program = Some(solid_program);
        self.post_program = Some(post_program);
        self.trans_program = Some(trans_program);
    } }

    pub fn render(&self, world: &World, status: &WindowStatus, target_framebuffer_id: i32) { unsafe {
        world.geometry_buffer_allocator.device_buffer.bind_indexed_target_base(gl_wrapper::SHADER_STORAGE_BUFFER, 0);
        world.light_buffer_allocator.device_buffer.bind_indexed_target_base(gl_wrapper::SHADER_STORAGE_BUFFER, 1);
        if let (Some(framebuffer), Some(solid_program), Some(post_program), Some(trans_program), Some(block_texture), Some(texture_attachment), Some(index_buffer)) = (&self.framebuffer, &self.solid_program, &self.post_program, &self.trans_program, &self.block_texture, &self.texture_attachment, &self.index_buffer) {
            post_program.uniform_1i(0, 0);
            post_program.uniform_1i(1, 1);
            solid_program.uniform_1i(1, 1);    
            trans_program.uniform_1i(1, 1);
    
            index_buffer.bind_target(gl_wrapper::ELEMENT_ARRAY_BUFFER);

            // sets up projection-model-view matrix
            let camera_matrix: [f32; 16] = *world.camera.get_camera_matrix().as_ref();
            solid_program.uniform_matrix_4fv(0, 1, false, &raw const camera_matrix as *const f32);
            trans_program.uniform_matrix_4fv(0, 1, false, &raw const camera_matrix as *const f32);
            
            let mut solid_drawn_sections = 0;
            let mut trans_drawn_sections = 0;
            
            self.cull_chunks(world, &mut solid_drawn_sections, &mut trans_drawn_sections);

            if status.fill_mode == gl_wrapper::LINE {
                gl_wrapper::PolygonMode(gl_wrapper::FRONT_AND_BACK, gl_wrapper::LINE);
            }
            self.render_solid(solid_drawn_sections);
            
            if status.fill_mode == gl_wrapper::LINE {
                gl_wrapper::PolygonMode(gl_wrapper::FRONT_AND_BACK, gl_wrapper::FILL);
            }
            self.render_post(target_framebuffer_id);  

            // blit depth buffer
            // gl_wrapper::BindFramebuffer(gl_wrapper::DRAW_FRAMEBUFFER, target_framebuffer_id as u32);
            // gl_wrapper::BindFramebuffer(gl_wrapper::READ_FRAMEBUFFER, framebuffer.id);
            // gl_wrapper::BlitFramebuffer(0, 0, status.width, status.height, 0, 0, status.width, status.height, gl_wrapper::DEPTH_BUFFER_BIT, gl_wrapper::NEAREST);
            
            gl_wrapper::BindFramebuffer(gl_wrapper::FRAMEBUFFER, target_framebuffer_id as u32);

            self.draw_trans(trans_drawn_sections, status);
        }
    } }

    pub fn draw_trans(&self, trans_drawn_sections: i32, status: &WindowStatus) { unsafe {
        let Some(trans_program) = &self.trans_program else { panic!(); };
        let Some(block_texture) = &self.block_texture else { panic!(); };
        
        trans_program.bind();

        gl_wrapper::Enable(gl_wrapper::DEPTH_TEST);
        gl_wrapper::Enable(gl_wrapper::BLEND);
        gl_wrapper::BlendFunc(gl_wrapper::SRC_ALPHA, gl_wrapper::ONE_MINUS_SRC_ALPHA);

        gl_wrapper::ActiveTexture(gl_wrapper::TEXTURE1);
        block_texture.bind(gl_wrapper::TEXTURE_2D_ARRAY);

        if trans_drawn_sections > 0 {
            if status.fill_mode == gl_wrapper::LINE {
                gl_wrapper::PolygonMode(gl_wrapper::FRONT_AND_BACK, gl_wrapper::LINE);
            }
            gl_wrapper::MultiDrawElementsIndirect(
                gl_wrapper::TRIANGLE_STRIP,
                gl_wrapper::UNSIGNED_INT,
                (*self.trans_indirect_buffer.get()).as_ptr() as *const c_void,
                trans_drawn_sections,
                0
            );
        }
    } }

    pub fn render_solid(&self, solid_drawn_sections: i32) { unsafe {
        let Some(solid_program) = &self.solid_program else { panic!(); };
        let Some(framebuffer) = &self.framebuffer else { panic!(); };
        let Some(block_texture) = &self.block_texture else { panic!(); };

        solid_program.bind();
        gl_wrapper::Disable(gl_wrapper::BLEND);
        framebuffer.bind(gl_wrapper::FRAMEBUFFER);
        gl_wrapper::ActiveTexture(gl_wrapper::TEXTURE1);
        block_texture.bind(gl_wrapper::TEXTURE_2D_ARRAY);

        // clear solids framebuffer
        let CLEAR_COLOR: [u32; 4] = [0, 0, 0, 0];
        framebuffer.clear_unsigned_integer_color_attachment(0, &raw const CLEAR_COLOR as *const u32);
        framebuffer.clear_depth_attachment(1.0);
        
        gl_wrapper::Enable(gl_wrapper::DEPTH_TEST);
        
        if solid_drawn_sections > 0 {
            gl_wrapper::MultiDrawElementsIndirect(
                gl_wrapper::TRIANGLE_STRIP,
                gl_wrapper::UNSIGNED_INT,
                (*self.solid_indirect_buffer.get()).as_ptr() as *const c_void,
                solid_drawn_sections,
                0
            );
        }
    } }

    pub fn render_post(&self, target_framebuffer_id: i32) { unsafe {
        let Some(post_program) = &self.post_program else { panic!(); };
        let Some(texture_attachment) = &self.texture_attachment else { panic!(); };
        let Some(block_texture) = &self.block_texture else { panic!(); };
        post_program.bind();
        gl_wrapper::BindFramebuffer(gl_wrapper::FRAMEBUFFER, target_framebuffer_id as u32);
    
        // gl_wrapper::ClearColor(1.0, 1.0, 1.0, 1.0);
        // gl_wrapper::Clear(gl_wrapper::COLOR_BUFFER_BIT | gl_wrapper::DEPTH_BUFFER_BIT);
        
        gl_wrapper::ActiveTexture(gl_wrapper::TEXTURE0);
        texture_attachment.bind(gl_wrapper::TEXTURE_2D);

        gl_wrapper::ActiveTexture(gl_wrapper::TEXTURE1);
        block_texture.bind(gl_wrapper::TEXTURE_2D_ARRAY);
                
        gl_wrapper::Disable(gl_wrapper::DEPTH_TEST);
        gl_wrapper::DrawArrays(gl_wrapper::TRIANGLES, 0, 6);
    } }

    pub fn cull_chunks(&self, world: &World, solid_drawn_sections: &mut i32, trans_drawn_sections: &mut i32) { unsafe {
        (*self.indices.get()).set_len(0);
        (*self.solid_indirect_buffer.get()).set_len(0);
        (*self.trans_indirect_buffer.get()).set_len(0);

        let local_frustum = world.camera.get_frustum();

        for section in world.sections.values() {            
            if section.solid_segment.is_none() && section.trans_segment.is_none() { continue; }
            match local_frustum.test_local_bounding_box(&section.get_local_bounding_box(&world.camera).into()) {
                BoundsCheckResult::Outside => { continue; }
                BoundsCheckResult::Partial => {}
                BoundsCheckResult::Inside => {}
            }
    
            (*self.indices.get()).push(0 as *const c_void);

            // only render section if it has a geometry and light segment
            if let (Some(solid_segment), Some(solid_light_segment)) = (&section.solid_segment, &section.solid_light_segment) {
                let solid_count = section.solid_quad_count as i32 * WorldRenderer::ELEMENT_INDICES_PER_QUAD;
                let solid_base_vertex = solid_segment.offset as i32 / WorldRenderer::BYTES_PER_QUAD * WorldRenderer::ELEMENTS_PER_QUAD;
                (*self.solid_indirect_buffer.get()).push(IndirectCommand {
                    count: solid_count as u32,
                    instance_count: 1,
                    first_index: 0,
                    base_vertex: solid_base_vertex,
                    base_instance: 0,
                });
                *solid_drawn_sections += 1;
            }
    
            if let (Some(trans_segment), Some(solid_light_segment)) = (&section.trans_segment, &section.trans_light_segment) {
                let trans_count = section.trans_quad_count as i32 * WorldRenderer::ELEMENT_INDICES_PER_QUAD;
                let trans_base_vertex = trans_segment.offset as i32 / WorldRenderer::BYTES_PER_QUAD * WorldRenderer::ELEMENTS_PER_QUAD;
                (*self.trans_indirect_buffer.get()).push(IndirectCommand {
                    count: trans_count as u32,
                    instance_count: 1,
                    first_index: 0,
                    base_vertex: trans_base_vertex,
                    base_instance: 0,
                });

                *trans_drawn_sections += 1;
            }
        }
    } }

    pub const ELEMENT_INDICES_PER_QUAD: i32 = 5;
    pub const BYTES_PER_QUAD: i32 = 8;
    pub const ELEMENTS_PER_QUAD: i32 = 4;
}

pub struct IndirectCommand {
    count: u32,
    instance_count: u32,
    first_index: u32,
    base_vertex: i32,
    base_instance: u32,
}