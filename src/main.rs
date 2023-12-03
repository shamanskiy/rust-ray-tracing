use std::io;

use cgmath::Vector3;
use image::RgbaImage;
use rust_ray_tracing::{camera::Camera, scene::Scene, sphere::Sphere};

fn main() -> io::Result<()> {
    let scene = make_scene();
    let camera = Camera::new();

    let img = camera.render(&scene);

    save_image(&img, "test.png")?;
    Ok(())
}

fn make_scene() -> Scene {
    let radius: Vector3<f32> = Vector3::new(0., 0., -1.);
    let sphere = Sphere::new(radius, 0.5);
    return Scene::new(Box::new(sphere));
}

fn save_image(img: &RgbaImage, filename: &str) -> io::Result<()> {
    img.save(filename)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
    Ok(())
}
