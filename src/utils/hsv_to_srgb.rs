use crate::RGB8;
use palette::{FromColor, Hsv, Srgb};

/// Convert a single HSV to RGB8 (smart-leds-trait pixel)
pub fn hsv_to_rgb8_pixel(hsv: Hsv) -> RGB8 {
    let srgb8: Srgb<u8> = Srgb::from_color(hsv).into_format::<u8>();
    RGB8 {
        r: srgb8.red,
        g: srgb8.green,
        b: srgb8.blue,
    }
}

// (Intentionally minimal: slice conversion helper removed.)
