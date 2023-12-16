use crate::{
    color::Color,
    geometries::hittable::{Hit, Hittable},
    ray::Ray,
};

use cgmath::InnerSpace;

const MAX_REFLECTIONS: u32 = 2;

pub struct Scene {
    hittables: Vec<Box<dyn Hittable>>,
}

impl Scene {
    pub fn new(hittables: Vec<Box<dyn Hittable>>) -> Self {
        Self { hittables }
    }

    pub fn test_ray(&self, ray: &Ray) -> Color {
        return self.test_ray_recursive(ray, 0);
    }

    fn test_ray_recursive(&self, ray: &Ray, reflection_level: u32) -> Color {
        let closest_hit = self.find_closest_hit(ray);

        if closest_hit.is_none() {
            return self.background(ray);
        }

        if reflection_level >= MAX_REFLECTIONS {
            return Color::BLACK;
        }

        let hit = closest_hit.unwrap();
        let reflection = hit.material.unwrap().reflect(hit.point, hit.normal);
        return reflection.color * self.test_ray_recursive(&reflection.ray, reflection_level + 1);
    }

    fn find_closest_hit(&self, ray: &Ray) -> Option<Hit> {
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

        return closest_hit;
    }

    fn background(&self, ray: &Ray) -> Color {
        let normalized_direction = ray.direction.normalize();
        let blend_factor = 0.5 * (normalized_direction.y + 1.);
        let color = Color::blend(Color::WHITE, Color::LIGHT_BLUE, blend_factor);
        return color;
    }
}

// fn normal_to_color(normal: Vector3<f32>) -> Color {
//     let normal_color = normal / 2. + Vector3::new(0.5, 0.5, 0.5);
//     let normal_color = Color::from_vector(normal_color);
//     return normal_color;
// }
