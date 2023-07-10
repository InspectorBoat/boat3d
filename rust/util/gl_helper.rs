use std::{ffi::{c_void, CString}, ptr, ops::Deref, num::NonZeroUsize};

use gl::types::GLuint;
use glfw::{Window, WindowEvent, Glfw, Context};

use log::debug;


// /*
pub fn init_gl(window: &mut Window) {
    gl::load_with(|s| window.get_proc_address(s) as *const _);
    window.set_all_polling(true);
    window.make_current();
}
pub fn init_glfw() -> Glfw {
    return glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
}
pub fn create_window(status: &WindowStatus) -> (Window, std::sync::mpsc::Receiver<(f64, WindowEvent)>) {
    return glfw::init(glfw::FAIL_ON_ERRORS).unwrap().create_window(status.width as u32, status.height as u32, "boat3d", glfw::WindowMode::Windowed)
            .expect("Failed to create GLFW window.");
}
pub fn setup_element_array() { unsafe {
    gl::ClearColor(1.0, 1.0, 1.0, 1.0);
    gl::Enable(gl::DEPTH_TEST);
    gl::Enable(gl::PRIMITIVE_RESTART);
    gl::PrimitiveRestartIndex(u32::MAX);
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
    index_buffer.storage(1024 * 1024 / 4, gl::DYNAMIC_STORAGE_BIT);
    index_buffer.upload_slice(&index_array.as_slice(), 0, index_array.len() as isize);
} }

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
    pub fn create() -> Buffer { unsafe {
        let mut id: u32 = 0;
        gl::CreateBuffers(1, &mut id);
        return Buffer { id };
    } }
    pub fn generate() -> Buffer { unsafe {
        let mut id: u32 = 0;
        gl::GenBuffers(1, &mut id);
        return Buffer { id };
    } }
    pub fn bind_target(&self, target: u32) { unsafe {
        gl::BindBuffer(target, self.id);
    } }
    pub fn bind_indexed_target_base(&self, target: u32, index: u32) { unsafe {
            gl::BindBufferBase(target, index, self.id);
    } }   
    pub fn bind_indexed_target(&self, target: u32, index: u32, offset: isize, length: isize) { unsafe {
            gl::BindBufferRange(target, index, self.id, offset, length);
    } }
    pub fn valid(&self) -> bool { unsafe {
        return gl::IsBuffer(self.id) == gl::TRUE;
    } }
    pub fn kill(self) { unsafe {
        gl::DeleteBuffers(1, &self.id);
    } }
    pub fn storage(&self, length: isize, flags: u32) { unsafe {
        gl::NamedBufferStorage(
            self.id,
            length,
            ptr::null(),
            flags
        );
    } }
    pub fn upload<T>(&self, data: &[T], length: isize, usage: gl::types::GLenum) { unsafe {
        gl::NamedBufferData(
            self.id,
            length,
            data.as_ptr() as *const c_void, usage
        );
    } }
    pub fn upload_slice<T>(&self, data: &[T], buffer_start: isize, length: isize) { unsafe {
        gl::NamedBufferSubData(
            self.id,
            buffer_start,
            length,
            data.as_ptr() as *const c_void
        );
    } }
    pub fn get_sub_data<T: Sized + Clone + Default>(&self, offset: isize, length: isize) -> Vec<T> { unsafe {
        let mut data = vec![Default::default(); (length as usize) * std::mem::size_of::<T>()];
        gl::GetNamedBufferSubData(
            self.id,
            offset,
            length,
            data.as_mut_ptr() as *mut c_void
        );
        return data;
    } }
    pub fn unbind(target: u32) { unsafe {
        gl::BindBuffer(gl::NONE, target);
    } }
    pub fn fake() -> Buffer {
        return Buffer { id: u32::MAX };
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
                let mut length: i32 = 4096;
                let mut log: Vec<u8> = vec![0; length as usize];
                gl::GetShaderInfoLog(id, length, &mut length, log.as_mut_ptr() as *mut i8);
    
                panic!("Failed to compile {} shader: {}", if r#type == gl::VERTEX_SHADER { "vertex" } else { "fragment" }, std::str::from_utf8(&log).unwrap())
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
        }
    } }

    pub fn bind(&self) { unsafe {
        gl::UseProgram(self.id);
    } }

    pub fn uniform1i(&self, location: i32, v0: i32) { unsafe {
        gl::ProgramUniform1i(self.id, location, v0);
    } }
}

pub struct WindowStatus {
    pub fill_mode: gl::types::GLenum,
    pub maximized: bool,
    pub width: i32,
    pub height: i32,
    pub mouse_captured: bool
}

impl WindowStatus {
    pub fn new() -> WindowStatus {
        return WindowStatus { fill_mode: gl::FILL, maximized: false, mouse_captured: false, width: 600, height: 600 }
    }
}

#[derive(Debug)]
pub struct BufferPoolAllocator<const S: usize, const P: usize> {
    pub buffer: Buffer,
    pub staging_buffer: Buffer,
    pub pages: Box<[bool; S]>,
    pub furthest: usize,
}

impl <const S: usize, const P: usize> BufferPoolAllocator<S, P> {
    pub fn new() -> BufferPoolAllocator<S, P> { unsafe {
        let buffer = Buffer::create();
        buffer.storage((P * S) as isize, gl::DYNAMIC_STORAGE_BIT);
        let staging_buffer = Buffer::create();
        staging_buffer.storage(1024 * 1024, gl::DYNAMIC_STORAGE_BIT);
        return BufferPoolAllocator {
            buffer: buffer,
            staging_buffer: staging_buffer,
            pages: Box::<[bool; S]>::new_zeroed().assume_init(),
            furthest: 0
        }
    } }
    // Size in bytes
    pub fn allocate(&mut self, size: usize) -> Option<Page> { unsafe {
        if size == 0 { return None; }
        let size = size.div_ceil(P);

        let mut run = 0;
        let mut start = 0;
        for i in 0..S {
            if !self.pages[i] {
                if run == 0 { start = i; }
                run += 1;
            }
            if run == size {
                for j in start..(start + size) {
                    self.pages[j] = true;
                }
                if start + size > self.furthest {
                    self.furthest = start + size;
                }
                return Some(Page {
                    start: start,
                    size: NonZeroUsize::new_unchecked(size),
                });
            }
        }
        return None;
    } }
    pub fn deallocate(&mut self, page: Option<Page>) {
        if let Some(page) = page {
            for i in page.start..(page.start + page.size.get()) {
                self.pages[i] = false;
            }
        }
    }

    pub fn upload_slice<T>(&mut self, page: Page, data: &[T], start: isize, length: isize) {
        self.buffer.upload_slice(data, (page.start * P + start as usize) as isize, length);
    }
    pub fn upload<T>(&mut self, page: &Page, data: &[T], length: isize) { unsafe {
        if length > (page.size.get() * P) as isize {
            panic!("exceeded allocation size");
        }
        self.staging_buffer.upload_slice(data, 0, length);

        gl::CopyNamedBufferSubData(self.staging_buffer.id, self.buffer.id, 0, (page.start * P) as isize, length as isize);
        // self.buffer.upload_slice (data, (page.start * 1024) as isize, length);
    } }
}

#[derive(Debug)]
pub struct Page {
    pub start: usize,
    pub size: NonZeroUsize,
}