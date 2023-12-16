use std::{io, time::Instant};

use cgmath::Vector3;
use image::RgbaImage;
use rust_ray_tracing::{
    camera::Camera, color::Color, geometries::sphere::Sphere, material_hittable::MaterialHittable,
    materials::diffusive::Diffusive, rand::random_generator::RandomGeneratorImpl, scene::Scene,
};

fn main() -> io::Result<()> {
    let scene = make_scene();
    let randomizer = RandomGeneratorImpl {};
    let camera = Camera::new(Box::new(randomizer));

    let start = Instant::now();
    let img = camera.render(&scene);
    let duration = start.elapsed();
    println!("rendering time: {:?}", duration);

    save_image(&img, "test.png")?;
    Ok(())
}

fn make_scene() -> Scene {
    let sphere_center = Vector3::new(0., 0., -1.);
    let sphere = Sphere::new(sphere_center, 0.5);
    let sphere_material = Diffusive::new(Color::RED, Box::new(RandomGeneratorImpl {}));
    let sphere_object = MaterialHittable::new(Box::new(sphere), Box::new(sphere_material));

    let floor_sphere_center = Vector3::new(0., -100.5, -1.);
    let floor_sphere = Sphere::new(floor_sphere_center, 100.);
    let floor_material = Diffusive::new(Color::MEDIUM_GRAY, Box::new(RandomGeneratorImpl {}));
    let floor_object = MaterialHittable::new(Box::new(floor_sphere), Box::new(floor_material));

    return Scene::new(vec![Box::new(sphere_object), Box::new(floor_object)]);
}

fn save_image(img: &RgbaImage, filename: &str) -> io::Result<()> {
    img.save(filename)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
    Ok(())
}
