pub mod buffer;
pub mod buffer_overlay;
pub mod rand;
mod renderer;

pub use renderer::Renderer;

pub const RESOLUTION: u32 = 2000;

pub fn calc_resolution(width: usize, height: usize) -> (usize, usize, f32, f32) {
    let aspect = width as f32 / height as f32;

    let (distortion_x, distortion_y) = (aspect, 1. / aspect);

    let new_width = (distortion_x * RESOLUTION as f32).round() as usize;
    let new_height = (distortion_y * RESOLUTION as f32).round() as usize;

    (new_width, new_height, distortion_x, distortion_y)
}

// aspect = width / height
// dist_x = aspect
// dist_y = height / width
//
// cw = RES * width / height
// ch = RES * height / width
//
// ux = x * (width / cw)
// ux = x * (width / (RES * width / height))
// ux = x * (height / RES)
//
// uy = y * (width / RES)
//
// e = (cw * ch) / (width * height)
// e = ((RES * (width / height)) * RES * height / width) / (width * height)
// e = ((RES * (1 / height)) * RES * (1 / width)
// e = RES^2 / (width * height)
