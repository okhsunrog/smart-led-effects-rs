use crate::{strip::EffectIterator, RGB8};
use palette::Hsv;
use rand_core::RngCore;

enum Direction {
    Up,
    Down,
}

/// Non-random Breathe (fixed hue)
pub struct Breathe<const N: usize> {
    colour: Hsv,
    direction: Direction,
    step: f32,
}

impl<const N: usize> Breathe<N> {
    const DEFAULT_STEP: f32 = 0.02;
    pub fn new_fixed(colour: Option<RGB8>, step_size: Option<f32>) -> Self {
        let mut colour = match colour {
            Some(rgb) => crate::utils::rgb8_to_hsv(rgb),
            None => Hsv::new(0.0, 1.0, 1.0),
        };
        colour.value = 0.0;
        Self {
            colour,
            direction: Direction::Up,
            step: step_size.unwrap_or(Self::DEFAULT_STEP),
        }
    }
}

impl<const N: usize> EffectIterator for Breathe<N> {
    fn name(&self) -> &'static str {
        "Breathe"
    }

    fn next_line(&mut self, buf: &mut [RGB8], _dt: u32) -> Option<usize> {
        match self.direction {
            Direction::Up => {
                self.colour.value += self.step;
                if self.colour.value >= 1.0 {
                    self.direction = Direction::Down;
                }
            }
            Direction::Down => {
                self.colour.value -= self.step;
                if self.colour.value <= 0.0 {
                    self.direction = Direction::Up;
                    self.colour.value = 0.0;
                }
            }
        }
        let px = crate::utils::hsv_to_rgb8_pixel(self.colour);
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

/// Random Breathe: picks a new random hue when the cycle resets.
pub struct BreatheRandom<const N: usize, R: RngCore> {
    colour: Hsv,
    rng: R,
    direction: Direction,
    step: f32,
}

impl<const N: usize, R: RngCore> BreatheRandom<N, R> {
    const DEFAULT_STEP: f32 = 0.02;
    pub fn new_random(mut rng: R, step_size: Option<f32>) -> Self {
        // initial random hue
        let hue = (rng.next_u32() as f32 / u32::MAX as f32) * 360.0;
        let mut colour = Hsv::new(hue, 1.0, 1.0);
        colour.value = 0.0;
        Self {
            colour,
            rng,
            direction: Direction::Up,
            step: step_size.unwrap_or(Self::DEFAULT_STEP),
        }
    }
}

impl<const N: usize, R: RngCore> EffectIterator for BreatheRandom<N, R> {
    fn name(&self) -> &'static str {
        "Breathe"
    }

    fn next_line(&mut self, buf: &mut [RGB8], _dt: u32) -> Option<usize> {
        match self.direction {
            Direction::Up => {
                self.colour.value += self.step;
                if self.colour.value >= 1.0 {
                    self.direction = Direction::Down;
                }
            }
            Direction::Down => {
                self.colour.value -= self.step;
                if self.colour.value <= 0.0 {
                    self.direction = Direction::Up;
                    // choose new hue
                    let hue = (self.rng.next_u32() as f32 / u32::MAX as f32) * 360.0;
                    self.colour = Hsv::new(hue, 1.0, 1.0);
                    self.colour.value = 0.0;
                }
            }
        }
        let px = crate::utils::hsv_to_rgb8_pixel(self.colour);
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
