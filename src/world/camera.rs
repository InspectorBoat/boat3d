use std::f32::consts::PI;

use cgmath::{Matrix4, PerspectiveFov, Rad, Vector3, Euler, Deg, Angle};
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
    pub camera_euler: Euler<Rad<f32>>,

    pub frustum_pos: Vector3<f32>,
    pub frustum_euler: Euler<Rad<f32>>,

    pub frustum_frozen: bool,
}
impl Camera {
    pub fn get_camera_matrix(&self) -> Matrix4<f32> {
        let perspective = Matrix4::from(PerspectiveFov {
            fovy: Rad(PI / 2.0),
            aspect: self.ratio,
            near: 0.1,
            far: 48000.0,
        });
        let modelview =
            Matrix4::from_angle_x(Rad(PI) + self.camera_euler.x)
            * Matrix4::from_angle_y(- self.camera_euler.y)
            * Matrix4::from_translation(Vector3 { x: -self.camera_pos.x, y: self.camera_pos.y, z: -self.camera_pos.z })
            * Matrix4::from_nonuniform_scale(1.0, -1.0, 1.0);
        return perspective * modelview;
    }

    pub fn get_frustum_matrix(&self) -> Matrix4<f32> {
        let perspective = Matrix4::from(PerspectiveFov {
            fovy: Rad(PI / 2.0),
            aspect: self.ratio,
            near: 0.1,
            far: 48000.0,
        });
        let modelview =
            Matrix4::from_angle_x(Rad(PI) + self.camera_euler.x)
            * Matrix4::from_angle_y(-self.camera_euler.y)
            * Matrix4::from_translation(Vector3 { x: -self.frustum_pos.x, y: self.frustum_pos.y, z: -self.frustum_pos.z })
            * Matrix4::from_nonuniform_scale(1.0, -1.0, 1.0);
        return perspective * modelview;
    }

    pub fn get_frustum(&self) -> FrustumCuller<f32> {
        let perspective = Matrix4::from(PerspectiveFov {
            fovy: Rad(PI / 2.0),
            aspect: self.ratio,
            near: 0.1,
            far: 48000.0,
        });
        
        let modelview = 
            Matrix4::from_angle_x(Rad(PI) + self.frustum_euler.x)
            * Matrix4::from_angle_y(-self.frustum_euler.y)
            * Matrix4::from_nonuniform_scale(1.0, -1.0, 1.0);
        return FrustumCuller::from_matrix(perspective * modelview);
    }

    pub fn step(&mut self, x: f64, y: f64, z: f64) {
        self.camera_pos.x += ((self.camera_euler.y.cos() as f64) * x - (self.camera_euler.y.sin() as f64) * z) as f32;
        self.camera_pos.z -= ((self.camera_euler.y.sin() as f64) * x + (self.camera_euler.y.cos() as f64) * z) as f32;
        self.camera_pos.y += y as f32;
    }
    
    pub fn new() -> Camera {
        return Camera { 
            prev_mouse: (f64::MAX, f64::MAX),
            ratio: 1.0,

            camera_pos: Vector3 { x: 0.0, y: 0.0, z: -256.0 * 0.0 },
            camera_euler: Euler { x: Rad(0.0), y: Rad(0.0), z: Rad(0.0) },

            frustum_pos: Vector3 { x: 0.0, y: 0.0, z: -256.0 * 0.0 },
            frustum_euler: Euler { x: Rad(0.0), y: Rad(0.0), z: Rad(0.0) },

            frustum_frozen: false
        };
        // Camera {
        //     prev_mouse: (-1193.0, -110.0),
        //     ratio: 1.0,
        //     camera_pos: Vector3 { x: -465.77737, y: 0.0, z: -670.4167 },
        //     camera_rot: Rot { yaw: 176.76808, pitch: -0.7279996 },
        //     frustum_pos: Vector3 { x: -465.77737, y: 0.0, z: -670.4167 },
        //     frustum_rot: Rot { yaw: 176.76808, pitch: -0.7279996 },
        //     frustum_frozen: false
        // }
    }
}