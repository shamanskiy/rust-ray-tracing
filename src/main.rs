use std::io;

use cgmath::Vector3;
use image::RgbaImage;
use rust_ray_tracing::{camera::Camera, hittable::Hittable, sphere::Sphere};

fn main() -> io::Result<()> {
    let scene = make_scene();
    let camera: Camera = Camera::new();

    let img: RgbaImage = camera.render(&scene);

    save_image(&img, "test.png")?;
    Ok(())
}

fn make_scene() -> impl Hittable {
    let radius: Vector3<f32> = Vector3::new(0., 0., -1.);
    return Sphere::new(radius, 0.5);
}

fn save_image(img: &RgbaImage, filename: &str) -> io::Result<()> {
    img.save(filename)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
    Ok(())
}
