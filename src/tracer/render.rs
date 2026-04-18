use image::{Rgb, RgbImage};
use indicatif::ProgressBar;

use crate::math::Vec3;

pub fn render_image(image: &mut RgbImage) {
    let width = image.width();
    let height = image.height();
    let aspect_ratio = width as f32 / height as f32;

    let progress_bar = ProgressBar::new((width * height) as u64);

    for y in 0..height {
        for x in 0..width {
            let u = x as f32 - (width as f32 - 1.0) / 2.0;
            let v = y as f32 - (height as f32 - 1.0) / 2.0;

            let ray_dir = Vec3::new(u / aspect_ratio, v, -1.0).normalized();

            let r = ray_dir.x() * 0.5 + 0.5;
            let g = ray_dir.y() * 0.5 + 0.5;
            let b = ray_dir.z() * 0.5 + 0.5;

            image.put_pixel(
                x,
                y,
                Rgb([(r * 255.0) as u8, (g * 255.0) as u8, (b * 255.0) as u8]),
            );
            progress_bar.inc(1);
        }
    }

    progress_bar.finish();
}
