use std::{hint::unreachable_unchecked, mem, ffi::c_void};

use cgmath::{Matrix4, Vector3, InnerSpace, Vector4, Vector2};
use core_simd::simd::Simd;
use gl::OR;

use crate::{cull::math::IntoVec4, gl_util::{texture::Texture, gl_wrapper, gl_helper::log_error}};

use super::{bounding_box::LocalBoundingBox, frustum_cull::SimdMul};

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
        // texture.bind(gl_wrapper::TEXTURE_2D);
        // log_error();
        // gl_wrapper::TexParameteri(gl_wrapper::TEXTURE_2D, gl_wrapper::TEXTURE_WRAP_S, gl_wrapper::REPEAT as i32);
        // gl_wrapper::TexParameteri(gl_wrapper::TEXTURE_2D, gl_wrapper::TEXTURE_WRAP_T, gl_wrapper::REPEAT as i32);
        // gl_wrapper::TexParameteri(gl_wrapper::TEXTURE_2D, gl_wrapper::TEXTURE_MIN_FILTER, gl_wrapper::NEAREST as i32);
        // gl_wrapper::TexParameteri(gl_wrapper::TEXTURE_2D, gl_wrapper::TEXTURE_MAG_FILTER, gl_wrapper::NEAREST as i32);
        // gl_wrapper::TextureStorage2D(texture.id, 1, gl_wrapper::RGBA8, width as i32, height as i32);
        // log_error();
        return Rasterizer {
            buffer: [0].repeat(width * height).into_boxed_slice(),
            width: width,
            height: height,
            texture: texture
        }
    } }
    pub fn rasterize(&mut self, bounding_box: &mut LocalBoundingBox, matrix: Matrix4<f32>, camera_pos: Vector3<f32>) { unsafe {
        let intersect_x = bounding_box.min.x <= camera_pos.x && camera_pos.x <= bounding_box.max.x;
        let intersect_y = bounding_box.min.y <= camera_pos.y && camera_pos.y <= bounding_box.max.y;
        let intersect_z = bounding_box.min.z <= camera_pos.z && camera_pos.z <= bounding_box.max.z;
        let visible_faces = 3 - intersect_x as u8 - intersect_y as u8 - intersect_z as u8;

        let out_vertices: [Vector3<f32>; 6];
        match visible_faces {
            3 => {
                let mut vertices = bounding_box.vertices();
                vertices.sort_unstable_by(|a, b| a.magnitude2().total_cmp(&b.magnitude2()));
                out_vertices = vertices[1..7].try_into().unwrap();
            }
            2 => {
                let mut vertices = bounding_box.vertices();
                vertices.sort_unstable_by(|a, b| a.magnitude2().total_cmp(&b.magnitude2()));
                out_vertices = vertices[0..6].try_into().unwrap();
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
        let out_vertices: [Vector2<i32>; 6] = out_vertices.map(|point| matrix.simd_mul(&point).into_vec4()).map(|point| Vector2 { x: point.x / point.w, y: point.y / point.w }.cast().unwrap());

        let min_x = *out_vertices.map(|point| point.x).iter().min().unwrap();
        let max_x = *out_vertices.map(|point| point.x).iter().max().unwrap();

        let min_y = *out_vertices.map(|point| point.y).iter().min().unwrap();
        let max_y = *out_vertices.map(|point| point.y).iter().max().unwrap();

        for x in min_x..max_x {
            for y in min_y..max_y {
                if Rasterizer::within(Vector2 { x: x, y: y }, out_vertices) {
                    self.draw_point(Vector2 { x: x, y: y });
                }
            }
        }
        for x in 0..10 {
            for y in 0..10 {
                self.draw_point(Vector2 { x: x, y: y });
            }
        }
    } }
    pub fn within(point: Vector2<i32>, out_vertices: [Vector2<i32>; 6]) -> bool {
        for (vertex_a, vertex_b) in out_vertices.iter().enumerate().map(|(i, vertex)| (*vertex, out_vertices[(i + 1) % 6])) {
            if Rasterizer::orient(point, vertex_a, vertex_b) <= 0 {
                return false;
            }
        }
        return true;
    }
    pub fn orient(point: Vector2<i32>, vertex_a: Vector2<i32>, vertex_b: Vector2<i32>) -> i32 {
        return (vertex_a.x - point.x) * (vertex_b.y - point.y) - (vertex_a.y - point.y) * (vertex_b.x - point.x);
    }
    pub fn draw_point(&mut self, point: Vector2<i32>) {
        self.buffer[point.x as usize * self.width + point.y as usize] = 255;
    }
    pub fn render_to_texture(&self) { unsafe {
        gl_wrapper::TextureSubImage2D(self.texture.id, 0, 0, 0, self.width as i32, self.height as i32, gl_wrapper::RED, gl_wrapper::BYTE, self.buffer.as_ptr() as *const c_void);
    } }
}
