//! Module for dealing with images

use crate::primitives::Backing;

pub mod ppm;

pub trait Pixel: Default + Clone {
    ///Should be from 0-1
    fn rgb (&self) -> [Backing; 3];
}