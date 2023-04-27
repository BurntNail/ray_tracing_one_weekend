#![warn(clippy::all, clippy::pedantic, clippy::nursery)] //add extra lints

use std::fs::File;
use crate::io::images::ppm::PPMImage;
use crate::primitives::Vec3;

pub mod io;
pub mod primitives;

fn main() {
    let mut image: PPMImage<Vec3> = PPMImage::new(256, 256);
    image.fun_fill();
    image.write(File::create("out.ppm").expect("unable to get file")).expect("unable to write to file");
}
