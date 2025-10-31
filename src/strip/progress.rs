use crate::{strip::EffectIterator, RGB8};
use palette::{Mix, Srgb};

pub struct ProgressBar<const N: usize> {
    start_colour: Srgb,
    end_colour: Srgb,
    gradient: bool,
    pixels_per_percent: f32,
    current_value: f32,
}

impl<const N: usize> ProgressBar<N> {
    const DEFAULT_START_COLOUR: Srgb = Srgb::new(0.0, 0.0, 1.0);
    const DEFAULT_END_COLOUR: Srgb = Srgb::new(1.0, 0.0, 0.0);
    pub fn new(
        start_colour: Option<Srgb>,
        end_colour: Option<Srgb>,
        gradient: Option<bool>,
    ) -> Self {
        Self {
            start_colour: start_colour.unwrap_or(Self::DEFAULT_START_COLOUR),
            end_colour: end_colour.unwrap_or(Self::DEFAULT_END_COLOUR),
            gradient: gradient.unwrap_or(false),
            pixels_per_percent: N as f32 / 100.0,
            current_value: 0.0,
        }
    }

    pub fn set_percentage(&mut self, percentage: f32) {
        self.current_value = percentage;
    }
}

impl<const N: usize> EffectIterator for ProgressBar<N> {
    fn name(&self) -> &'static str {
        "ProgressBar"
    }

    fn next_line(&mut self, buf: &mut [RGB8], _dt: u32) -> Option<usize> {
        let percentage = self.current_value.clamp(0.0, 100.0);
        let pixels = N - (self.pixels_per_percent * (100.0 - percentage)) as usize;
        let len = core::cmp::min(N, buf.len());
        for i in 0..len {
            buf[i] = RGB8 { r: 0, g: 0, b: 0 };
        }
        if self.gradient {
            for i in 0..core::cmp::min(pixels, len) {
                let p: Srgb<u8> = self
                    .start_colour
                    .mix(self.end_colour, i as f32 / N as f32)
                    .into_format();
                buf[i] = RGB8 {
                    r: p.red,
                    g: p.green,
                    b: p.blue,
                };
            }
        } else {
            let mix = percentage / 100.0;
            let p: Srgb<u8> = self.start_colour.mix(self.end_colour, mix).into_format();
            let px = RGB8 {
                r: p.red,
                g: p.green,
                b: p.blue,
            };
            for i in 0..core::cmp::min(pixels, len) {
                buf[i] = px;
            }
        }
        Some(len)
    }

    fn pixel_count(&self) -> usize {
        N
    }
}
