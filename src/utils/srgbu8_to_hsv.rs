use crate::RGB8;
use palette::{FromColor, Hsv, Srgb};

pub fn rgb8_to_hsv(input: RGB8) -> Hsv {
    let srgb = Srgb::<u8>::new(input.r, input.g, input.b);
    Hsv::from_color(srgb.into_format())
}
