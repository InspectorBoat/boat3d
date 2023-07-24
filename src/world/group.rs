use cgmath::Vector3;
use cgmath_culling::BoundingBox;

use crate::Section;

use super::{camera::Camera, world::SectionType};

pub struct Group {
    pub sections: [SectionType; 4 * 4 * 4],
    pub pos: Vector3<i32>,
    pub present: u8,
}

impl Group {
    pub fn get_section(&mut self, pos: Vector3<i32>) -> &mut SectionType {
        return &mut self.sections[Group::index(pos.x % 4, pos.y % 4, pos.z % 4)];
    }

    pub fn set_block(&mut self, section_pos: Vector3<i32>, block_pos: Vector3<i32>, block_id: u16) {
        let section = self.get_section(section_pos);
        // if let Some(section) = section {
        //     section.blocks[Section::index(block_pos.x, block_pos.y, block_pos.z)] = block_id;
        // }
    }

    pub fn get_bounding_box(&self, camera: &Camera) -> BoundingBox<f32> {
        return BoundingBox {
            min: Vector3 {
                x: (self.pos.x * 256) as f32 - camera.frustum_pos.x,
                y: (self.pos.y * 256) as f32 - camera.frustum_pos.y,
                z: (self.pos.z * 256) as f32 - camera.frustum_pos.z,
            },
            max: Vector3 {
                x: (self.pos.x * 256) as f32 - camera.frustum_pos.x + 256.0 * 4.0,
                y: (self.pos.y * 256) as f32 - camera.frustum_pos.y + 256.0 * 4.0,
                z: (self.pos.z * 256) as f32 - camera.frustum_pos.z + 256.0 * 4.0,
            }
        };
    }

    #[inline]
    pub fn index<T: TryInto<usize>>(x: T, y: T, z: T) -> usize { unsafe {
        return (x.try_into().unwrap_unchecked() << 4) |
               (y.try_into().unwrap_unchecked() << 2) |
               (z.try_into().unwrap_unchecked() << 0);
    } }
}