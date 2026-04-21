mod math;
mod tracer;

use std::fs::File;

use image::{ImageFormat, RgbImage};
use rand::{RngExt, SeedableRng, rngs::Xoshiro256PlusPlus};

use crate::{
    math::{Mat4, Vec3},
    tracer::{
        camera::Camera, hittable::Hittable, material::Material, render::render_image,
        sphere::Sphere,
    },
};

fn main() {
    const WIDTH: u32 = 1200;
    const HEIGHT: u32 = 500;
    const SPP: u32 = 500;
    const THREADS: u32 = 8;

    let look_from = Vec3::new(13.0, 2.0, 3.0);
    let look_at = Vec3::new(0.0, 0.0, 0.0);
    let look_up = Vec3::new(0.0, 1.0, 0.0);

    let camera_mat = Mat4::look_at(&look_from, &look_at, &look_up);

    let camera = Camera::new(
        WIDTH as f32 / HEIGHT as f32,
        20.0f32.to_radians(),
        10.0,
        0.6f32.to_radians(),
    );

    let mut objects = Vec::<Box<dyn Hittable>>::new();
    let ground_material = Material::new_lambertian(Vec3::new(0.5, 0.5, 0.5));
    objects.push(Box::new(Sphere::new(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        &ground_material,
    )));

    let mut rng = Xoshiro256PlusPlus::seed_from_u64(283748328);

    for a in -11..11 {
        for b in -11..11 {
            let center = Vec3::new(
                a as f32 + rng.random_range(0.0..0.9),
                0.2,
                b as f32 + rng.random_range(0.0..0.9),
            );

            if (center - Vec3::new(4.0, 0.2, 0.0)).len() > 0.9 {
                let choose_mat = rng.random_range(0.0..1.0);
                if choose_mat < 0.8 {
                    let albedo = Vec3::random_range(0.0, 1.0, &mut rng)
                        .pairwise(&Vec3::random_range(0.0, 1.0, &mut rng));
                    let lambertian = Material::new_lambertian(albedo);
                    objects.push(Box::new(Sphere::new(center, 0.2, &lambertian)));
                } else if choose_mat < 0.95 {
                    let albedo = Vec3::random_range(0.5, 1.0, &mut rng);
                    let fuzz = rng.random_range(0.0..=0.5);
                    let metal = Material::new_metal(albedo, fuzz);
                    objects.push(Box::new(Sphere::new(center, 0.2, &metal)));
                } else {
                    let dielectric = Material::new_dielectric(1.5);
                    objects.push(Box::new(Sphere::new(center, 0.2, &dielectric)));
                }
            }
        }
    }

    objects.push(Box::new(Sphere::new(
        Vec3::new(0.0, 1.0, 0.0),
        1.0,
        &Material::new_dielectric(1.5),
    )));
    objects.push(Box::new(Sphere::new(
        Vec3::new(-4.0, 1.0, 0.0),
        1.0,
        &Material::new_lambertian(Vec3::new(0.4, 0.2, 0.1)),
    )));
    objects.push(Box::new(Sphere::new(
        Vec3::new(4.0, 1.0, 0.0),
        1.0,
        &Material::new_metal(Vec3::new(0.7, 0.6, 0.5), 0.0),
    )));

    let mut image = RgbImage::new(WIDTH, HEIGHT);
    render_image(&mut image, &camera, &camera_mat, &objects, SPP, 25, THREADS);

    let mut file = match File::create("render.png") {
        Ok(file) => file,
        Err(err) => {
            eprintln!("Cannot open file 'render.png': {}", err);
            return;
        }
    };

    if let Err(err) = image.write_to(&mut file, ImageFormat::Png) {
        eprintln!("Cannot write image to file 'render.png': {}", err);
    }
}
