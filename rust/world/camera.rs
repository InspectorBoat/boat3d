use core::str;
use std::f32::consts::PI;

use ultraviolet::{Mat4, Vec3, projection::perspective_gl};
#[derive(Debug)]
pub struct Rot {
    pub yaw: f32,
    pub pitch: f32
}
#[derive(Debug)]
pub struct Camera {
    pub prev_mouse: (f64, f64),
    pub ratio: f32,
    pub pos: Vec3,
    pub rot: Rot
}
impl Camera {
    pub fn get_matrix(&mut self) -> Mat4 {
        let mat = Mat4::identity()
        * perspective_gl(PI / 2.0, self.ratio, 0.1, 1000.0)
        * Mat4::from_rotation_x(PI + self.rot.pitch)
        * Mat4::from_rotation_y(- self.rot.yaw)
        * Mat4::from_translation(Vec3::new(-self.pos.x, self.pos.y, -self.pos.z))
        * Mat4::from_nonuniform_scale(Vec3{
            x: 1.0 / 16.0,
            y: -1.0 / 16.0,
            z: 1.0 / 16.0
        })
        ;
        return mat;
    }

    pub fn step(&mut self, x: f64, z: f64) {
        self.pos.x += ((self.rot.yaw.cos() as f64) * x - (self.rot.yaw.sin() as f64) * z) as f32;
        self.pos.z -= ((self.rot.yaw.sin() as f64) * x + (self.rot.yaw.cos() as f64) * z) as f32;
    }
    pub fn new() -> Camera {
        Camera { 
            pos: Vec3 { x: 0.0, y: 0.0, z: 16.0 * 11.0 * 0.0 },
            prev_mouse: (f64::MAX, f64::MAX),
            ratio: 1.0,
            rot: Rot { pitch: 0.0, yaw: 0.0 }
        }
    }
}