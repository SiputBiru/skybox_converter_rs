pub mod codecs;
pub mod image_utils;
pub mod layouts;
pub mod math;
mod paths;

pub use codecs::{OutputFormat, ToneMapType, get_encoder};
pub use layouts::{LayoutType, generate_layout};
pub use math::CubeFace;

use std::path::PathBuf;
use thiserror::Error;

#[derive(Debug)]
pub struct Config {
    pub input: PathBuf,
    pub output: PathBuf,
    pub format: OutputFormat,
    pub layout: LayoutType,
    pub tonemap: ToneMapType,
    pub exposure: f32,
    pub size: u32,
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("failed to load image: {0}")]
    ImageLoad(#[from] image::ImageError),

    #[error("encode error: {0}")]
    Encode(String),

    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
}

pub fn run(config: Config) -> Result<(), Error> {
    use rayon::prelude::*;

    println!("Loading {}...", config.input.display());
    let img = image::open(&config.input)?.into_rgb32f();

    // Stats
    let raw_pixels = img.as_raw();
    let max_brightness = raw_pixels
        .par_iter()
        .cloned()
        .reduce(|| 0.0f32, |a, b| a.max(b));
    if max_brightness > 10.0 {
        let suggested = 1.0 / (max_brightness * 0.1);
        println!(
            "Note: Max Brightness = {:.2}. Recommended exposure: ~{:.4}",
            max_brightness, suggested
        );
    }

    println!("Generating layout...");

    let layout_output = generate_layout(config.layout, &img, config.size);

    println!(
        "Encoding to output (Tone Map: {:?}, Exposure: {})...",
        config.tonemap, config.exposure
    );

    let encoder = get_encoder(config.format, config.tonemap, config.exposure);

    match layout_output {
        layouts::LayoutOutput::Single(buffer) => {
            encoder
                .encode(&buffer, &config.output)
                .map_err(Error::Encode)?;
            println!("Success! Saved to {}", config.output.display());
        }

        layouts::LayoutOutput::Frames(faces) => {
            for (face, buffer) in faces {
                let suffix = paths::face_suffix(face);
                let new_path = paths::append_suffix(&config.output, suffix);

                encoder.encode(&buffer, &new_path).map_err(Error::Encode)?;
                println!("Saved {}", new_path.display());
            }
        }
    }

    Ok(())
}
