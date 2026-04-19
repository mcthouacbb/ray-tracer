mod math;
mod tracer;

use std::fs::File;

use image::{ImageFormat, Rgb, RgbImage};
use indicatif::ProgressBar;

use crate::tracer::render::render_image;

fn main() {
    const WIDTH: u32 = 800;
    const HEIGHT: u32 = 600;
    let mut image = RgbImage::new(WIDTH, HEIGHT);
    render_image(&mut image);

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
