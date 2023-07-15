use core::str;
use std::f32::consts::PI;

use frustum_query::frustum::Frustum;
use ultraviolet::{Mat4, Vec3, projection::perspective_gl};
#[derive(Debug, Clone, Copy)]
pub struct Rot {
    pub yaw: f32,
    pub pitch: f32
}
#[derive(Debug)]
pub struct Camera {
    pub prev_mouse: (f64, f64),
    pub ratio: f32,
    pub camera_pos: Vec3,
    pub camera_rot: Rot,
    pub frustum_pos: Vec3,
    pub frustum_rot: Rot,
    pub frustum_frozen: bool,
}
impl Camera {
    pub fn get_matrix(&mut self) -> Mat4 {
        let perspective = perspective_gl(PI / 2.0, self.ratio, 0.1, 16000.0);
        let modelview =
            Mat4::from_rotation_x(PI + self.camera_rot.pitch)
            * Mat4::from_rotation_y(- self.camera_rot.yaw)
            * Mat4::from_translation(Vec3::new(-self.camera_pos.x, self.camera_pos.y, -self.camera_pos.z))
            * Mat4::from_nonuniform_scale(Vec3{
                x: 1.0,
                y: -1.0,
                z: 1.0
            });

        
        return perspective * modelview;
    }

    pub fn get_frustum(&mut self) -> Frustum {
        if !self.frustum_frozen {
            self.frustum_pos = self.camera_pos;
            self.frustum_rot = self.camera_rot;
        }
        let modelview = 
            Mat4::from_rotation_x(PI + self.frustum_rot.pitch)
            * Mat4::from_rotation_y(- self.frustum_rot.yaw)
            * Mat4::from_nonuniform_scale(Vec3{
                x: 1.0,
                y: -1.0,
                z: 1.0
            });
        let projection = Mat4::identity()
            * perspective_gl(PI / 2.0, self.ratio, 0.1, 16000.0);
        return frustum_query::frustum::Frustum::from_modelview_and_projection(modelview.as_array(), projection.as_array());
    }

    pub fn step(&mut self, x: f64, y: f64, z: f64) {
        self.camera_pos.x += ((self.camera_rot.yaw.cos() as f64) * x - (self.camera_rot.yaw.sin() as f64) * z) as f32;
        self.camera_pos.z -= ((self.camera_rot.yaw.sin() as f64) * x + (self.camera_rot.yaw.cos() as f64) * z) as f32;
        self.camera_pos.y += y as f32;
    }
    
    pub fn new() -> Camera {
        Camera { 
            prev_mouse: (f64::MAX, f64::MAX),
            ratio: 1.0,
            camera_pos: Vec3 { x: 0.0, y: 0.0, z: 16.0 * 11.0 * 0.0 },
            camera_rot: Rot { pitch: 0.0, yaw: 0.0 },
            frustum_pos: Vec3 { x: 0.0, y: 0.0, z: 16.0 * 11.0 * 0.0 },
            frustum_rot: Rot { pitch: 0.0, yaw: 0.0 },
            frustum_frozen: false
        }
    }
}