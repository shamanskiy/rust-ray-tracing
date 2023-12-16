use image::{ImageBuffer, RgbaImage};

use crate::{color::Color, rand::random_generator::RandomGenerator, ray::Ray, scene::Scene};
use cgmath::Vector3;

pub struct Camera {
    randomizer: Box<dyn RandomGenerator>,
}

const NUM_SAMPLES: u32 = 10;

impl Camera {
    pub fn new(randomizer: Box<dyn RandomGenerator>) -> Self {
        Self {
            randomizer: randomizer,
        }
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
            let mut total_color = Color::BLACK;

            for _ in 0..NUM_SAMPLES {
                let u = self.to_param(x, width);
                let v = self.to_param(y, height);
                let direction = upper_left_corner + u * horizontal_span + v * vertical_span;
                let ray = Ray::new(origin, direction);

                total_color = total_color + scene.test_ray(&ray);
            }

            *pixel = (total_color / NUM_SAMPLES as f32).to_rgba();
        }
        return img;
    }

    fn to_param(&self, index: u32, max_index: u32) -> f32 {
        let random_offset: f32 = self.randomizer.f32();
        return (index as f32 + random_offset) / max_index as f32;
    }
}
