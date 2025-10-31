use crate::{strip::EffectIterator, RGB8};
use palette::{FromColor, Hsv, Srgb};

pub struct Cycle<const N: usize> {
    hue: f32,
    step_size: f32,
}

impl<const N: usize> Cycle<N> {
    pub fn new(steps: Option<usize>) -> Self {
        let step = steps.unwrap_or(360);
        let step_size = 360.0 / step as f32;
        Self {
            hue: 0.0,
            step_size,
        }
    }
}

impl<const N: usize> EffectIterator for Cycle<N> {
    fn name(&self) -> &'static str {
        "Cycle"
    }

    fn next_line(&mut self, buf: &mut [RGB8], _dt: u32) -> Option<usize> {
        self.hue += self.step_size;
        if self.hue >= 360.0 {
            self.hue -= 360.0;
        }
        let hsv = Hsv::new(self.hue, 1.0, 1.0);
        let srgb8: Srgb<u8> = Srgb::from_color(hsv).into_format();
        let px = RGB8 {
            r: srgb8.red,
            g: srgb8.green,
            b: srgb8.blue,
        };
        let len = core::cmp::min(N, buf.len());
        for i in 0..len {
            buf[i] = px;
        }
        Some(len)
    }

    fn pixel_count(&self) -> usize {
        N
    }
}
