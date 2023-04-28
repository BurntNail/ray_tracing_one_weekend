#![warn(clippy::all, clippy::pedantic, clippy::nursery)] //add extra lints
#![allow(clippy::cast_sign_loss, clippy::cast_precision_loss)]

use crate::{
    io::images::ppm::PPMImage,
    primitives::{
        collisions::{sphere::Sphere, HittableList},
        Decimal, Vec3,
    },
};
use std::{fs::File, rc::Rc};

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

    let lower_left_corner =
        origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);

    let mut world = HittableList::default();
    world.add(Rc::new(Box::new(Sphere::new(
        Vec3::new(0.0, 0.0, -1.0),
        0.5,
    ))));
    world.add(Rc::new(Box::new(Sphere::new(
        Vec3::new(0.0, -100.5, -1.0),
        100.0,
    ))));

    let mut image: PPMImage<Vec3> = PPMImage::new(WIDTH, HEIGHT);
    image.fill(origin, lower_left_corner, horizontal, vertical, &world);
    image
        .write(File::create("out.ppm").expect("unable to get file"))
        .expect("unable to write to file");
}
