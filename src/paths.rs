use std::ffi::OsString;
use std::path::{Path, PathBuf};

use crate::math::CubeFace;

pub fn face_suffix(face: CubeFace) -> &'static str {
    match face {
        CubeFace::Right => "px",
        CubeFace::Left => "nx",
        CubeFace::Top => "py",
        CubeFace::Bottom => "ny",
        CubeFace::Front => "pz",
        CubeFace::Back => "nz",
    }
}

pub fn append_suffix(path: &Path, suffix: &str) -> PathBuf {
    let stem = path.file_stem().unwrap_or_default();

    let mut name = OsString::from(stem);
    name.push("_");
    name.push(suffix);

    if let Some(ext) = path.extension() {
        name.push(".");
        name.push(ext);
    }

    path.with_file_name(name)
}
