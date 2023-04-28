#![warn(clippy::all, clippy::pedantic, clippy::nursery)] //add extra lints
#![allow(clippy::cast_sign_loss, clippy::cast_precision_loss)]

use std::fs::File;
use crate::io::images::ppm::PPMImage;
use crate::primitives::{Decimal, Vec3};

pub mod io;
pub mod primitives;

fn main() {
    const ASPECT_RATIO: Decimal = 16.0 / 9.0;
    const WIDTH: usize = 400;
    const HEIGHT: usize = (WIDTH as Decimal / ASPECT_RATIO) as usize;

    let viewport_height = 2.0;
    let viewport_width = viewport_height * ASPECT_RATIO;
    let focal_length = 1.0;


    let origin = Vec3::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);

    let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);
    

    let mut image: PPMImage<Vec3> = PPMImage::new(WIDTH, HEIGHT);
    image.fun_fill(origin, lower_left_corner, horizontal, vertical);
    image.write(File::create("out.ppm").expect("unable to get file")).expect("unable to write to file");
}
