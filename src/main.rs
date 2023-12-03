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
    let sphere_center = Vector3::new(0., 0., -1.);
    let sphere = Sphere::new(sphere_center, 0.5);

    let floor_sphere_center = Vector3::new(0., -100.5, -1.);
    let floor_sphere = Sphere::new(floor_sphere_center, 100.);
    return Scene::new(vec![Box::new(sphere), Box::new(floor_sphere)]);
}

fn save_image(img: &RgbaImage, filename: &str) -> io::Result<()> {
    img.save(filename)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
    Ok(())
}
