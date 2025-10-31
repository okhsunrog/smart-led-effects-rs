use crate::{strip::EffectIterator, utils::hsv_to_rgb8_pixel, RGB8};
use palette::Hsv;

pub struct RunningLights<const N: usize> {
    colour: Hsv,
    position: usize,
    reverse: bool,
    period: usize,
}

impl<const N: usize> RunningLights<N> {
    pub fn new(colour: Option<RGB8>, reverse: bool, period: Option<usize>) -> Self {
        Self {
            colour: match colour {
                Some(rgb) => crate::utils::rgb8_to_hsv(rgb),
                None => Hsv::new(0.0, 0.0, 1.0),
            },
            position: if reverse { N } else { 0 },
            reverse,
            period: period.unwrap_or(N.max(1)),
        }
    }
}

impl<const N: usize> EffectIterator for RunningLights<N> {
    fn name(&self) -> &'static str {
        "RunningLights"
    }

    fn next_line(&mut self, buf: &mut [RGB8], _dt: u32) -> Option<usize> {
        let len = core::cmp::min(N, buf.len());
        for i in 0..len {
            let phase = (i + self.position) % self.period;
            let half = self.period / 2;
            let brightness = if phase < half {
                (phase as f32) / (half as f32)
            } else {
                ((self.period - phase) as f32) / (half as f32)
            };
            let mut hsv = self.colour;
            hsv.value = brightness.min(1.0).max(0.0);
            buf[i] = hsv_to_rgb8_pixel(hsv);
        }
        if self.reverse {
            if self.position == 0 {
                self.position = self.period;
            } else {
                self.position -= 1;
            }
        } else {
            self.position += 1;
            if self.position >= self.period {
                self.position = 0;
            }
        }
        Some(len)
    }

    fn pixel_count(&self) -> usize {
        N
    }
}
