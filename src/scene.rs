use crate::{color::Color, hittable::Hittable, ray::Ray};

use image::Rgba;

use cgmath::{InnerSpace, Vector3};

pub struct Scene {
    hittable: Box<dyn Hittable>,
}

impl Scene {
    pub fn new(hittable: Box<dyn Hittable>) -> Self {
        Self { hittable }
    }

    pub fn test_ray(&self, ray: &Ray) -> Rgba<u8> {
        return match self.hittable.test_ray(&ray, 0., 100.) {
            Some(hit) => normal_to_color(hit.normal),
            None => self.background(&ray),
        };
    }

    fn background(&self, ray: &Ray) -> Rgba<u8> {
        let normalized_direction = ray.direction.normalize();
        let blend_factor = 0.5 * (normalized_direction.y + 1.);
        let color = Color::blend(Color::WHITE, Color::LIGHT_BLUE, blend_factor);
        return color.to_rgba();
    }
}

fn normal_to_color(normal: Vector3<f32>) -> Rgba<u8> {
    let normal_color = normal / 2. + Vector3::new(0.5, 0.5, 0.5);
    let normal_color = Color::from_vector(normal_color);
    return normal_color.to_rgba();
}
