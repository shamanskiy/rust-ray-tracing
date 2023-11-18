use image::{ImageBuffer, Rgba, RgbaImage};

use crate::{hittable::Hittable, ray::Ray};
use cgmath::{InnerSpace, Vector3};

pub struct Camera {}

impl Camera {
    pub fn new() -> Self {
        Self {}
    }

    pub fn render(&self, hittable: &dyn Hittable) -> RgbaImage {
        let width = 200;
        let height = 100;
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

            match hittable.test_ray(&ray, 0., 100.) {
                Some(color) => *pixel = color,
                None => *pixel = self.color_ray(&ray),
            }
        }
        return img;
    }

    fn color_ray(&self, ray: &Ray) -> Rgba<u8> {
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
