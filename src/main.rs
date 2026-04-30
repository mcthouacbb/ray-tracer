mod math;
mod tracer;
mod transform;

use std::{fs::File, sync::Arc, time::Instant};

use image::{ImageFormat, ImageReader, RgbImage};
use rand::{RngExt, SeedableRng, rngs::Xoshiro256PlusPlus};

use crate::{
    math::Vec3,
    tracer::{
        aabb::AABB,
        camera::Camera,
        material::Material,
        primitives::{
            sphere::{self, Sphere},
            triangle::Triangle,
        },
        render::render_image,
        scene::{Scene, SubObject},
        texture::{ImageTexture, SolidColor, SpatialChecker},
    },
    transform::Transform,
};

fn load_obj_model(file_name: &str) -> Vec<Triangle> {
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

    for tri in &mut tris {
        let mut new_vertices = [Vec3::ZERO; 3];
        for i in 0..3 {
            new_vertices[i] = (tri.vertices()[i] - aabb.center()) * scale
        }
        tri.vertices_mut().copy_from_slice(&new_vertices);
    }
    tris
}

fn write_image(image: &RgbImage, filename: &str) {
    let mut file = match File::create(filename) {
        Ok(file) => file,
        Err(err) => {
            eprintln!("Cannot open file '{}': {}", filename, err);
            return;
        }
    };

    if let Err(err) = image.write_to(&mut file, ImageFormat::Png) {
        eprintln!("Cannot write image to file '{}': {}", filename, err);
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
        10.0,
        0.6f32.to_radians(),
    );

    let mut scene = Scene::new();

    let objects = load_obj_model("res/villager.obj");
    let villager_id = scene.add_mesh(SubObject::new(objects));

    scene.add_blas_instance(
        villager_id,
        Transform::look_at_scale(
            &Vec3::new(-5.0, 1.0, -6.0),
            &Vec3::new(13.0, 2.0, 3.0),
            &Vec3::new(0.0, 1.0, 0.0),
            &Vec3::from_value(1.5),
        ),
        Material::new_emissive(Vec3::new(0.902, 0.554, 0.388)),
    );
    scene.add_blas_instance(
        villager_id,
        Transform::look_at_scale(
            &Vec3::new(-7.0, 1.0, 2.0),
            &(Vec3::new(-7.0, 1.0, 2.0) - Vec3::new(13.0, 2.0, 3.0)),
            &Vec3::new(0.0, 1.0, 0.0),
            &Vec3::from_value(1.5),
        ),
        Material::new_metal(
            Arc::new(SolidColor::new(Vec3::new(0.404, 0.902, 0.388))),
            0.3,
        ),
    );

    let even_color = Arc::new(SolidColor::new(Vec3::new(0.2, 0.3, 0.1)));
    let odd_color = Arc::new(SolidColor::new(Vec3::new(0.9, 0.9, 0.9)));
    let checker = Arc::new(SpatialChecker::new(0.32, even_color, odd_color));
    let ground_material = Material::new_lambertian(checker);
    scene.add_sphere(
        Sphere::new(Vec3::new(0.0, -1000.0, 0.0), 1000.0),
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
                    let lambertian = Material::new_lambertian(Arc::new(SolidColor::new(albedo)));
                    scene.add_sphere(Sphere::new(center, 0.2), lambertian);
                } else if choose_mat < 0.8 {
                    let color = Vec3::random_range(0.5, 1.0, &mut rng);
                    let emissive = Material::new_emissive(color);
                    scene.add_sphere(Sphere::new(center, 0.2), emissive);
                } else if choose_mat < 0.95 {
                    let albedo = Vec3::random_range(0.5, 1.0, &mut rng);
                    let fuzz = rng.random_range(0.0..=0.5);
                    let metal = Material::new_metal(Arc::new(SolidColor::new(albedo)), fuzz);
                    scene.add_sphere(Sphere::new(center, 0.2), metal);
                } else {
                    let dielectric = Material::new_dielectric(1.5);
                    scene.add_sphere(Sphere::new(center, 0.2), dielectric);
                }
            }
        }
    }

    // unwrap() lol!
    let earth_image = ImageReader::open("res/earthmap.jpg")
        .unwrap()
        .decode()
        .unwrap()
        .into_rgb32f();
    let earth_texture = Arc::new(ImageTexture::new(earth_image));
    let earth_material = Material::new_lambertian(earth_texture);

    scene.add_sphere(Sphere::new(Vec3::new(-1.0, 1.5, 3.8), 0.8), earth_material);

    scene.add_sphere(
        Sphere::new(Vec3::new(0.0, 1.0, 0.0), 1.0),
        Material::new_dielectric(1.5),
    );

    scene.add_sphere(
        Sphere::new(Vec3::new(-4.0, 1.0, 0.0), 1.0),
        Material::new_lambertian(Arc::new(SolidColor::new(Vec3::new(0.4, 0.2, 0.1)))),
    );
    scene.add_sphere(
        Sphere::new(Vec3::new(4.0, 1.0, 0.0), 1.0),
        Material::new_metal(Arc::new(SolidColor::new(Vec3::new(0.7, 0.6, 0.5))), 0.0),
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

    write_image(&image, "render.png");
}
