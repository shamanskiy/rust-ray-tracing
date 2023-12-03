use crate::{
    color::Color,
    hittable::{Hit, Hittable},
    ray::Ray,
};

use image::Rgba;

use cgmath::{InnerSpace, Vector3};

pub struct Scene {
    hittables: Vec<Box<dyn Hittable>>,
}

impl Scene {
    pub fn new(hittables: Vec<Box<dyn Hittable>>) -> Self {
        Self { hittables }
    }

    pub fn test_ray(&self, ray: &Ray) -> Rgba<u8> {
        let mut closest_hit: Option<Hit> = None;
        let mut closest_hit_param = f32::INFINITY;

        for hittable in self.hittables.iter() {
            match hittable.test_ray(ray, 0., 100.) {
                Some(hit) => {
                    if hit.param < closest_hit_param {
                        closest_hit_param = hit.param;
                        closest_hit = Some(hit);
                    }
                }
                None => {}
            }
        }

        return match closest_hit {
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
