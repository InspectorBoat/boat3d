use cgmath::{Matrix4, Vector3};

pub struct Rasterizer {
    pub framebuffer: Box<[bool]>,
    pub width: usize,
    pub height: usize,
}

// impl Rasterizer {
//     pub fn rasterize(bounding_box: &mut BoundingBox<f32>, matrix: Matrix4<f32>, camera_pos: Vector3<f32>) {
//         let intersect_x = bounding_box.min.x <= camera_pos.x && camera_pos.x <= bounding_box.max.x;
//         let intersect_y = bounding_box.min.y <= camera_pos.y && camera_pos.y <= bounding_box.max.y;
//         let intersect_z = bounding_box.min.z <= camera_pos.z && camera_pos.z <= bounding_box.max.z;
//         let visible_faces = 3 - intersect_x as u8 - intersect_y as u8 - intersect_z as u8;

//         // let outer_points = [Vector3]
//     }
// }