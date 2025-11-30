use super::SkyboxEncoder;
use crate::error::Result;
use image::Rgb32FImage;
use std::path::Path;

pub struct ExrEncoder;

impl SkyboxEncoder for ExrEncoder {
    fn encode(&self, image: &Rgb32FImage, output_path: &Path) -> Result<()> {
        image.save(output_path)?;

        Ok(())
    }
}
