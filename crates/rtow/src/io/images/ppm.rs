use crate::{
    io::images::Pixel,
    primitives::{camera::Camera, collisions::Hittable, Colour, Decimal, Vec3},
};
use indicatif::{ProgressBar, ProgressStyle};
use rand::{thread_rng, Rng};
use rayon::prelude::{IndexedParallelIterator, IntoParallelIterator, ParallelIterator};
use std::{
    io,
    io::Write,
    ops::{Index, IndexMut},
    sync::mpsc::channel,
};

///Struct to hold a PPM-Based Image
pub struct PPMImage<P: Pixel> {
    width: usize,
    height: usize,
    ///Our pixels, stored in row-major configuration
    pixels: Vec<P>,
}

///Utility function to write a PPM Pixel, including newline
fn write_ppm_pixel(
    pixel: &impl Pixel,
    write: &mut impl Write,
    samples_per_pixel: usize,
) -> io::Result<()> {
    let [red, green, blue] = pixel
        .rgb()
        .map(|colour_value| 1.0 / (samples_per_pixel as Decimal) * colour_value)
        .map(Decimal::sqrt); //gamma correction

    let red = (red * 259.99) as u32;
    let green = (green * 259.99) as u32;
    let blue = (blue * 259.99) as u32;

    writeln!(write, "{red} {green} {blue}")?;

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
    pub fn write(&self, mut w: impl Write, samples_per_pixel: usize) -> io::Result<()> {
        writeln!(
            w,
            "P3\n{width} {height}\n255",
            width = self.width,
            height = self.height
        )?;

        for y in (0..self.height).rev() {
            for x in 0..self.width {
                write_ppm_pixel(&self[(x, y)], &mut w, samples_per_pixel)?;
            }
        }

        Ok(())
    }
}

impl PPMImage<Vec3> {
    pub fn fill(
        &mut self,
        camera: &Camera,
        world: &dyn Hittable,
        samples_per_pixel: usize,
        max_depth: usize,
    ) {
        let no = (self.width * self.height) as u64;
        let progress_bar = ProgressBar::new(no); //make a new progress bar with the number of runs we expect to do
        progress_bar.set_style(
            ProgressStyle::with_template(
                "{spinner} Elapsed: [{elapsed_precise}], ETA: [{eta_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7}",
            )
                .unwrap()
                .progress_chars("##-"),
        );

        let (tx, rx) = channel();

        (0..self.width)
            .into_par_iter()
            .chunks(self.width / 2)
            .for_each_with((tx, world), |(tx, world), rows| {
                let world = world.clone();
                let mut rng = thread_rng();

                for x in rows {
                    for y in 0..self.height {
                        let mut colour = Colour::new(0.0, 0.0, 0.0);

                        for _ in 0..samples_per_pixel {
                            let u = (x as Decimal + rng.gen_range(0.0..=1.0))
                                / (self.width - 1) as Decimal;
                            let v = (y as Decimal + rng.gen_range(0.0..=1.0))
                                / (self.height - 1) as Decimal;

                            let ray = camera.get_ray(u, v);
                            colour += ray.colour(world, max_depth);
                        }

                        progress_bar.inc(1);
                        tx.send((x, y, colour)).expect("unable to send");
                    }
                }
            });

        let mut recv_count = 0;
        while recv_count < no {
            for (x, y, colour) in rx.try_iter() {
                self[(x, y)] = colour;
                recv_count += 1;
            }
        }

        progress_bar.finish_and_clear();
    }
}
