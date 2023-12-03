use image::{ImageBuffer, RgbaImage};

use crate::{ray::Ray, scene::Scene};
use cgmath::Vector3;

pub struct Camera {}

impl Camera {
    pub fn new() -> Self {
        Self {}
    }

    pub fn render(&self, scene: &Scene) -> RgbaImage {
        let width = 1000;
        let height = 500;
        let upper_left_corner: Vector3<f32> = Vector3::new(-2., 1., -1.);
        let origin: Vector3<f32> = Vector3::new(0., 0., 0.);
        let horizontal_span: Vector3<f32> = Vector3::new(4., 0., 0.);
        let vertical_span: Vector3<f32> = Vector3::new(0., -2., 0.);

        let mut img: RgbaImage = ImageBuffer::new(width, height);
        for (x, y, pixel) in img.enumerate_pixels_mut() {
            let u = x as f32 / width as f32;
            let v = y as f32 / height as f32;
            let direction = upper_left_corner + u * horizontal_span + v * vertical_span;
            let ray = Ray::new(origin, direction);

            *pixel = scene.test_ray(&ray);
        }
        return img;
    }
}
