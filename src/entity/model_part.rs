use cgmath::Vector3;

use super::model_box::ModelBox;

pub struct ModelPart {
    pub pos: Vector3<f64>,
    pub hinge: Vector3<f64>,
    pub pivot: Vector3<f64>,
    pub boxes: Vec<ModelBox>,
    pub children: Vec<ModelPart>
}

/* 
 * SSBO contains:
 * model part pos - f32x4
 * model part rot - f32x4
 * box min - f32x4
 * box max - f32x4
 */