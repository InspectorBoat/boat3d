#![feature(portable_simd)]
use std::simd::{SimdFloat, StdFloat, SimdPartialOrd};
use core::simd::Simd;
use std::mem;

#[derive(Clone)]
#[repr(C, align(16))]
pub struct Vec4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32
}
impl Vec4 {
    fn as_array(&self) -> &[f32; 4] { unsafe {
        return mem::transmute(self);
    } }
}

#[derive(Clone)]
#[repr(C, align(16))]
pub struct Mat4(pub [Vec4; 4]);
impl Mat4 {
    fn row(&self, row: usize) -> &Vec4 {
        return &self.0[row];
    }
    // this one requires the matrix to be transposed
    fn mul_reduce(&self, vec: &Vec4) -> Vec4 { unsafe {
        let row_0 = Simd::from_array(*self.row(0).as_array());
        let row_1 = Simd::from_array(*self.row(1).as_array());
        let row_2 = Simd::from_array(*self.row(2).as_array());
        let row_3 = Simd::from_array(*self.row(3).as_array());
        let vec = Simd::from_array(*vec.as_array());
        return Vec4 {
            x: (row_0 * vec).reduce_sum(),
            y: (row_1 * vec).reduce_sum(),
            z: (row_2 * vec).reduce_sum(),
            w: (row_3 * vec).reduce_sum(),
        }
    } }
    // uses simd fma operations
    fn mul_fma(&self, vec: &Vec4) -> Simd<f32, 4> { unsafe {
        let row_0 = Simd::from_array(*self.row(0).as_array());
        let row_1 = Simd::from_array(*self.row(1).as_array());
        let row_2 = Simd::from_array(*self.row(2).as_array());
        let row_3 = Simd::from_array(*self.row(3).as_array());
        let mut sum = Simd::splat(0.0);
        sum = row_0.mul_add(Simd::splat(vec.x), sum);
        sum = row_1.mul_add(Simd::splat(vec.y), sum);
        sum = row_2.mul_add(Simd::splat(vec.z), sum);
        sum = row_3.mul_add(Simd::splat(vec.w), sum);
        return sum;
    } }
    // uses unfused operations
    fn mul_unfma(&self, vec: &Vec4) -> Simd<f32, 4> { unsafe {
        let row_0 = Simd::from_array(*self.row(0).as_array());
        let row_1 = Simd::from_array(*self.row(1).as_array());
        let row_2 = Simd::from_array(*self.row(2).as_array());
        let row_3 = Simd::from_array(*self.row(3).as_array());
        let mut sum = Simd::splat(0.0);
        sum = row_0 * Simd::splat(vec.x) + sum;
        sum = row_1 * Simd::splat(vec.y) + sum;
        sum = row_2 * Simd::splat(vec.z) + sum;
        sum = row_3 * Simd::splat(vec.w) + sum;
        return sum;
    } }
}

pub fn frustum_cull(center_point: Vec4, outer_matrix: Mat4, inner_matrix: Mat4) -> u8 {
    let outer_projection = outer_matrix.mul_fma(&center_point);
    let outer_fail = outer_projection.abs().simd_gt(Simd::splat(outer_projection[3].abs())).any();
    let inner_projection = inner_matrix.mul_fma(&center_point);
    let inner_fail = inner_projection.abs().simd_gt(Simd::splat(inner_projection[3].abs())).any();
    return 2 - outer_fail as u8 - inner_fail as u8;
}

fn frustum_cull_slow(center_point: Vec4, outer_matrix: Mat4, inner_matrix: Mat4) -> u8 { unsafe {
    let outer_projection = outer_matrix.mul_fma(&center_point);
    let inner_projection = inner_matrix.mul_fma(&center_point);
    let outer_projection = mem::transmute::<_, Vec4>(outer_projection);
    let inner_projection = mem::transmute::<_, Vec4>(inner_projection);
    if (outer_projection.x < -outer_projection.w) | (outer_projection.x > outer_projection.w) |
       (outer_projection.y < -outer_projection.w) | (outer_projection.y > outer_projection.w) |
       (outer_projection.z < -outer_projection.w) | (outer_projection.z > outer_projection.w) {
        // completely outside
        return 0;
    }

    if (inner_projection.x < -inner_projection.w) | (inner_projection.x > inner_projection.w) |
       (inner_projection.y < -inner_projection.w) | (inner_projection.y > inner_projection.w) |
       (inner_projection.z < -inner_projection.w) | (inner_projection.z > inner_projection.w) {
        // partially inside
        return 1;
    }

    // completely inside
    return 2;
} }