pub mod buffer;
pub mod rand;
mod renderer;

pub use renderer::Renderer;

pub const RESOLUTION: u32 = 800;

pub fn calc_resolution(width: u32, height: u32) -> (u32, u32, f32, f32) {
    let aspect = width as f32 / height as f32;

    let (distortion_x, distortion_y) = (aspect, 1. / aspect);

    let new_width = (distortion_x * RESOLUTION as f32).round() as u32;
    let new_height = (distortion_y * RESOLUTION as f32).round() as u32;
    (new_width, new_height, distortion_x, distortion_y)
}
