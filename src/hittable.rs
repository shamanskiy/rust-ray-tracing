use image::Rgba;

use crate::ray::Ray;

pub trait Hittable {
    fn test_ray(&self, ray: &Ray, min_t: f32, max_t: f32) -> Option<Rgba<u8>>;
}
