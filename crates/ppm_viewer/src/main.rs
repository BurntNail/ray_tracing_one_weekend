use crate::parser::PPMImage;
use clap::Parser;
use piston_window::{clear, rectangle, PistonWindow, RenderEvent, WindowSettings};
use std::{fs::read_to_string, path::PathBuf};

mod parser;

#[derive(Parser)]
pub struct Args {
    #[arg(short, long)]
    ///The file to read from
    pub file: PathBuf,
    #[arg(short, long, default_value_t = 1600.0)]
    ///The width of the viewing window
    pub window_width: f64,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let img = PPMImage::try_from(read_to_string(args.file.clone())?.as_str())?;
    let normalised_colours = img.to_normalised_rgba();
    let scale_factor = args.window_width / img.width as f64;

    let mut window: PistonWindow = WindowSettings::new(
        args.file.to_str().unwrap_or("PPM Viewer"),
        (
            args.window_width as u32,
            (img.height as f64 * scale_factor) as u32,
        ),
    )
    .exit_on_esc(true)
    .build()?;

    while let Some(e) = window.next() {
        if e.render_args().is_some() {
            window.draw_2d(&e, |c, g, _d| {
                clear([0.0, 0.0, 0.0, 0.0], g);

                for y in 0..img.height {
                    for x in 0..img.width {
                        let rect = [x, y, x + 1, y + 1].map(|x| x as f64 * scale_factor);

                        rectangle(
                            normalised_colours[(y * img.width + x) as usize],
                            rect,
                            c.transform,
                            g,
                        );
                    }
                }
            });
        }
    }

    Ok(())
}
