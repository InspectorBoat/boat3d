use std::mem::{transmute, self};
use std::ops::Shr;
use std::thread::current;

use cgmath_culling::FrustumCuller;
use core_simd::simd::*;
use std_float::StdFloat;
use crate::gl_util::gl_wrapper::CullFace;
use crate::cull::math;

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

    pub fn from_frustum_culler(culler: FrustumCuller<f32>) -> Self { unsafe {
        pub struct PubFrustumCuller {
            pub nx_x: f32,
            pub nx_y: f32,
            pub nx_z: f32,
            pub nx_w: f32,
            pub px_x: f32,
            pub px_y: f32,
            pub px_z: f32,
            pub px_w: f32,
            pub ny_x: f32,
            pub ny_y: f32,
            pub ny_z: f32,
            pub ny_w: f32,
            pub py_x: f32,
            pub py_y: f32,
            pub py_z: f32,
            pub py_w: f32,
            pub nz_x: f32,
            pub nz_y: f32,
            pub nz_z: f32,
            pub nz_w: f32,
            pub pz_x: f32,
            pub pz_y: f32,
            pub pz_z: f32,
            pub pz_w: f32,        
        }
        let culler = mem::transmute::<_, PubFrustumCuller>(culler);
        return LocalFrustum {
            plane_xs: [culler.nx_x, culler.px_x, culler.ny_x, culler.py_x, culler.nz_x, culler.pz_x].into(),
            plane_ys: [culler.nx_y, culler.px_y, culler.ny_y, culler.py_y, culler.nz_y, culler.pz_y].into(),
            plane_zs: [culler.nx_z, culler.px_z, culler.ny_z, culler.py_z, culler.nz_z, culler.pz_z].into(),
            plane_ws: [culler.nx_w, culler.px_w, culler.ny_w, culler.py_w, culler.nz_w, culler.pz_w].into()
        }
    } }

    // #[inline(always)]
    pub fn test_local_bounding_box(&self, bb: &LocalBoundingBox) -> BoundsCheckResult { unsafe {
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
        let result = BoundsCheckResult::from_int_unchecked(none_outside as u8 + all_inside as u8);
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
    pub fn from_int_unchecked(val: u8) -> Self { unsafe {
        return mem::transmute(val);
    } }

    pub fn combine(self, rhs: Self) -> Self { unsafe {
        return BoundsCheckResult::from_int_unchecked((self as u8).min(rhs as u8));
    } }
}

/// Relative to the camera position
pub struct LocalBoundingBox {
    pub min: f32x3,
    pub max: f32x3,
}