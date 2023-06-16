use std::{ffi::{c_void, CString}, ptr, ops::Deref};

use gl::types::GLuint;
use glfw::{Window, WindowEvent, Glfw};

use log::debug;


// /*
pub fn init_gl(window: &mut Window) {
    return gl::load_with(|s| window.get_proc_address(s) as *const _);
}
pub fn init_glfw() -> Glfw {
    return glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
}
pub fn create_window() -> (Window, std::sync::mpsc::Receiver<(f64, WindowEvent)>) {
    return glfw::init(glfw::FAIL_ON_ERRORS).unwrap().create_window(600, 600, "boat3d", glfw::WindowMode::Windowed)
            .expect("Failed to create GLFW window.");
}
// */

pub fn log_error() {
    match unsafe { gl::GetError() } {
        gl::INVALID_ENUM => println!("INVALID_ENUM"),
        gl::INVALID_VALUE => println!("INVALID_VALUE"),
        gl::INVALID_OPERATION => println!("INVALID_OPERATION"),
        0 => println!("NONE"),
        default => println!("{default}")
    }
}
pub fn log_if_error() {
    match unsafe { gl::GetError() } {
        0 => return,
        gl::INVALID_ENUM => println!("INVALID_ENUM"),
        gl::INVALID_VALUE => println!("INVALID_VALUE"),
        gl::INVALID_OPERATION => println!("INVALID_OPERATION"),
        default => println!("{default}")
    }
}

#[derive(Debug)]
#[derive(PartialEq)]
pub struct Buffer {
    id: u32
}

#[allow(dead_code)]
impl Buffer {
    pub fn create() -> Buffer {
        let mut id: u32 = 0;
        unsafe { gl::CreateBuffers(1, &mut id); }
        return Buffer { id };
    }
    pub fn generate() -> Buffer {
        let mut id: u32 = 0;
        unsafe { gl::GenBuffers(1, &mut id); }
        return Buffer { id };
    }
    pub fn bind_target(&self, target: u32) {
        unsafe { gl::BindBuffer(target, self.id) }
    }
    pub fn bind_indexed_target_base(&self, target: u32, index: u32) {
        unsafe {
            gl::BindBufferBase(target, index, self.id);
        }
    }
    pub fn bind_indexed_target(&self, target: u32, index: u32, offset: isize, length: isize) {
        unsafe {
            gl::BindBufferRange(target, index, self.id, offset, length);
        }
    }
    pub fn valid(&self) -> bool {
        return unsafe { gl::IsBuffer(self.id) } == gl::TRUE
    }
    pub fn kill(self) {
        unsafe { gl::DeleteBuffers(1, &self.id) }
    }
    pub fn storage(&self, length: isize, flags: u32) {
        unsafe {
            gl::NamedBufferStorage(self.id, length, ptr::null(), flags);
        }
    }
    pub fn upload<T>(&self, data: &[T], length: isize, usage: gl::types::GLenum) {
        unsafe {
            gl::NamedBufferData(
                self.id,
                length,
                data.as_ptr() as *const c_void, usage
            )
        }
    }
    pub fn upload_slice<T>(&self, data: &[T], buffer_start: isize, length: isize) {
        unsafe {
            gl::NamedBufferSubData(
                self.id,
                buffer_start,
                length,
                data.as_ptr() as *const c_void
            )
        }
    }
    pub fn get_sub_data<T: Sized + Clone + Default>(&self, offset: isize, length: isize) -> Vec<T> {
        let mut data = vec![Default::default(); (length as usize) * std::mem::size_of::<T>()];
        unsafe {
            gl::GetNamedBufferSubData(self.id, offset, length, data.as_mut_ptr() as *mut c_void);
        }
        return data;
    }
    pub fn unbind(target: u32) {
        unsafe { gl::BindBuffer(gl::NONE, target) }
    }
}

