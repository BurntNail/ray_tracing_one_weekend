//! Module for dealing with images

use crate::primitives::Decimal;

pub mod ppm;

pub trait Pixel: Default + Clone {
    ///Should be from 0-1
    fn rgb (&self) -> [Decimal; 3];
}