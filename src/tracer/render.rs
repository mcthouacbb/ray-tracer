use image::{Rgb, RgbImage};
use indicatif::ProgressBar;

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

pub fn ray_color(ray: &Ray, objects: &[Box<dyn Hittable>]) -> Vec3 {
    let mut ray_hit = RayHit::NONE;
    for object in objects {
        ray_hit.replace_if_closer(&object.trace(ray));
    }
    if ray_hit.dist() < f32::INFINITY {
        ray_hit.normal() * 0.5 + Vec3::from_value(0.5)
    } else {
        sky_color(&ray)
    }
}

pub fn render_image(image: &mut RgbImage) {
    let width = image.width();
    let height = image.height();
    let aspect_ratio = width as f32 / height as f32;

    let progress_bar = ProgressBar::new((width * height) as u64);

    let mut objects = Vec::<Box<dyn Hittable>>::new();
    objects.push(Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5)));
    objects.push(Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0)));

    const CAMERA_POS: Vec3 = Vec3::ZERO;

    for y in 0..height {
        for x in 0..width {
            let u = (2.0 * x as f32 - width as f32 + 1.0) / width as f32;
            let v = -(2.0 * y as f32 - height as f32 + 1.0) / height as f32;

            let ray_dir = Vec3::new(u * aspect_ratio, v, -1.0).normalized();

            let ray = Ray::new(CAMERA_POS, ray_dir);
            let color = ray_color(&ray, &objects);

            image.put_pixel(
                x,
                y,
                Rgb([
                    (color.x() * 255.0) as u8,
                    (color.y() * 255.0) as u8,
                    (color.z() * 255.0) as u8,
                ]),
            );
            progress_bar.inc(1);
        }
    }

    progress_bar.finish();
}
