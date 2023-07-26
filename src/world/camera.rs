use core::str;
use std::f32::consts::PI;

use cgmath::{Matrix4, Perspective, PerspectiveFov, Rad, Vector3};
use cgmath_culling::FrustumCuller;
#[derive(Debug, Clone, Copy)]
pub struct Rot {
    pub yaw: f32,
    pub pitch: f32
}
#[derive(Debug)]
pub struct Camera {
    pub prev_mouse: (f64, f64),
    pub ratio: f32,
    pub camera_pos: Vector3<f32>,
    pub camera_rot: Rot,
    pub frustum_pos: Vector3<f32>,
    pub frustum_rot: Rot,
    pub frustum_frozen: bool,
}
impl Camera {
    pub fn get_matrix(&mut self) -> Matrix4<f32> {
        let perspective = Matrix4::from(PerspectiveFov {
            fovy: Rad(PI / 2.0),
            aspect: self.ratio,
            near: 0.1,
            far: 48000.0,
        });
        let modelview =
            Matrix4::from_angle_x(Rad(PI + self.camera_rot.pitch))
            * Matrix4::from_angle_y(Rad(- self.camera_rot.yaw))
            * Matrix4::from_translation(Vector3 { x: -self.camera_pos.x, y: self.camera_pos.y, z: -self.camera_pos.z })
            * Matrix4::from_nonuniform_scale(1.0, -1.0, 1.0);
        return perspective * modelview;
    }

    pub fn get_frustum(&mut self) -> FrustumCuller<f32> {
        if !self.frustum_frozen {
            self.frustum_pos = self.camera_pos;
            self.frustum_rot = self.camera_rot;
        }
        let perspective = Matrix4::from(PerspectiveFov {
            fovy: Rad(PI / 2.0),
            aspect: self.ratio,
            near: 0.1,
            far: 48000.0,
        });
        let modelview = 
            Matrix4::from_angle_x(Rad(PI + self.frustum_rot.pitch))
            * Matrix4::from_angle_y(Rad(- self.frustum_rot.yaw))
            * Matrix4::from_nonuniform_scale(1.0, -1.0, 1.0);
        return FrustumCuller::from_matrix(perspective * modelview);
    }

    pub fn step(&mut self, x: f64, y: f64, z: f64) {
        self.camera_pos.x += ((self.camera_rot.yaw.cos() as f64) * x - (self.camera_rot.yaw.sin() as f64) * z) as f32;
        self.camera_pos.z -= ((self.camera_rot.yaw.sin() as f64) * x + (self.camera_rot.yaw.cos() as f64) * z) as f32;
        self.camera_pos.y += y as f32;
    }
    
    pub fn new() -> Camera {
        // return Camera { 
        //     prev_mouse: (f64::MAX, f64::MAX),
        //     ratio: 1.0,
        //     camera_pos: Vector3 { x: 0.0, y: 0.0, z: -256.0 * 2.0 },
        //     camera_rot: Rot { pitch: 0.0, yaw: 180.0 },
        //     frustum_pos: Vector3 { x: 0.0, y: 0.0, z: -256.0 * 2.0 },
        //     frustum_rot: Rot { pitch: 0.0, yaw: 180.0 },
        //     frustum_frozen: false
        // };
        Camera {
            prev_mouse: (-1193.0, -110.0),
            ratio: 1.0,
            camera_pos: Vector3 { x: -465.77737, y: 0.0, z: -670.4167 },
            camera_rot: Rot { yaw: 176.76808, pitch: -0.7279996 },
            frustum_pos: Vector3 { x: -465.77737, y: 0.0, z: -670.4167 },
            frustum_rot: Rot { yaw: 176.76808, pitch: -0.7279996 },
            frustum_frozen: false
        }
    }
}