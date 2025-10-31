use palette::{FromColor, Hsv, Srgb};
use crate::RGB8;

/// Convert a single HSV to RGB8 (smart-leds-trait pixel)
pub fn hsv_to_rgb8_pixel(hsv: Hsv) -> RGB8 {
    let srgb8: Srgb<u8> = Srgb::from_color(hsv).into_format::<u8>();
    RGB8 { r: srgb8.red, g: srgb8.green, b: srgb8.blue }
}

/// Convert a slice of HSVs into an output buffer of RGB8.
/// Lengths must match; extra output elements are left untouched.
pub fn hsv_slice_to_rgb8(out: &mut [RGB8], src: &[Hsv]) {
    let len = core::cmp::min(out.len(), src.len());
    for i in 0..len {
        out[i] = hsv_to_rgb8_pixel(src[i]);
    }
}
