use image::{Rgb, RgbImage};
use indicatif::ProgressBar;
use rand::{RngExt, SeedableRng, rngs::Xoshiro256PlusPlus};

use crate::{
    math::Vec3,
    tracer::{
        hittable::Hittable,
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
    if ray_hit.dist() < f32::INFINITY {
        let new_dir = ray_hit.normal() + Vec3::random_unit(rng);
        let new_origin = ray.origin() + ray.dir() * ray_hit.dist() + 1e-4 * ray_hit.normal();
        let new_ray = Ray::new(new_origin, new_dir.normalized());
        return 0.5 * ray_color(&new_ray, objects, rng, depth - 1);
    } else {
        sky_color(&ray)
    }
}

pub fn render_image(image: &mut RgbImage, spp: u32, max_depth: u32) {
    let width = image.width();
    let height = image.height();
    let aspect_ratio = width as f32 / height as f32;

    let progress_bar = ProgressBar::new((width * height) as u64);

    let mut objects = Vec::<Box<dyn Hittable>>::new();
    objects.push(Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5)));
    objects.push(Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0)));

    const CAMERA_POS: Vec3 = Vec3::ZERO;

    let mut rng = Xoshiro256PlusPlus::from_rng(&mut rand::rng());

    for y in 0..height {
        for x in 0..width {
            let mut accum_color = Vec3::ZERO;

            for _ in 0..spp {
                let jitter_x = rng.random_range(-0.5..=0.5f32);
                let jitter_y = rng.random_range(-0.5..=0.5f32);
                let u = (2.0 * (x as f32 + jitter_x) as f32 - width as f32 + 1.0) / width as f32;
                let v = -(2.0 * (y as f32 + jitter_y) as f32 - height as f32 + 1.0) / height as f32;

                let ray_dir = Vec3::new(u * aspect_ratio, v, -1.0).normalized();
                let ray = Ray::new(CAMERA_POS, ray_dir);

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
