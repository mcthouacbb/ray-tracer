mod math;

use std::fs::File;

use image::{ImageFormat, Rgb, RgbImage};
use indicatif::ProgressBar;

fn main() {
    const WIDTH: u32 = 800;
    const HEIGHT: u32 = 600;
    let mut image = RgbImage::new(WIDTH, HEIGHT);

    let progress_bar = ProgressBar::new((WIDTH * HEIGHT) as u64);

    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let r = x as f32 / WIDTH as f32;
            let g = y as f32 / HEIGHT as f32;
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
