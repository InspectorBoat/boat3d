use std::sync::mpsc::Receiver;

use glfw::{Window, WindowEvent, Glfw, Context};

use super::gl_wrapper;

pub fn init_gl(window: &mut Window) {
    gl_wrapper::load_with(|s| window.get_proc_address(s) as *const _);
    window.set_all_polling(true);
    window.make_current();
}
pub fn init_glfw() -> Glfw {
    return glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
}
pub fn create_window(status: &WindowStatus) -> (Window, Receiver<(f64, WindowEvent)>) {
    return glfw::init(glfw::FAIL_ON_ERRORS).unwrap().create_window(status.width as u32, status.height as u32, "boat3d", glfw::WindowMode::Windowed)
            .expect("Failed to create GLFW window.");
}
pub fn log_error() {
    match unsafe { gl_wrapper::GetError() } {
        gl_wrapper::INVALID_ENUM => println!("INVALID_ENUM"),
        gl_wrapper::INVALID_VALUE => println!("INVALID_VALUE"),
        gl_wrapper::INVALID_OPERATION => println!("INVALID_OPERATION"),
        0 => println!("NONE"),
        default => println!("{default}")
    }
}
pub fn log_if_error() {
    match unsafe { gl_wrapper::GetError() } {
        0 => return,
        gl_wrapper::INVALID_ENUM => println!("INVALID_ENUM"),
        gl_wrapper::INVALID_VALUE => println!("INVALID_VALUE"),
        gl_wrapper::INVALID_OPERATION => println!("INVALID_OPERATION"),
        default => println!("{default}")
    }
}

pub struct WindowStatus {
    pub fill_mode: u32,
    pub maximized: bool,
    pub width: i32,
    pub height: i32,
    pub mouse_captured: bool
}

impl WindowStatus {
    pub fn new() -> WindowStatus {
        return WindowStatus { fill_mode: gl_wrapper::FILL, maximized: false, mouse_captured: false, width: 600, height: 600 }
    }
}
