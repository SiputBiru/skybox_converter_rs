use crate::error::Result;
use image::Rgb32FImage;
use std::path::Path;

pub mod exr;
pub mod png;
pub mod tonemap;

pub use tonemap::ToneMapType;

#[derive(Debug, Clone, Copy)]
pub enum OutputFormat {
    Png,
    Exr,
}

pub trait SkyboxEncoder {
    fn encode(&self, image: &Rgb32FImage, output_path: &Path) -> Result<()>;
}

pub fn get_encoder(
    format: OutputFormat,
    tonemap: ToneMapType,
    exposure: f32,
) -> Box<dyn SkyboxEncoder> {
    match format {
        OutputFormat::Png => Box::new(png::PngEncoder { tonemap, exposure }),
        OutputFormat::Exr => Box::new(exr::ExrEncoder),
    }
}
