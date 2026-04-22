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

fn load_obj_model(file_name: &str, material: &Material, objects: &mut Vec<Box<dyn Hittable>>) {
    match tobj::load_obj(
        file_name,
        &tobj::LoadOptions {
            triangulate: true,
            single_index: true,
            ..Default::default()
        },
    ) {
        Ok((models, _)) => {
            for model in models {
                let mesh = &model.mesh;
                for indices in mesh.indices.chunks_exact(3) {
                    let mut vertices = [Vec3::ZERO; 3];
                    for v in 0..3 {
                        let i = indices[v] as usize;
                        vertices[v] = Vec3::new(
                            mesh.positions[(3 * i) as usize],
                            mesh.positions[(3 * i + 1) as usize],
                            mesh.positions[(3 * i + 2) as usize],
                        );
                    }
                    let tri = Triangle::new(vertices[0], vertices[1], vertices[2], material);
                    objects.push(Box::new(tri));
                }
            }
        }
        Err(err) => {
            eprintln!("Failed to load .obj file '{}': {}", file_name, err);
        }
    }
}

fn main() {
    const WIDTH: u32 = 1200;
    const HEIGHT: u32 = 500;
    const SPP: u32 = 500;
    const THREADS: u32 = 8;

    let look_from = Vec3::new(5.0, 0.0, 30.0);
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
    let material = Material::new_lambertian(Vec3::new(0.404, 0.902, 0.388));
    load_obj_model("res/villager.obj", &material, &mut objects);

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
