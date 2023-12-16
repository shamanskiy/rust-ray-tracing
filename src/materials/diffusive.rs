use crate::{color::Color, rand::random_generator::RandomGenerator, ray::Ray};

use super::material::{Material, Reflection, ReflectionType};

pub struct Diffusive {
    color: Color,
    randomizer: Box<dyn RandomGenerator>,
}

impl Diffusive {
    pub fn new(color: Color, randomizer: Box<dyn RandomGenerator>) -> Self {
        Self {
            color: color,
            randomizer: randomizer,
        }
    }
}

impl Material for Diffusive {
    fn reflect(
        &self,
        hit_point: cgmath::Vector3<f32>,
        hit_normal: cgmath::Vector3<f32>,
    ) -> Reflection {
        let reflected_direction = hit_normal + self.randomizer.sphere_vec3();
        return Reflection {
            reflection_type: ReflectionType::Scattered,
            ray: Ray::new(hit_point, reflected_direction),
            color: self.color,
        };
    }
}
