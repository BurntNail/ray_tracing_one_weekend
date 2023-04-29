#![warn(clippy::all, clippy::pedantic, clippy::nursery)] //add extra lints
#![allow(clippy::cast_sign_loss, clippy::cast_precision_loss)]

use crate::{
    io::images::ppm::PPMImage,
    primitives::{
        camera::Camera,
        collisions::{sphere::Sphere, HittableList},
        Decimal, Vec3,
    },
};
use std::{fs::File, rc::Rc};

pub mod io;
pub mod primitives;

fn main() {
    const ASPECT_RATIO: Decimal = Camera::ASPECT_RATIO;
    const WIDTH: usize = 400;
    const HEIGHT: usize = (WIDTH as Decimal / ASPECT_RATIO) as usize;

    const SAMPLES_PER_PIXEL: usize = 100;

    let cam = Camera::default();

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
    image.fill(&cam, &world, SAMPLES_PER_PIXEL);
    image
        .write(
            File::create("out.ppm").expect("unable to get file"),
            SAMPLES_PER_PIXEL,
        )
        .expect("unable to write to file");
}
