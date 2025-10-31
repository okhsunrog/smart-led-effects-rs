use crate::{strip::EffectIterator, RGB8};
use palette::{FromColor, Hsv, Srgb};

#[derive(Debug, PartialEq)]
enum Direction {
    Forward,
    Backward,
}

impl Direction {
    fn next(&mut self) {
        match self {
            Direction::Forward => *self = Direction::Backward,
            _ => *self = Direction::Forward,
        }
    }
}

pub struct Cylon<const N: usize> {
    colour: Hsv,
    direction: Direction,
    start: usize,
    size: usize,
    fade: f32,
}

impl<const N: usize> Cylon<N> {
    const DEFAULT_SIZE: usize = 4;
    const DEFAULT_FADE: f32 = 0.2;

    pub fn new(colour: Srgb<u8>, size: Option<usize>, fade: Option<f32>) -> Self {
        let size = size.unwrap_or(Self::DEFAULT_SIZE).min(N).max(1);
        Self {
            colour: Hsv::from_color(colour.into_format()),
            start: size - 1,
            direction: Direction::Forward,
            size,
            fade: fade.unwrap_or(Self::DEFAULT_FADE),
        }
    }
}

impl<const N: usize> EffectIterator for Cylon<N> {
    fn name(&self) -> &'static str {
        "Cylon"
    }

    fn next_line(&mut self, buf: &mut [RGB8], _dt: u32) -> Option<usize> {
        let len = core::cmp::min(N, buf.len());
        // advance position
        match self.direction {
            Direction::Forward => {
                if self.start == N - 1 {
                    self.direction.next();
                } else {
                    self.start += 1;
                }
            }
            Direction::Backward => {
                if self.start == self.size - 1 {
                    self.direction.next();
                } else {
                    self.start -= 1;
                }
            }
        }

        // render
        for i in 0..len {
            let mut brightness = 0.0f32;
            // head segment [start - size + 1 ..= start]
            if self.start + 1 >= self.size {
                let head_start = self.start + 1 - self.size;
                if i >= head_start && i <= self.start {
                    brightness = 1.0;
                }
            }
            // trail
            let mut val = 0.8f32;
            match self.direction {
                Direction::Forward => {
                    if self.start + 1 > self.size {
                        let tail_end = self.start + 1 - self.size; // inclusive index behind head
                        if i <= tail_end {
                            let dist = (tail_end - i) as f32;
                            val = (0.8 - self.fade * dist).max(0.0);
                            brightness += val;
                        }
                    }
                }
                Direction::Backward => {
                    if i >= self.start {
                        let dist = (i - self.start) as f32;
                        val = (0.8 - self.fade * dist).max(0.0);
                        brightness += val;
                    }
                }
            }
            let mut hsv = self.colour;
            hsv.value = brightness.min(1.0);
            let srgb8: Srgb<u8> = Srgb::from_color(hsv).into_format();
            buf[i] = RGB8 {
                r: srgb8.red,
                g: srgb8.green,
                b: srgb8.blue,
            };
        }
        Some(len)
    }

    fn pixel_count(&self) -> usize {
        N
    }
}
