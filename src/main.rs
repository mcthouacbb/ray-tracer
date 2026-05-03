mod math;
mod tracer;
mod transform;

use std::{f32, fs::File, sync::Arc, time::Instant};

use image::{ImageFormat, ImageReader, RgbImage};
use rand::{RngExt, SeedableRng, rngs::Xoshiro256PlusPlus};

use crate::{
    math::{Quat, Vec2, Vec3},
    tracer::{
        aabb::AABB,
        camera::Camera,
        material::Material,
        primitives::{sphere::Sphere, triangle::Triangle},
        render::render_image,
        scene::{Scene, SkyLight, SubObject},
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
                    let mut uvs = [Vec2::ZERO; 3];
                    for v in 0..3 {
                        let i = indices[v] as usize;
                        vertices[v] = Vec3::new(
                            mesh.positions[(3 * i) as usize],
                            mesh.positions[(3 * i + 1) as usize],
                            mesh.positions[(3 * i + 2) as usize],
                        );
                        uvs[v] = Vec2::new(
                            mesh.texcoords[(2 * i) as usize],
                            mesh.texcoords[(2 * i + 1) as usize],
                        );
                        aabb.add_point(vertices[v]);
                    }
                    tris.push(Triangle::new(vertices, uvs));
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

struct BasicSkyLight {
    sky_color_a: Vec3,
    sky_color_b: Vec3,
}

impl BasicSkyLight {
    fn new(sky_color_a: Vec3, sky_color_b: Vec3) -> Self {
        Self {
            sky_color_a,
            sky_color_b,
        }
    }
}

impl SkyLight for BasicSkyLight {
    fn get_color(&self, ray_dir: &Vec3) -> Vec3 {
        let ray_dir = ray_dir.normalized();
        let a = ray_dir.y() * 0.5 + 0.5;
        (1.0 - a) * self.sky_color_a + a * self.sky_color_b
    }
}

fn spheres_scene(width: u32, height: u32) -> (Camera, Transform, Scene) {
    let look_from = Vec3::new(13.0, 2.0, 3.0);
    let look_at = Vec3::new(0.0, 0.0, 0.0);
    let look_up = Vec3::new(0.0, 1.0, 0.0);

    let camera_transform = Transform::look_at(&look_from, &look_at, &look_up);

    let camera = Camera::new(
        width as f32 / height as f32,
        20.0f32.to_radians(),
        10.0,
        0.6f32.to_radians(),
    );

    let mut scene = Scene::new();

    let villager_id = scene.add_mesh(SubObject::new(load_obj_model("res/villager.obj")));

    let villager_texture = Arc::new(ImageTexture::new(
        ImageReader::open("res/villager.png")
            .unwrap()
            .decode()
            .unwrap()
            .to_rgb32f(),
    ));

    scene.add_blas_instance(
        villager_id,
        Transform::look_at_scale(
            &Vec3::new(-5.0, 1.0, -6.0),
            &Vec3::new(13.0, 2.0, 3.0),
            &Vec3::new(0.0, 1.0, 0.0),
            &Vec3::from_value(1.5),
        ),
        Material::new_lambertian(villager_texture.clone()),
    );

    scene.add_blas_instance(
        villager_id,
        Transform::look_at_scale(
            &Vec3::new(-7.0, 1.0, 2.0),
            &(Vec3::new(-7.0, 1.0, 2.0) - Vec3::new(13.0, 2.0, 3.0)),
            &Vec3::new(0.0, 1.0, 0.0),
            &Vec3::from_value(1.5),
        ),
        Material::new_lambertian(villager_texture.clone()),
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
                    let color = Vec3::random_range(0.4, 0.9, &mut rng);
                    let color = color.pairwise(&color);
                    let emissive = Material::new_emissive(4.0 * color);
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

    scene.set_sky_light(Box::new(BasicSkyLight::new(
        Vec3::from_value(0.35),
        Vec3::new(0.175, 0.245, 0.35),
    )));

    scene.finalize();

    (camera, camera_transform, scene)
}

fn cornell_box(width: u32, height: u32) -> (Camera, Transform, Scene) {
    fn add_quad(scene: &mut Scene, pt: Vec3, dir1: Vec3, dir2: Vec3, material: Material) {
        let tri1 = Triangle::new(
            [pt, pt + dir1, pt + dir2],
            [
                Vec2::new(0.0, 0.0),
                Vec2::new(1.0, 0.0),
                Vec2::new(0.0, 1.0),
            ],
        );
        let tri2 = Triangle::new(
            [pt + dir2, pt + dir1, pt + dir2 + dir1],
            [
                Vec2::new(0.0, 1.0),
                Vec2::new(1.0, 0.0),
                Vec2::new(1.0, 1.0),
            ],
        );
        let mesh = SubObject::new(vec![tri1, tri2]);
        let quad_id = scene.add_mesh(mesh);
        scene.add_blas_instance(quad_id, Transform::default(), material);
    }

    let look_from = Vec3::new(278.0, 278.0, -800.0);
    let look_at = Vec3::new(278.0, 278.0, 0.0);
    let look_up = Vec3::new(0.0, 1.0, 0.0);

    let camera_transform = Transform::look_at(&look_from, &look_at, &look_up);

    let camera = Camera::new(
        width as f32 / height as f32,
        40.0f32.to_radians(),
        1.0,
        0.0f32.to_radians(),
    );

    let cube = SubObject::new(load_obj_model("res/cube.obj"));
    let mut scene = Scene::new();
    let cube_id = scene.add_mesh(cube);

    let red = Material::new_lambertian(Arc::new(SolidColor::new(Vec3::new(0.65, 0.05, 0.05))));
    let white = Material::new_lambertian(Arc::new(SolidColor::new(Vec3::from_value(0.73))));
    let green = Material::new_lambertian(Arc::new(SolidColor::new(Vec3::new(0.12, 0.45, 0.15))));
    let light = Material::new_emissive(Vec3::new(25.0, 25.0, 25.0));

    add_quad(
        &mut scene,
        Vec3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        green,
    );

    add_quad(
        &mut scene,
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        red,
    );

    add_quad(
        &mut scene,
        Vec3::new(343.0, 554.0, 332.0),
        Vec3::new(-130.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -105.0),
        light,
    );

    add_quad(
        &mut scene,
        Vec3::new(0.0, 555.0, 0.0),
        Vec3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        white.clone(),
    );

    add_quad(
        &mut scene,
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        white.clone(),
    );

    add_quad(
        &mut scene,
        Vec3::new(0.0, 0.0, 555.0),
        Vec3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        white.clone(),
    );

    scene.add_blas_instance(
        cube_id,
        Transform::new(
            &Vec3::new(372.83, 165.0, 339.66),
            &Quat::from_axis_angle(&Vec3::new(0.0, 1.0, 0.0), f32::consts::PI / 12.0),
            &Vec3::new(165.0, 330.0, 165.0),
        ),
        white.clone(),
    );

    scene.add_blas_instance(
        cube_id,
        Transform::new(
            &Vec3::new(182.97, 82.5, 168.95),
            &Quat::from_axis_angle(&Vec3::new(0.0, 1.0, 0.0), -f32::consts::PI / 10.0),
            &Vec3::new(165.0, 165.0, 165.0),
        ),
        white.clone(),
    );

    scene.finalize();

    (camera, camera_transform, scene)
}

fn main() {
    const WIDTH: u32 = 1200;
    const HEIGHT: u32 = 500;
    const SPP: u32 = 1000;
    const THREADS: u32 = 8;

    let (camera, camera_transform, scene) = cornell_box(WIDTH, HEIGHT);

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
