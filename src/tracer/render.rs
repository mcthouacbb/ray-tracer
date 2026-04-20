use std::f32;

use image::{Rgb, RgbImage};
use indicatif::ProgressBar;
use rand::{RngExt, SeedableRng, rngs::Xoshiro256PlusPlus};

use crate::{
    math::{Mat4, Vec3},
    tracer::{
        camera::Camera,
        hittable::Hittable,
        material::Material,
        ray::{Ray, RayHit},
        sphere::Sphere,
    },
};

pub fn sky_color(ray: &Ray) -> Vec3 {
    let a = 0.5 * ray.dir().y() + 0.5;
    (1.0 - a) * Vec3::new(1.0, 1.0, 1.0) + a * Vec3::new(0.5, 0.7, 1.0)
}

pub fn linear_to_srgb(linear: f32) -> f32 {
    if linear < 0.0031308 {
        linear * 12.92
    } else {
        1.055 * linear.powf(0.41666) - 0.055
    }
}

pub fn ray_color(
    ray: &Ray,
    objects: &[Box<dyn Hittable>],
    rng: &mut impl RngExt,
    depth: u32,
) -> Vec3 {
    if depth == 0 {
        return Vec3::ZERO;
    }

    let mut ray_hit = RayHit::NONE;
    for object in objects {
        ray_hit.replace_if_closer(&object.trace(ray));
    }
    ray_hit.finalize(&ray);
    if ray_hit.dist() < f32::INFINITY {
        match ray_hit.material().scatter(&ray, &ray_hit, rng) {
            Some(scatter_result) => {
                let sub_color = ray_color(scatter_result.scattered_ray(), objects, rng, depth - 1);
                scatter_result.attenuation().pairwise(&sub_color)
            }
            None => Vec3::new(0.0, 0.0, 0.0),
        }
    } else {
        sky_color(&ray)
    }
}

pub fn render_image(image: &mut RgbImage, spp: u32, max_depth: u32) {
    let width = image.width();
    let height = image.height();
    let aspect_ratio = width as f32 / height as f32;

    let look_from = Vec3::new(13.0, 2.0, 3.0);
    let look_at = Vec3::new(0.0, 0.0, 0.0);
    let look_up = Vec3::new(0.0, 1.0, 0.0);

    let camera_mat = Mat4::look_at(&look_from, &look_at, &look_up);

    let camera = Camera::new(
        aspect_ratio,
        20.0f32.to_radians(),
        10.0,
        0.6f32.to_radians(),
    );

    let progress_bar = ProgressBar::new((width * height) as u64);

    let mut objects = Vec::<Box<dyn Hittable>>::new();
    let ground_material = Material::new_lambertian(Vec3::new(0.5, 0.5, 0.5));
    objects.push(Box::new(Sphere::new(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        &ground_material,
    )));

    let mut rng = Xoshiro256PlusPlus::from_rng(&mut rand::rng());

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

    for y in 0..height {
        for x in 0..width {
            let mut accum_color = Vec3::ZERO;

            for _ in 0..spp {
                let jitter_x = rng.random_range(-0.5..=0.5f32);
                let jitter_y = rng.random_range(-0.5..=0.5f32);
                let u = (2.0 * (x as f32 + jitter_x) as f32 - width as f32 + 1.0) / width as f32;
                let v = -(2.0 * (y as f32 + jitter_y) as f32 - height as f32 + 1.0) / height as f32;

                let camera_ray = camera.get_ray_dir(u, v, &mut rng);
                let ray = Ray::new(
                    camera_mat.transform_pos(&camera_ray.origin()),
                    camera_mat.transform_dir(&camera_ray.dir()),
                );

                let color = ray_color(&ray, &objects, &mut rng, max_depth);
                accum_color += color;
            }

            let pixel_color = accum_color / spp as f32;
            image.put_pixel(
                x,
                y,
                Rgb([
                    (linear_to_srgb(pixel_color.x()) * 255.0 + 0.5) as u8,
                    (linear_to_srgb(pixel_color.y()) * 255.0 + 0.5) as u8,
                    (linear_to_srgb(pixel_color.z()) * 255.0 + 0.5) as u8,
                ]),
            );
            progress_bar.inc(1);
        }
    }

    progress_bar.finish();
}
