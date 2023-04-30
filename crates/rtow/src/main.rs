#![warn(clippy::all, clippy::pedantic, clippy::nursery)] //add extra lints
#![allow(clippy::cast_sign_loss, clippy::cast_precision_loss)]

use crate::{
    io::images::ppm::PPMImage,
    primitives::{
        camera::Camera,
        collisions::{sphere::Sphere, HittableList},
        materials::Material,
        Colour, Decimal, Vec3,
    },
};
use std::{fs::File, sync::Arc};

pub mod io;
pub mod primitives;

fn main() {
    const ASPECT_RATIO: Decimal = Camera::ASPECT_RATIO;
    const WIDTH: usize = 400;
    const HEIGHT: usize = (WIDTH as Decimal / ASPECT_RATIO) as usize;

    const SAMPLES_PER_PIXEL: usize = 100;
    const MAX_DEPTH: usize = 50;

    let cam = Camera::default();
    let ground_mat = Material::LambertianDiffuse(Colour::new(0.8, 0.8, 0.0));
    let centre_mat = Material::LambertianDiffuse(Colour::new(0.7, 0.3, 0.3));
    let left_mat = Material::Metal(Colour::new(0.8, 0.8, 0.8), 0.3);
    let right_mat = Material::Metal(Colour::new(0.8, 0.6, 0.2), 1.0);

    let mut world = HittableList::default();
    world.add(Arc::new(Box::new(Sphere::new(
        Vec3::new(0.0, -100.5, -1.0),
        100.0,
        ground_mat,
    ))));
    world.add(Arc::new(Box::new(Sphere::new(
        Vec3::new(0.0, 0.0, -1.0),
        0.5,
        centre_mat,
    ))));
    world.add(Arc::new(Box::new(Sphere::new(
        Vec3::new(-1.0, 0.0, -1.0),
        0.5,
        left_mat,
    ))));
    world.add(Arc::new(Box::new(Sphere::new(
        Vec3::new(1.0, 0.0, -1.0),
        0.5,
        right_mat,
    ))));

    let mut image: PPMImage<Vec3> = PPMImage::new(WIDTH, HEIGHT);
    image.fill(&cam, &world, SAMPLES_PER_PIXEL, MAX_DEPTH);
    image
        .write(
            File::create("out.ppm").expect("unable to get file"),
            SAMPLES_PER_PIXEL,
        )
        .expect("unable to write to file");
}