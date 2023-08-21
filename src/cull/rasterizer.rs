use std::{hint::unreachable_unchecked, mem, ffi::c_void};

use cgmath::{Matrix4, Vector3, InnerSpace, Vector4, Vector2};
use core_simd::simd::Simd;
use gl::OR;

use crate::{cull::math::IntoVec4, gl_util::{texture::Texture, gl_wrapper, gl_helper::log_error}};

use super::{bounding_box::BoundingBox, frustum_cull::SimdMul};

#[derive(Debug)]
pub struct Rasterizer {
    pub buffer: Box<[u8]>,
    pub width: usize,
    pub height: usize,
    pub texture: Texture
}

impl Rasterizer {
    pub fn new(width: usize, height: usize) -> Rasterizer { unsafe {
        let texture = Texture::create(gl_wrapper::TEXTURE_2D);
        texture.storage_2d(1, gl_wrapper::RGBA8, width as i32, height as i32);
        texture.parameter_signed_integer(gl_wrapper::TEXTURE_WRAP_S, gl_wrapper::REPEAT as i32);
        texture.parameter_signed_integer(gl_wrapper::TEXTURE_WRAP_T, gl_wrapper::REPEAT as i32);
        texture.parameter_signed_integer(gl_wrapper::TEXTURE_MIN_FILTER, gl_wrapper::NEAREST as i32);
        texture.parameter_signed_integer(gl_wrapper::TEXTURE_MAG_FILTER, gl_wrapper::NEAREST as i32);
        return Rasterizer {
            buffer: [0].repeat(width * height).into_boxed_slice(),
            width: width,
            height: height,
            texture: texture
        }
    } }
    pub fn clear(&mut self) {
        for b in self.buffer.iter_mut() {
            *b = 0;
        }
    } 
    pub fn rasterize(&mut self, local_bounding_box: &mut BoundingBox, local_matrix: Matrix4<f32>) { unsafe {
        let intersect_x = local_bounding_box.min.x <= 0.0 && 0.0 <= local_bounding_box.max.x;
        let intersect_y = local_bounding_box.min.y <= 0.0 && 0.0 <= local_bounding_box.max.y;
        let intersect_z = local_bounding_box.min.z <= 0.0 && 0.0 <= local_bounding_box.max.z;
        let visible_faces = 3 - intersect_x as u8 - intersect_y as u8 - intersect_z as u8;

        let out_vertices: [Vector3<f32>; 6];
        let middle_point: Vector3<f32>;
        match visible_faces {
            3 => {
                let mut vertices = local_bounding_box.vertices();
                vertices.sort_unstable_by(|a, b| (a.x.abs() + a.y.abs() + a.z.abs()).total_cmp(&(b.x.abs() + b.y.abs() + b.z.abs())));
                out_vertices = vertices[1..7].try_into().unwrap();
                middle_point = vertices[0];
            }
            2 => {
                let mut vertices = local_bounding_box.vertices();
                vertices.sort_unstable_by(|a, b| (a.x.abs() + a.y.abs() + a.z.abs()).total_cmp(&(b.x.abs() + b.y.abs() + b.z.abs())));
                out_vertices = vertices[0..6].try_into().unwrap();
                middle_point = vertices[6];
            }
            1 => {
                // let mut vertices = bounding_box.vertices();
                // vertices.sort_unstable_by(|a, b| a.magnitude2().total_cmp(&b.magnitude2()));
                // let out_points: [Vector3<f32>; 4] = vertices[0..4].try_into().unwrap();
                // let out_points = out_points.map(|point| matrix.simd_mul(&point).into_vec4()).map(|point| point / point.w);
                // println!("{out_points:#?}");
                return;
            }
            _ => { return; }
        }

        // multiplies by matrix, converts into vec4, normalized homogeneous coordinates, converts into integer vector
        let mut out_vertices: [Vector2<i32>; 6] = out_vertices
            .map(|point| local_matrix.simd_mul(&point).into_vec4())
            .map(|point| Vector2 {
                x: (point.x / point.w * self.width as f32 / 2.0) as i32 + self.width as i32 / 2,
                y: -(point.y / point.w * self.height as f32 / 2.0) as i32 + self.height as i32 / 2
            });
        let middle_point = local_matrix.simd_mul(&middle_point).into_vec4();
        let middle_point = Vector2 {
            x: (middle_point.x / middle_point.w * self.width as f32 / 2.0) as i32 + self.width as i32 / 2,
            y: -(middle_point.y / middle_point.w * self.height as f32 / 2.0) as i32 + self.height as i32 / 2
        };
        out_vertices.sort_unstable_by(|a, b| {
            (a - middle_point).cast::<f32>().unwrap().angle(Vector2 { x: 0.0, y: 1.0 })
                .partial_cmp(&(b - middle_point).cast::<f32>().unwrap().angle(Vector2 { x: 0.0, y: 1.0 }))
                .unwrap()
        });

        let min_x = *out_vertices.map(|point| point.x).iter().min().unwrap().max(&0);
        let max_x = *out_vertices.map(|point| point.x).iter().max().unwrap().min(&(self.width as i32));
        
        let min_y = *out_vertices.map(|point| point.y).iter().min().unwrap().max(&0);
        let max_y = *out_vertices.map(|point| point.y).iter().max().unwrap().min(&(self.height as i32));

        for point in out_vertices {
            self.draw_point(Vector2 { x: point.x + 0, y: point.y + 0 });
            self.draw_point(Vector2 { x: point.x + 0, y: point.y + 1 });
            self.draw_point(Vector2 { x: point.x + 1, y: point.y + 0 });
            self.draw_point(Vector2 { x: point.x + 1, y: point.y + 1 });
        }
        
        // for x in min_x..max_x {
        //     for y in min_y..max_y {
        //         if Rasterizer::within(Vector2 { x: x, y: y }, out_vertices) {
        //             self.draw_point(Vector2 { x: x, y: y });
        //         }
        //     }
        // }
    } }
    pub fn within(point: Vector2<i32>, out_vertices: [Vector2<i32>; 6]) -> bool {
        for (vertex_a, vertex_b) in out_vertices.iter().enumerate().map(|(i, vertex)| (*vertex, out_vertices[(i + 1) % 6])) {
            if Rasterizer::orient(point, vertex_a, vertex_b) > 0 {
                return false;
            }
        }
        return true;
    }
    pub fn orient(point: Vector2<i32>, vertex_a: Vector2<i32>, vertex_b: Vector2<i32>) -> i32 {
        let a = vertex_a - point;
        let b = vertex_b - vertex_a;
        return a.perp_dot(b);
        // return (vertex_a.x - point.x) * (vertex_b.y - point.y) - (vertex_a.y - point.y) * (vertex_b.x - point.x);
    }
    pub fn draw_point(&mut self, point: Vector2<i32>) {
        if point.x <= 0 || point.x >= self.width as i32 || point.y <= 0 || point.y >= self.height as i32 {
            return;
        }
        self.buffer[point.x as usize + (self.height - point.y as usize - 1) * self.width] = 255;
    }
    pub fn render_to_texture(&self) { unsafe {
        let image: [u8; 100 * 100] = [0; 100 * 100].map(|_| rand::random::<u8>());
        // self.texture.sub_image_2d(0, 0, 0, 100, 100, gl_wrapper::RED, gl_wrapper::UNSIGNED_BYTE, &raw const image as *const c_void);
        self.texture.sub_image_2d(0, 0, 0, 600, 600, gl_wrapper::RED, gl_wrapper::UNSIGNED_BYTE, self.buffer.as_ptr() as *const c_void);
    } }
    pub fn kill(self) {
        self.texture.kill();
    }
}
