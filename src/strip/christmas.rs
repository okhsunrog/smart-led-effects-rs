use crate::{strip::EffectIterator, RGB8};
use palette::{Mix, Srgb};
use rand_core::RngCore;

pub struct Sparkle {
    colour: Srgb,
    intensity: f32,
    location: usize,
}

pub struct Christmas<const N: usize, const S: usize, R: RngCore> {
    frequency: u8,
    probability: f32,
    fade: f32,
    sparkles: [Option<Sparkle>; S],
    rng: R,
}

impl<const N: usize, const S: usize, R: RngCore> Christmas<N, S, R> {
    const DEFAULT_FREQUENCY: u8 = 0x04;
    const DEFAULT_PROBABILITY: f32 = 0.1;
    const DEFAULT_FADE: f32 = 0.4;
    const BACKGROUND: Srgb = Srgb::new(6.0 / 255.0, 108.0 / 255.0, 22.0 / 255.0);

    pub fn new(rng: R, sparkle: Option<u8>, probability: Option<f32>, fade: Option<f32>) -> Self {
        Self {
            frequency: sparkle.unwrap_or(Self::DEFAULT_FREQUENCY),
            fade: fade.unwrap_or(Self::DEFAULT_FADE),
            probability: probability.unwrap_or(Self::DEFAULT_PROBABILITY),
            sparkles: core::array::from_fn(|_| None),
            rng,
        }
    }

    fn generate_sparkle(&mut self) {
        let chance = (self.rng.next_u32() as f32) / (u32::MAX as f32 + 1.0);
        if chance > self.probability {
            return;
        }
        let index = (self.rng.next_u32() as usize) % N;
        let c_index = (self.rng.next_u32() as f32) / (u32::MAX as f32 + 1.0);
        let colour = if c_index < 0.5 {
            Srgb::new(1.0, 0.0, 0.0)
        } else if c_index < 0.80 {
            Srgb::new(0.0, 0.84, 1.0)
        } else {
            Srgb::new(0.0, 0.0, 1.0)
        };
        // insert into first free slot
        if let Some(slot) = self.sparkles.iter_mut().find(|s| s.is_none()) {
            *slot = Some(Sparkle {
                colour,
                intensity: 1.0,
                location: index,
            });
        }
    }

    fn fade_sparkles(&mut self) {
        for s in self.sparkles.iter_mut() {
            if let Some(sp) = s {
                sp.intensity -= self.fade;
                if sp.intensity <= 0.0 {
                    *s = None;
                }
            }
        }
    }
}

impl<const N: usize, const S: usize, R: RngCore> EffectIterator for Christmas<N, S, R> {
    fn name(&self) -> &'static str {
        "Christmas"
    }

    fn next_line(&mut self, buf: &mut [RGB8], _dt: u32) -> Option<usize> {
        self.fade_sparkles();
        let chances = (self.rng.next_u32() % self.frequency as u32) as u8;
        for _ in 0..chances {
            self.generate_sparkle();
        }
        let len = core::cmp::min(N, buf.len());
        // background
        let base: Srgb<u8> = Self::BACKGROUND.into_format();
        for slot in buf.iter_mut().take(len) {
            *slot = RGB8 { r: base.red, g: base.green, b: base.blue };
        }
        // add sparkles by mixing onto background
        for s in self.sparkles.iter().filter_map(|x| x.as_ref()) {
            if s.location < len {
                let bg: Srgb = Self::BACKGROUND;
                let mixed: Srgb<u8> = bg.mix(s.colour, s.intensity).into_format();
                buf[s.location] = RGB8 {
                    r: mixed.red,
                    g: mixed.green,
                    b: mixed.blue,
                };
            }
        }
        Some(len)
    }

    fn pixel_count(&self) -> usize {
        N
    }
}
