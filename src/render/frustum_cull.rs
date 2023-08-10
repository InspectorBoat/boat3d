#![feature(portable_simd)]
use std::{simd::{SimdFloat, StdFloat, SimdPartialOrd, LaneCount, SupportedLaneCount}, hint};
use core::simd::Simd;
use std::mem;

use cgmath::{Vector4, Matrix4, Matrix};

pub trait SimdMul {
    fn simd_mul(&self, vec: &Vector4<f32>) -> Simd<f32, 4>;
}
impl SimdMul for Matrix4<f32> {
    fn simd_mul(&self, vec: &Vector4<f32>) -> Simd<f32, 4> { unsafe {
        let row_0 = Simd::from_array(*self.row(0).as_ref());
        let row_1 = Simd::from_array(*self.row(1).as_ref());
        let row_2 = Simd::from_array(*self.row(2).as_ref());
        let row_3 = Simd::from_array(*self.row(3).as_ref());
        let mut sum = Simd::splat(0.0);
        sum = row_0.fast_fma(Simd::splat(vec.x), sum);
        sum = row_1.fast_fma(Simd::splat(vec.y), sum);
        sum = row_2.fast_fma(Simd::splat(vec.z), sum);
        sum = row_3.fast_fma(Simd::splat(vec.w), sum);
        return sum;
    } }
}

pub fn frustum_cull(center_point: Vector4<f32>, outer_matrix: Matrix4<f32>, inner_matrix: Matrix4<f32>) -> u8 {
    let outer_projection = outer_matrix.simd_mul(&center_point);
    let outer_fail = outer_projection.abs().simd_gt(Simd::splat(outer_projection[3].abs())).any();
    let inner_projection = inner_matrix.simd_mul(&center_point);
    let inner_fail = inner_projection.abs().simd_gt(Simd::splat(inner_projection[3].abs())).any();
    return 2 - outer_fail as u8 - inner_fail as u8;
}

pub trait FastFma {
    fn fast_fma(self, a: Self, b: Self) -> Self;
}

impl <const LANES: usize> FastFma for Simd<f32, LANES> where LaneCount<LANES>: SupportedLaneCount {
    #[inline(always)]
    fn fast_fma(self, a: Self, b: Self) -> Self {
        if cfg!(target_feature = "fma") {
            return self.mul_add(a, b);
        }
        else {
            return self * a + b;
        }
    }
}