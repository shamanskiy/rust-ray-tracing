use cgmath::Vector3;

use crate::{color::Color, ray::Ray};

pub trait Material {
    fn reflect(&self, hit_point: Vector3<f32>, hit_normal: Vector3<f32>) -> Reflection;
}

pub struct Reflection {
    pub reflection_type: ReflectionType,
    pub ray: Ray,
    pub color: Color,
}

pub enum ReflectionType {
    Scattered,
    Absorbed,
}
