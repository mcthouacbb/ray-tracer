mod math;
mod tracer;
mod transform;

use std::{fs::File, time::Instant};

use image::{ImageFormat, RgbImage};
use rand::{RngExt, SeedableRng, rngs::Xoshiro256PlusPlus};

use crate::{
    math::Vec3,
    tracer::{
        aabb::AABB,
        camera::Camera,
        material::Material,
        primitives::{Primitive, sphere::Sphere, triangle::Triangle},
        render::render_image,
        scene::{Scene, SubObject},
    },
    transform::Transform,
};

fn load_obj_model(file_name: &str, objects: &mut Vec<Box<dyn Primitive>>) {
    let mut tris = Vec::new();
    let mut aabb = AABB::NEG_INF;
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
                        aabb.add_point(vertices[v]);
                    }
                    tris.push(Triangle::new(vertices[0], vertices[1], vertices[2]));
                }
            }
        }
        Err(err) => {
            eprintln!("Failed to load .obj file '{}': {}", file_name, err);
        }
    }

    let scale = 3.0 / (aabb.extent().x() + aabb.extent().y() + aabb.extent().z());

    for tri in tris {
        let mut new_vertices = [Vec3::ZERO; 3];
        for i in 0..3 {
            new_vertices[i] = (tri.vertices()[i] - aabb.center()) * scale
        }
        objects.push(Box::new(Triangle::new(
            new_vertices[0],
            new_vertices[1],
            new_vertices[2],
        )));
    }
}

fn main() {
    const WIDTH: u32 = 1200;
    const HEIGHT: u32 = 500;
    const SPP: u32 = 500;
    const THREADS: u32 = 8;

    let look_from = Vec3::new(13.0, 2.0, 3.0);
    let look_at = Vec3::new(0.0, 0.0, 0.0);
    let look_up = Vec3::new(0.0, 1.0, 0.0);

    let camera_transform = Transform::look_at(&look_from, &look_at, &look_up);

    let camera = Camera::new(
        WIDTH as f32 / HEIGHT as f32,
        20.0f32.to_radians(),
        16.0,
        0.6f32.to_radians(),
    );

    let mut scene = Scene::new();

    let mut objects = Vec::<Box<dyn Primitive>>::new();
    load_obj_model("res/villager.obj", &mut objects);
    let villager_id = scene.add_mesh(SubObject::new(objects));

    scene.add_blas_instance(
        villager_id,
        Transform::look_at_scale(
            &Vec3::new(-5.0, 1.0, -6.0),
            &Vec3::new(13.0, 2.0, 3.0),
            &Vec3::new(0.0, 1.0, 0.0),
            &Vec3::from_value(1.5),
        ),
        Material::new_emissive(Vec3::new(0.404, 0.902, 0.388)),
    );
    scene.add_blas_instance(
        villager_id,
        Transform::look_at_scale(
            &Vec3::new(-7.0, 1.0, 2.0),
            &(Vec3::new(-7.0, 1.0, 2.0) - Vec3::new(13.0, 2.0, 3.0)),
            &Vec3::new(0.0, 1.0, 0.0),
            &Vec3::from_value(1.5),
        ),
        Material::new_metal(Vec3::new(0.404, 0.902, 0.388), 0.3),
    );

    let ground_material = Material::new_lambertian(Vec3::new(0.5, 0.5, 0.5));
    scene.add_global_primitive(
        Box::new(Sphere::new(Vec3::new(0.0, -1000.0, 0.0), 1000.0)),
        ground_material,
    );

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
                if choose_mat < 0.5 {
                    let albedo = Vec3::random_range(0.0, 1.0, &mut rng)
                        .pairwise(&Vec3::random_range(0.0, 1.0, &mut rng));
                    let lambertian = Material::new_lambertian(albedo);
                    scene.add_global_primitive(Box::new(Sphere::new(center, 0.2)), lambertian);
                } else if choose_mat < 0.8 {
                    let color = Vec3::random_range(0.5, 1.0, &mut rng);
                    let emissive = Material::new_emissive(color);
                    scene.add_global_primitive(Box::new(Sphere::new(center, 0.2)), emissive);
                } else if choose_mat < 0.95 {
                    let albedo = Vec3::random_range(0.5, 1.0, &mut rng);
                    let fuzz = rng.random_range(0.0..=0.5);
                    let metal = Material::new_metal(albedo, fuzz);
                    scene.add_global_primitive(Box::new(Sphere::new(center, 0.2)), metal);
                } else {
                    let dielectric = Material::new_dielectric(1.5);
                    scene.add_global_primitive(Box::new(Sphere::new(center, 0.2)), dielectric);
                }
            }
        }
    }

    scene.add_global_primitive(
        Box::new(Sphere::new(Vec3::new(0.0, 1.0, 0.0), 1.0)),
        Material::new_dielectric(1.5),
    );
    scene.add_global_primitive(
        Box::new(Sphere::new(Vec3::new(-4.0, 1.0, 0.0), 1.0)),
        Material::new_lambertian(Vec3::new(0.4, 0.2, 0.1)),
    );
    scene.add_global_primitive(
        Box::new(Sphere::new(Vec3::new(4.0, 1.0, 0.0), 1.0)),
        Material::new_metal(Vec3::new(0.7, 0.6, 0.5), 0.0),
    );

    scene.finalize();

    let mut image = RgbImage::new(WIDTH, HEIGHT);
    let t1 = Instant::now();
    render_image(
        &mut image,
        &camera,
        &camera_transform,
        &scene,
        SPP,
        25,
        THREADS,
    );
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
