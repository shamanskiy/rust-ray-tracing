use crate::{hittable::Hittable, ray::Ray};

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
            Some(hit) => Rgba([255, 0, 0, 255]),
            None => self.background(&ray),
        };
    }

    fn background(&self, ray: &Ray) -> Rgba<u8> {
        let normalized_direction = ray.direction.normalize();
        let blend_factor = 0.5 * (normalized_direction.y + 1.);
        let white = Vector3::new(1., 1., 1.);
        let light_blue = Vector3::new(0.5, 0.7, 1.0);

        let color = (1. - blend_factor) * white + blend_factor * light_blue;

        let red = (color.x * 255.) as u8;
        let green = (color.y * 255.) as u8;
        let blue = (color.z * 255.) as u8;
        return Rgba([red, green, blue, 255]);
    }
}
