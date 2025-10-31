use crate::{strip::EffectIterator, RGB8};
use palette::{FromColor, Hsv, Srgb};
use rand_core::RngCore;

pub struct Twinkle<const N: usize, R: RngCore> {
    frequency: u8,
    probability: f32,
    fade: f32,
    colour: Option<Hsv>,
    current: [Hsv; N],
    rng: R,
}

impl<const N: usize, R: RngCore> Twinkle<N, R> {
    const DEFAULT_FREQUENCY: u8 = 0x04;
    const DEFAULT_PROBABILITY: f32 = 0.1;
    const DEFAULT_FADE: f32 = 0.02;
    pub fn new(
        rng: R,
        colour: Option<Srgb<u8>>,
        sparkle: Option<u8>,
        probability: Option<f32>,
        fade: Option<f32>,
    ) -> Self {
        Self {
            frequency: sparkle.unwrap_or(Self::DEFAULT_FREQUENCY),
            fade: fade.unwrap_or(Self::DEFAULT_FADE),
            probability: probability.unwrap_or(Self::DEFAULT_PROBABILITY),
            current: [Hsv::new(0.0, 1.0, 0.0); N],
            colour: colour.map(|colour| Hsv::from_color(colour.into_format())),
            rng,
        }
    }

    pub fn sparkle(rng: R, colour: Option<Srgb<u8>>) -> Self {
        let colour = match colour {
            Some(colour) => Some(colour),
            None => Some(Srgb::<u8>::new(255, 255, 255)),
        };
        Self::new(rng, colour, Some(20), Some(0.4), Some(1.0))
    }

    fn generate_sparkle(&mut self) {
        let index = (self.rng.next_u32() as usize) % N;
        let mut sparkle = match self.colour {
            Some(colour) => colour,
            None => {
                let hue = (self.rng.next_u32() as f32) / (u32::MAX as f32 + 1.0) * 360.0;
                Hsv::new(hue, 1.0, 0.0)
            }
        };
        let v = 0.5 + (self.rng.next_u32() as f32) / (u32::MAX as f32 + 1.0) * 0.5;
        sparkle.value = v;
        let chance = (self.rng.next_u32() as f32) / (u32::MAX as f32 + 1.0);
        if chance < self.probability {
            self.current[index] = sparkle;
        }
    }

    fn fade_sparkles(&mut self) {
        for pixel in self.current.iter_mut() {
            pixel.value = if pixel.value > self.fade {
                pixel.value - self.fade
            } else {
                0.0
            };
        }
    }
}

impl<const N: usize, R: RngCore> EffectIterator for Twinkle<N, R> {
    fn name(&self) -> &'static str {
        "Twinkle"
    }

    fn next_line(&mut self, buf: &mut [RGB8], _dt: u32) -> Option<usize> {
        self.fade_sparkles();
        let chances = (self.rng.next_u32() % self.frequency as u32) as u8;
        for _ in 0..chances {
            self.generate_sparkle();
        }
        let len = core::cmp::min(N, buf.len());
        for i in 0..len {
            let p: Srgb<u8> = Srgb::from_color(self.current[i]).into_format();
            buf[i] = RGB8 {
                r: p.red,
                g: p.green,
                b: p.blue,
            };
        }
        Some(len)
    }

    fn pixel_count(&self) -> usize {
        N
    }
}
