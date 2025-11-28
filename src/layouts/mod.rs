use image::Rgb32FImage;
pub mod cross;
pub mod separate;
pub mod strip;

use crate::image_utils::sample_bilinear;

pub enum LayoutOutput {
    Single(Rgb32FImage),
    Frames(Vec<(CubeFace, Rgb32FImage)>),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LayoutType {
    Cross,
    StripHorizontal,
    StripVertical,
    Separate,
}

pub trait SkyboxLayout {
    fn generate(&self, source: &Rgb32FImage, face_size: u32) -> LayoutOutput;
}

pub fn generate_layout(layout: LayoutType, source: &Rgb32FImage, face_size: u32) -> LayoutOutput {
    let processor: Box<dyn SkyboxLayout> = match layout {
        LayoutType::Cross => Box::new(cross::CrossLayout),

        LayoutType::StripHorizontal => Box::new(strip::StripLayout {
            direction: strip::StripDirection::Horizontal,
        }),

        LayoutType::StripVertical => Box::new(strip::StripLayout {
            direction: strip::StripDirection::Vertical,
        }),

        LayoutType::Separate => Box::new(separate::SeparateLayout),
    };

    processor.generate(source, face_size)
}

// --- SHARED HELPERS ---
use crate::math::{self, CubeFace};
use image::ImageBuffer;
use rayon::prelude::*;

pub fn render_all_faces(source: &Rgb32FImage, face_size: u32) -> Vec<(CubeFace, Rgb32FImage)> {
    let faces = vec![
        CubeFace::Right,
        CubeFace::Left,
        CubeFace::Top,
        CubeFace::Bottom,
        CubeFace::Front,
        CubeFace::Back,
    ];

    faces
        .par_iter()
        .map(|&face| {
            let buffer = extract_single_face(source, face, face_size);
            (face, buffer)
        })
        .collect()
}

fn extract_single_face(source: &Rgb32FImage, face: CubeFace, size: u32) -> Rgb32FImage {
    let mut buffer = ImageBuffer::new(size, size);

    buffer.par_enumerate_pixels_mut().for_each(|(x, y, pixel)| {
        let u = (x as f32 + 0.5) / size as f32;
        let v = (y as f32 + 0.5) / size as f32;

        let source_uv = math::calculate_source_uv(face, u, v);

        // Re-using the bilinear sampler logic
        // For production, this should be shared in a utils module
        *pixel = sample_bilinear(source, source_uv.x, source_uv.y);
    });

    buffer
}
