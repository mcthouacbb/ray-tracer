mod math;
mod tracer;

use std::{fs::File, time::Instant};

use image::{ImageFormat, RgbImage};

use crate::{
    math::{Mat4, Vec3},
    tracer::{
        camera::Camera, hittable::Hittable, material::Material, primitives::triangle::Triangle,
        render::render_image,
    },
};

fn main() {
    const WIDTH: u32 = 1200;
    const HEIGHT: u32 = 500;
    const SPP: u32 = 500;
    const THREADS: u32 = 8;

    let look_from = Vec3::new(0.0, 0.0, 0.0);
    let look_at = Vec3::new(0.0, 0.0, -1.0);
    let look_up = Vec3::new(0.0, 1.0, 0.0);

    let camera_mat = Mat4::look_at(&look_from, &look_at, &look_up);

    let camera = Camera::new(
        WIDTH as f32 / HEIGHT as f32,
        90.0f32.to_radians(),
        1.0,
        0.0f32.to_radians(),
    );

    let mut objects = Vec::<Box<dyn Hittable>>::new();
    let material = Material::new_lambertian(Vec3::new(0.5, 1.0, 1.0));
    objects.push(Box::new(Triangle::new(
        Vec3::new(0.0, 0.0, -2.0),
        Vec3::new(1.0, 0.0, -2.0),
        Vec3::new(0.0, 1.0, -2.0),
        &material,
    )));

    let mut image = RgbImage::new(WIDTH, HEIGHT);
    let t1 = Instant::now();
    render_image(&mut image, &camera, &camera_mat, &objects, SPP, 25, THREADS);
    let t2 = Instant::now();

    let time = t2 - t1;
    println!("Time spent rendering: {}s", time.as_secs_f64());

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
