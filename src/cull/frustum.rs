use std::mem::{transmute, self};
use std::ops::Shr;
use std::thread::current;

use cgmath::{Matrix4, SquareMatrix, Vector3};
use core_simd::simd::*;
use std_float::StdFloat;
use crate::gl_util::gl_wrapper::CullFace;
use crate::cull::math;

use super::bounding_box::SimdBoundingBox;
use super::math::*;

/// When using this, it is expected that coordinates are relative to the camera rather than the
/// world origin.
pub struct LocalFrustum {
    plane_xs: f32x6,
    plane_ys: f32x6,
    plane_zs: f32x6,
    plane_ws: f32x6,
}

impl LocalFrustum {
    pub fn new(planes: [f32x6; 4]) -> Self {
        LocalFrustum {
            plane_xs: planes[0],
            plane_ys: planes[1],
            plane_zs: planes[2],
            plane_ws: planes[3],
        }
    }

    pub fn from_matrix(matrix: Matrix4<f32>) -> Self { unsafe {
        return LocalFrustum {
            plane_xs: [matrix.x.w + matrix.x.x, matrix.x.w - matrix.x.x, matrix.x.w + matrix.x.y, matrix.x.w - matrix.x.y, matrix.x.w + matrix.x.z, matrix.x.w - matrix.x.z].into(),
            plane_ys: [matrix.y.w + matrix.y.x, matrix.y.w - matrix.y.x, matrix.y.w + matrix.y.y, matrix.y.w - matrix.y.y, matrix.y.w + matrix.y.z, matrix.y.w - matrix.y.z].into(),
            plane_zs: [matrix.z.w + matrix.z.x, matrix.z.w - matrix.z.x, matrix.z.w + matrix.z.y, matrix.z.w - matrix.z.y, matrix.z.w + matrix.z.z, matrix.z.w - matrix.z.z].into(),
            plane_ws: [matrix.w.w + matrix.w.x, matrix.w.w - matrix.w.x, matrix.w.w + matrix.w.y, matrix.w.w - matrix.w.y, matrix.w.w + matrix.w.z, matrix.w.w - matrix.w.z].into()
        }
    } }

    // #[inline(always)]
    pub fn test_local_bounding_box(&self, bb: &SimdBoundingBox) -> BoundsCheckResult { unsafe {
        // These unsafe mask shenanigans just check if the sign bit is set for each lane.
        // This is faster than doing a manual comparison with something like simd_gt.
        let is_neg_x = Mask::from_int_unchecked(self.plane_xs.to_bits().cast::<i32>() >> Simd::splat(31));
        let is_neg_y = Mask::from_int_unchecked(self.plane_ys.to_bits().cast::<i32>() >> Simd::splat(31));
        let is_neg_z = Mask::from_int_unchecked(self.plane_zs.to_bits().cast::<i32>() >> Simd::splat(31));

        let bb_min_x = Simd::splat(bb.min.x());
        let bb_max_x = Simd::splat(bb.max.x());
        let outside_bounds_x = is_neg_x.select(bb_min_x, bb_max_x);
        let inside_bounds_x = is_neg_x.select(bb_max_x, bb_min_x);

        let bb_min_y = Simd::splat(bb.min.y());
        let bb_max_y = Simd::splat(bb.max.y());
        let outside_bounds_y = is_neg_y.select(bb_min_y, bb_max_y);
        let inside_bounds_y = is_neg_y.select(bb_max_y, bb_min_y);

        let bb_min_z = Simd::splat(bb.min.z());
        let bb_max_z = Simd::splat(bb.max.z());
        let outside_bounds_z = is_neg_z.select(bb_min_z, bb_max_z);
        let inside_bounds_z = is_neg_z.select(bb_max_z, bb_min_z);

        let outside_length_sq = self.plane_xs.fast_fma(
            outside_bounds_x,
            self.plane_ys.fast_fma(outside_bounds_y, self.plane_zs * outside_bounds_z),
        );

        let inside_length_sq = self.plane_xs.fast_fma(
            inside_bounds_x,
            self.plane_ys.fast_fma(inside_bounds_y, self.plane_zs * inside_bounds_z),
        );

        // if any outside lengths are greater than -w, return OUTSIDE
        // if all inside lengths are greater than -w, return INSIDE
        // otherwise, return PARTIAL
        let none_outside = outside_length_sq.simd_ge(-self.plane_ws).to_bitmask() == 0b111111;
        let all_inside = inside_length_sq.simd_ge(-self.plane_ws).to_bitmask() == 0b111111;
        let result = BoundsCheckResult::from_int(none_outside as u8 + all_inside as u8);
        return result;
    } }
}

#[repr(u8)]
#[derive(Debug)]
pub enum BoundsCheckResult {
    Outside = 0,
    Partial = 1,
    Inside = 2,
}

impl BoundsCheckResult {
    pub fn from_int(val: u8) -> Self { unsafe {
        return mem::transmute(val);
    } }

    pub fn combine(self, rhs: Self) -> Self { unsafe {
        return BoundsCheckResult::from_int((self as u8).min(rhs as u8));
    } }
}