impl Drop for Buffer {
    fn drop(&mut self) {
        // if self.valid() {
            // panic!();
        // }
    }
}

pub struct Shader {
    id: u32
}

impl Shader {
    pub fn create(r#type: u32, source: &str) -> Shader {
        let id = unsafe { gl::CreateShader(r#type) };
        unsafe {
            let string = CString::new(source).unwrap();
            gl::ShaderSource(id, 1, &string.as_ptr(), ptr::null());
            gl::CompileShader(id);

            let mut status = 0;
            gl::GetShaderiv(id, gl::COMPILE_STATUS, &mut status);
            if status != gl::TRUE as i32 {
                panic!("Failed to compile {} shader", if r#type == gl::VERTEX_SHADER { "vertex" } else { "fragment" })
            }
        }
        return Shader {
            id
        }
    }
}

pub struct Program {
    id: u32
}

impl Program {
    pub fn create(vertex_shader: Shader, fragment_shader: Shader) -> Program {
        let id = unsafe { gl::CreateProgram() };
        unsafe {
            gl::AttachShader(id, vertex_shader.id);
            gl::AttachShader(id, fragment_shader.id);
            gl::LinkProgram(id);

            let mut status = 0;
            gl::GetProgramiv(id, gl::LINK_STATUS, &mut status);
            if status != gl::TRUE as i32 { panic!() }
        }

        return Program {
            id
        }
    }

    pub fn bind(program: Program) {
        unsafe { gl::UseProgram(program.id) }
    }
}

pub struct WindowStatus {
    pub fill_mode: gl::types::GLenum,
    pub maximized: bool,
    pub mouse_captured: bool
}

impl WindowStatus {
    pub fn new() -> WindowStatus {
        return WindowStatus { fill_mode: gl::FILL, maximized: false, mouse_captured: false}
    }
}

#[derive(Debug)]
pub struct BufferArena {
    pub buffer: Buffer,
    pub staging_buffer: Buffer,
    pub pages: Box<[bool; 1048576]>,

    // page size of 1024
}

impl BufferArena {
    pub fn new() -> BufferArena {
        let buffer = Buffer::create();
        buffer.storage(1048576 * 1024, gl::DYNAMIC_STORAGE_BIT);
        let staging_buffer = Buffer::create();
        staging_buffer.storage(1048576, gl::DYNAMIC_STORAGE_BIT);
        return BufferArena {
            buffer: buffer,
            staging_buffer: staging_buffer,
            pages: unsafe { Box::<[bool; 1048576]>::new_zeroed().assume_init() }
        }
    }
    // Size in bytes
    pub fn allocate(&mut self, size: usize) -> Option<Page> {
        if size == 0 { return None; }
        let size = size.div_ceil(1024);

        let mut run = 0;
        let mut start = 0;
        for i in 0..1048576 {
            if !self.pages[i] {
                if run == 0 { start = i; }
                run += 1;
            }
            if run == size {
                for j in start..(start + size) {
                    self.pages[j] = true;
                }
                return Some(Page {
                    start: start,
                    size: size
                });
            }
        }
        return None;
    }
    pub fn deallocate(&mut self, page: Page) {
        for i in page.start..(page.start + page.size) {
            self.pages[i] = false;
        }
    }

    pub fn upload_slice<T>(&mut self, page: Page, data: &[T], start: isize, length: isize) {
        self.buffer.upload_slice(data, (page.start * 1024 + start as usize) as isize, length);
    }
    pub fn upload<T>(&mut self, page: &Page, data: &[T], length: isize) {
        self.staging_buffer.upload_slice(data, 0, length);

        unsafe { gl::CopyNamedBufferSubData(self.staging_buffer.id, self.buffer.id, 0, (page.start * 1024) as isize, length as isize); }
        // self.buffer.upload_slice(data, (page.start * 1024) as isize, length);
    }
}

#[derive(Debug)]
pub struct Page {
    pub start: usize,
    pub size: usize,
}