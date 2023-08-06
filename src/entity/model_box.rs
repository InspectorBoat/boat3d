use cgmath::Vector3;

pub struct ModelBox {
    pub min: Vector3<f64>,
    pub max: Vector3<f64>,
    pub texture: u16
}