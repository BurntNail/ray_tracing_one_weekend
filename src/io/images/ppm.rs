use std::io;
use std::io::Write;
use std::ops::{Index, IndexMut};
use crate::io::images::{Pixel};
use crate::primitives::{Colour, Vec3};

pub struct PPMImage<P: Pixel> {
    width: usize,
    height: usize,
    pixels: Vec<P>
}

fn write_ppm_pixel (p: impl Pixel, w: &mut impl Write) -> io::Result<()> {
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
        if x >= self.width || y >= self.height {
            panic!("image pos oob");
        }

        &self.pixels[y * self.width + x]
    }
}
impl<P: Pixel> IndexMut<(usize, usize)> for PPMImage<P> {
    fn index_mut(&mut self, (x, y): (usize, usize)) -> &mut Self::Output {
        if x >= self.width || y >= self.height {
            panic!("image pos oob");
        }

        &mut self.pixels[y * self.width + x]
    }
}

impl<P: Pixel> PPMImage<P> {
    pub fn new (width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            pixels: vec![P::default(); width * height]
        }
    }

    pub fn write (&self, mut w: impl Write) -> io::Result<()> {
        writeln!(w, "P3\n{width} {height}\n255", width=self.width, height=self.height)?;

        for y in (0..self.height).rev() {
            for x in 0..self.width {
                write_ppm_pixel(self[(x, y)].clone(), &mut w)?;
            }
        }


        Ok(())
    }
}

impl PPMImage<Vec3> {
    pub fn fun_fill (&mut self) {
        for x in 0..self.width {
            for y in 0..self.height {
                self[(x, y)] = Colour::new(x as f32 / self.width as f32, y as f32 / self.height as f32, 0.25);
            }
        }
    }
}