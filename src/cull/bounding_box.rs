use cgmath::Vector3;
use core_simd::simd::Simd;

use super::math::f32x3;

/// Relative to the camera position
pub struct SimdLocalBoundingBox {
    pub min: f32x3,
    pub max: f32x3,
}

pub struct LocalBoundingBox {
    pub min: Vector3<f32>,
    pub max: Vector3<f32>,
}

impl From<LocalBoundingBox> for SimdLocalBoundingBox {
    fn from(value: LocalBoundingBox) -> Self {
        return SimdLocalBoundingBox {
            min: Simd::from_array([value.min.x, value.min.y, value.min.z]),
            max: Simd::from_array([value.max.x, value.max.y, value.max.z]),
        }
    }
}

impl From<SimdLocalBoundingBox> for LocalBoundingBox {
    fn from(value: SimdLocalBoundingBox) -> Self {
        return LocalBoundingBox {
            min: Vector3 { x: value.min[0], y: value.min[1], z: value.min[2] },
            max: Vector3 { x: value.max[0], y: value.max[1], z: value.max[2] },
        }
    }
}


impl LocalBoundingBox {
    pub fn vertices(&self) -> [Vector3<f32>; 8] {
        return [
            Vector3 { x: self.min.x, y: self.min.y, z: self.min.z },
            Vector3 { x: self.min.x, y: self.min.y, z: self.max.z },
            Vector3 { x: self.min.x, y: self.max.y, z: self.min.z },
            Vector3 { x: self.min.x, y: self.max.y, z: self.max.z },
            Vector3 { x: self.max.x, y: self.min.y, z: self.min.z },
            Vector3 { x: self.max.x, y: self.min.y, z: self.max.z },
            Vector3 { x: self.max.x, y: self.max.y, z: self.min.z },
            Vector3 { x: self.max.x, y: self.max.y, z: self.max.z },
        ];
    }
}