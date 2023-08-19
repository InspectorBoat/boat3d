use std::f32::consts::PI;

use cgmath::{Matrix4, PerspectiveFov, Rad, Vector3, Euler, Deg, Angle};

use crate::cull::frustum::LocalFrustum;
#[derive(Debug, Clone, Copy)]
pub struct Rot {
    pub yaw: f32,
    pub pitch: f32
}

#[derive(Debug)]
pub struct Camera {
    pub prev_mouse: (f64, f64),
    pub aspect: f32,
    
    pub camera_pos: Vector3<f32>,
    pub camera_rot: Euler<Rad<f32>>,

    pub frustum_pos: Vector3<f32>,
    pub frustum_rot: Euler<Rad<f32>>,

    pub frustum_frozen: bool,

    pub window_width: i32,
    pub window_height: i32
}
impl Camera {
    pub fn get_camera_matrix(&self) -> Matrix4<f32> {
        let perspective = Matrix4::from(PerspectiveFov {
            fovy: Camera::FOVY,
            aspect: self.aspect,
            near: Camera::NEAR_PLANE,
            far: Camera::FAR_PLANE,
        });

        let modelview =
            Matrix4::from_angle_x(Rad(PI) + self.camera_rot.x)
            * Matrix4::from_angle_y(-self.camera_rot.y)
            * Matrix4::from_angle_z(self.camera_rot.z)
            * Matrix4::from_translation(Vector3 { x: self.camera_pos.x, y: self.camera_pos.y, z: -self.camera_pos.z })
            * Matrix4::from_nonuniform_scale(-1.0, -1.0, 1.0);
        return perspective * modelview;
    }

    pub fn get_local_camera_matrix(&self) -> Matrix4<f32> {
        let perspective = Matrix4::from(PerspectiveFov {
            fovy: Camera::FOVY,
            aspect: self.aspect,
            near: Camera::NEAR_PLANE,
            far: Camera::FAR_PLANE,
        });

        let modelview =
            Matrix4::from_angle_x(Rad(PI) + self.camera_rot.x)
            * Matrix4::from_angle_y(-self.camera_rot.y)
            * Matrix4::from_angle_z(self.camera_rot.z)
            * Matrix4::from_nonuniform_scale(-1.0, -1.0, 1.0);
        return perspective * modelview;
    }

    pub fn get_frustum_matrix(&self) -> Matrix4<f32> {
        let perspective = Matrix4::from(PerspectiveFov {
            fovy: Camera::FOVY,
            aspect: self.aspect,
            near: Camera::NEAR_PLANE,
            far: Camera::FAR_PLANE,
        });
        
        let modelview = 
            Matrix4::from_angle_x(Rad(PI) + self.frustum_rot.x)
            * Matrix4::from_angle_y(-self.frustum_rot.y)
            * Matrix4::from_angle_z(self.frustum_rot.z)
            * Matrix4::from_nonuniform_scale(-1.0, -1.0, 1.0);
        return perspective * modelview;
    }
    
    pub fn get_frustum(&self) -> LocalFrustum {
        return LocalFrustum::from_matrix(self.get_frustum_matrix());
    }

    pub fn step(&mut self, x: f64, y: f64, z: f64) {
        self.camera_pos.x -= ((self.camera_rot.y.cos() as f64) * x - (self.camera_rot.y.sin() as f64) * z) as f32;
        self.camera_pos.z -= ((self.camera_rot.y.sin() as f64) * x + (self.camera_rot.y.cos() as f64) * z) as f32;
        self.camera_pos.y += y as f32;
    }
    
    pub fn new() -> Camera {
        // return Camera { 
        //     prev_mouse: (f64::MAX, f64::MAX),
        //     aspect: 1.0,

        //     camera_pos: Vector3 { x: 0.0, y: 0.0, z: -256.0 * 0.0 },
        //     camera_rot: Euler { x: Rad(0.0), y: Rad(0.0), z: Rad(0.0) },

        //     frustum_pos: Vector3 { x: 0.0, y: 0.0, z: -256.0 * 0.0 },
        //     frustum_rot: Euler { x: Rad(0.0), y: Rad(0.0), z: Rad(0.0) },

        //     frustum_frozen: false,
            
        //     window_width: 600,
        //     window_height: 600,
        // };
        return Camera {
            prev_mouse: (-95.0, -41.0),
            aspect: 1.0,

            camera_pos: Vector3 { x: -7.324981, y: -7.35001, z: -6.9660172 },
            camera_rot: Euler { x: Rad(-0.7160001), y: Rad(-0.8439991), z: Rad(0.0) },
            
            frustum_pos: Vector3 { x: -7.324981, y: -7.35001, z: -6.9660172 },
            frustum_rot: Euler { x: Rad(-0.7160001), y: Rad(-0.8439991), z: Rad(0.0) },
            
            frustum_frozen: false,
            
            window_width: 600,
            window_height: 600
        };
    }
    pub const NEAR_PLANE: f32 = 0.1;
    pub const FAR_PLANE: f32 = 48000.0;
    pub const FOVY: Rad<f32> = Rad(PI / 2.0);
}