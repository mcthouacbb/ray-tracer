use std::{
    collections::VecDeque,
    f32,
    sync::{
        Arc, Mutex,
        atomic::{AtomicU32, Ordering},
    },
    thread,
};

use image::{Rgb, RgbImage};
use indicatif::ProgressBar;
use rand::{RngExt, SeedableRng, rngs::Xoshiro256PlusPlus};

use crate::{
    math::Vec3,
    tracer::{camera::Camera, ray::Ray, scene::Scene},
    transform::Transform,
};

pub fn sky_color(ray: &Ray) -> Vec3 {
    let a = 0.5 * ray.dir().normalized().y() + 0.5;
    (1.0 - a) * Vec3::new(1.0, 1.0, 1.0) + a * Vec3::new(0.5, 0.7, 1.0)
}

pub fn linear_to_srgb(linear: f32) -> f32 {
    if linear < 0.0031308 {
        linear * 12.92
    } else {
        1.055 * linear.powf(0.41666) - 0.055
    }
}

pub fn ray_color(ray: &Ray, scene: &Scene, rng: &mut impl RngExt, depth: u32) -> Vec3 {
    if depth == 0 {
        return Vec3::ZERO;
    }

    let (ray_hit, scene_hit) = scene.trace(ray);

    if ray_hit.dist() < f32::INFINITY {
        let scene_hit = scene_hit.unwrap();
        let scatter_color = match scene_hit
            .material()
            .scatter(&ray, &ray_hit, &scene_hit, rng)
        {
            Some(scatter_result) => {
                let sub_color = ray_color(scatter_result.scattered_ray(), scene, rng, depth - 1);
                scatter_result.attenuation().pairwise(&sub_color)
            }
            None => Vec3::ZERO,
        };
        let emissive_color = scene_hit.material().emitted();
        emissive_color + scatter_color
    } else {
        0.35 * sky_color(&ray)
    }
}

pub fn render_image(
    image: &mut RgbImage,
    camera: &Camera,
    camera_transform: &Transform,
    scene: &Scene,
    spp: u32,
    max_depth: u32,
    num_threads: u32,
) {
    let width = image.width();
    let height = image.height();
    assert!(width % 4 == 0 && height % 4 == 0);

    let progress_bar = ProgressBar::new((width * height) as u64);

    let tiles = (0..width * height / 16)
        .map(|tile_idx| (tile_idx % (width / 4), tile_idx / (width / 4)))
        .collect::<VecDeque<(u32, u32)>>();

    let tile_queue = Arc::new(Mutex::new(tiles));

    let pixel_buffer = (0..width * height)
        .map(|_| AtomicU32::new(0))
        .collect::<Vec<AtomicU32>>();

    thread::scope(|s| {
        for _ in 0..num_threads {
            s.spawn(|| {
                let mut rng = Xoshiro256PlusPlus::from_rng(&mut rand::rng());

                loop {
                    let mut result = tile_queue.lock().unwrap();
                    if result.is_empty() {
                        break;
                    }

                    let tile = result.pop_front().unwrap();
                    std::mem::drop(result);

                    for py in 0..4 {
                        for px in 0..4 {
                            let x = 4 * tile.0 + px;
                            let y = 4 * tile.1 + py;

                            let mut accum_color = Vec3::ZERO;

                            for _ in 0..spp {
                                let jitter_x = rng.random_range(-0.5..=0.5f32);
                                let jitter_y = rng.random_range(-0.5..=0.5f32);
                                let u = (2.0 * (x as f32 + jitter_x) as f32 - width as f32 + 1.0)
                                    / width as f32;
                                let v = -(2.0 * (y as f32 + jitter_y) as f32 - height as f32 + 1.0)
                                    / height as f32;

                                let camera_ray = camera.get_ray_dir(u, v, &mut rng);
                                let camera_mat = camera_transform.transform();
                                let ray = Ray::new(
                                    camera_mat.transform_pos(&camera_ray.origin()),
                                    camera_mat.transform_dir(&camera_ray.dir()),
                                );

                                let color = ray_color(&ray, &scene, &mut rng, max_depth);
                                accum_color += color;
                            }

                            let pixel_color = accum_color / spp as f32;
                            let r = (linear_to_srgb(pixel_color.x()) * 255.0 + 0.5) as u8;
                            let g = (linear_to_srgb(pixel_color.y()) * 255.0 + 0.5) as u8;
                            let b = (linear_to_srgb(pixel_color.z()) * 255.0 + 0.5) as u8;
                            let rgb_u32 = (r as u32) << 16 | (g as u32) << 8 | b as u32;
                            pixel_buffer[(y * width + x) as usize]
                                .store(rgb_u32, Ordering::Relaxed);
                        }
                    }

                    progress_bar.inc(16);
                }
            });
        }
    });

    for y in 0..height {
        for x in 0..width {
            let rgb_u32 = pixel_buffer[(y * width + x) as usize].load(Ordering::Relaxed);
            let r = rgb_u32 >> 16;
            let g = (rgb_u32 >> 8) & 0xFF;
            let b = rgb_u32 & 0xFF;
            image.put_pixel(x, y, Rgb([r as u8, g as u8, b as u8]));
        }
    }

    progress_bar.finish();
}
