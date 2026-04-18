use image::{Rgb, RgbImage};
use indicatif::ProgressBar;

pub fn render_image(image: &mut RgbImage) {
    let width = image.width();
    let height = image.height();

    let progress_bar = ProgressBar::new((width * height) as u64);

    for y in 0..height {
        for x in 0..width {
            let r = x as f32 / width as f32;
            let g = y as f32 / height as f32;
            let b = 0.0;
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
