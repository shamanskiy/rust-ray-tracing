use cgmath::{InnerSpace, Vector3};
use rand::Rng;

pub trait RandomGenerator {
    // generate a random float in the [0,1) range
    fn f32(&self) -> f32;

    // generate a random vector in a unit sphere
    fn sphere_vec3(&self) -> Vector3<f32>;
}

pub struct RandomGeneratorImpl {}

impl RandomGeneratorImpl {
    fn vec3(&self) -> Vector3<f32> {
        return Vector3::new(self.f32(), self.f32(), self.f32());
    }
}

impl RandomGenerator for RandomGeneratorImpl {
    fn f32(&self) -> f32 {
        return rand::thread_rng().gen_range(0.0..1.0);
    }

    fn sphere_vec3(&self) -> Vector3<f32> {
        let unit_diag_vec = Vector3::new(1., 1., 1.);

        let mut vec3 = Vector3::new(1., 0., 0.);
        while vec3.dot(vec3) >= 1. {
            vec3 = self.vec3() * 2. - unit_diag_vec;
        }

        return vec3;
    }
}
