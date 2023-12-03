use cgmath::Vector3;

use crate::{
    hittable::{Hit, Hittable},
    ray::Ray,
};

pub struct Sphere {
    pub center: Vector3<f32>,
    pub radius: f32,
}

impl Sphere {
    pub fn new(center: Vector3<f32>, radius: f32) -> Self {
        Self { center, radius }
    }
}

impl Hittable for Sphere {
    fn test_ray(&self, ray: &Ray, min_t: f32, max_t: f32) -> Option<Hit> {
        let center_to_origin = ray.origin - self.center;

        let a = cgmath::dot(ray.direction, ray.direction);
        let b = 2. * cgmath::dot(ray.direction, center_to_origin);
        let c = cgmath::dot(center_to_origin, center_to_origin) - self.radius * self.radius;

        let d = b * b - 4. * a * c;
        if d < 0. {
            return None;
        }
        let d_sqrt = d.sqrt();

        let left = (-b - d_sqrt) / (2. * a);
        if left >= min_t && left <= max_t {
            let hit = Hit { param: left };
            return Some(hit);
        }

        let right = (-b + d_sqrt) / (2. * a);
        if right >= min_t && right <= max_t {
            let hit = Hit { param: right };
            return Some(hit);
        }

        return None;
    }
}
