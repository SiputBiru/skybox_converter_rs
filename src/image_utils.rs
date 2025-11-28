use glam::Vec3;
use image::{Rgb, Rgb32FImage};

pub fn sample_bilinear(source: &Rgb32FImage, u: f32, v: f32) -> Rgb<f32> {
    let width = source.width() as f32;
    let height = source.height() as f32;

    let x = (u * width) - 0.5;
    let y = (v * height) - 0.5;

    let x0 = x.floor();
    let y0 = y.floor();
    let tx = x - x0;
    let ty = y - y0;

    let get_pixel = |ix: f32, iy: f32| -> Vec3 {
        let final_x = (ix as i32).rem_euclid(width as i32) as u32;
        let final_y = (iy as i32).clamp(0, height as i32 - 1) as u32;
        let p = source.get_pixel(final_x, final_y);
        Vec3::new(p[0], p[1], p[2])
    };

    let c00 = get_pixel(x0, y0);
    let c10 = get_pixel(x0 + 1.0, y0);
    let c01 = get_pixel(x0, y0 + 1.0);
    let c11 = get_pixel(x0 + 1.0, y0 + 1.0);

    let top = c00.lerp(c10, tx);
    let bottom = c01.lerp(c11, tx);
    let final_color = top.lerp(bottom, ty);

    Rgb([final_color.x, final_color.y, final_color.z])
}
