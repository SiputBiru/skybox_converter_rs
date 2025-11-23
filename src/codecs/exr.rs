use super::SkyboxEncoder;
use image::{ImageError, Rgb32FImage};
use std::path::Path;

pub struct ExrEncoder;

impl SkyboxEncoder for ExrEncoder {
    fn encode(&self, image: &Rgb32FImage, output_path: &Path) -> Result<(), String> {
        image
            .save(output_path)
            .map_err(|e| format!("Failed to save EXR: {}", e))
    }
}
