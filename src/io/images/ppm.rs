use crate::{
    io::images::Pixel,
    primitives::{collisions::Hittable, Ray, Vec3},
};
use std::{
    io,
    io::Write,
    ops::{Index, IndexMut},
};

///Struct to hold a PPM-Based Image
pub struct PPMImage<P: Pixel> {
    width: usize,
    height: usize,
    ///Our pixels, stored in row-major configuration
    pixels: Vec<P>,
}

///Utility function to write a PPM Pixel, including newline
fn write_ppm_pixel(p: impl Pixel, w: &mut impl Write) -> io::Result<()> {
    let [r, g, b] = p.rgb();
    let r = (r * 259.99) as u32;
    let g = (g * 259.99) as u32;
    let b = (b * 259.99) as u32;

    writeln!(w, "{r} {g} {b}")?;

    Ok(())
}

impl<P: Pixel> Index<(usize, usize)> for PPMImage<P> {
    type Output = P;

    fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
        assert!(!(x >= self.width || y >= self.height), "image pos oob");

        &self.pixels[y * self.width + x]
    }
}
impl<P: Pixel> IndexMut<(usize, usize)> for PPMImage<P> {
    fn index_mut(&mut self, (x, y): (usize, usize)) -> &mut Self::Output {
        assert!(!(x >= self.width || y >= self.height), "image pos oob");

        &mut self.pixels[y * self.width + x]
    }
}

impl<P: Pixel> PPMImage<P> {
    #[must_use]
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            pixels: vec![P::default(); width * height],
        }
    }

    ///Function to write the PPM to a given [`Write`] object
    ///
    /// # Errors
    /// If we fail to write to the object, we bubble it up
    pub fn write(&self, mut w: impl Write) -> io::Result<()> {
        writeln!(
            w,
            "P3\n{width} {height}\n255",
            width = self.width,
            height = self.height
        )?;

        for y in (0..self.height).rev() {
            for x in 0..self.width {
                write_ppm_pixel(self[(x, y)].clone(), &mut w)?;
            }
        }

        Ok(())
    }
}

impl PPMImage<Vec3> {
    pub fn fill(
        &mut self,
        origin: Vec3,
        lower_left_corner: Vec3,
        horizontal: Vec3,
        vertical: Vec3,
        world: &dyn Hittable,
    ) {
        for x in 0..self.width {
            for y in 0..self.height {
                let u = x as f32 / (self.width - 1) as f32;
                let v = y as f32 / (self.height - 1) as f32;

                let ray = Ray::new(
                    origin,
                    lower_left_corner + u * horizontal + v * vertical - origin,
                );
                self[(x, y)] = ray.colour(world);
            }
        }
    }
}
