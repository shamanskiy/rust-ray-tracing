use crate::ray::Ray;

pub trait Hittable {
    fn test_ray(&self, ray: &Ray, min_t: f32, max_t: f32) -> Option<Hit>;
}

pub struct Hit {
    pub param: f32,
    // pub point: Vector3<f32>,
    // pub normal: Vector3<f32>,
}
