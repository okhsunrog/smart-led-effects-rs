use crate::{strip::EffectIterator, RGB8};
use palette::Hsv;
use rand_core::RngCore;

pub struct SnowSparkle<const N: usize, R: RngCore> {
    frequency: u8,
    probability: f32,
    fade: f32,
    colour: Hsv,
    current: [Hsv; N],
    rng: R,
}

impl<const N: usize, R: RngCore> SnowSparkle<N, R> {
    const DEFAULT_FREQUENCY: u8 = 0x04;
    const DEFAULT_PROBABILITY: f32 = 0.1;
    const DEFAULT_FADE: f32 = 0.4;
    const BASE_BRIGHTNESS: f32 = 0.2;

    pub fn new(
        mut rng: R,
        colour: Option<RGB8>,
        sparkle: Option<u8>,
        probability: Option<f32>,
        fade: Option<f32>,
    ) -> Self {
        let mut colour = match colour {
            Some(colour) => crate::utils::rgb8_to_hsv(colour),
            None => Hsv::new(0.0, 0.0, 1.0),
        };
        colour.value = Self::BASE_BRIGHTNESS;
        let arr = [colour; N];
        Self {
            frequency: sparkle.unwrap_or(Self::DEFAULT_FREQUENCY),
            fade: fade.unwrap_or(Self::DEFAULT_FADE),
            probability: probability.unwrap_or(Self::DEFAULT_PROBABILITY),
            current: arr,
            colour,
            rng,
        }
    }

    pub fn sparkle(mut rng: R, colour: Option<RGB8>) -> Self {
        let colour = match colour {
            Some(colour) => Some(colour),
            None => Some(RGB8 {
                r: 255,
                g: 255,
                b: 255,
            }),
        };
        Self::new(rng, colour, Some(20), Some(0.4), Some(1.0))
    }

    fn generate_sparkle(&mut self) {
        let idx = (self.rng.next_u32() as usize) % N;
        let mut sparkle = self.colour;
        // random value in [0.5, 1.0)
        let v = 0.5 + (self.rng.next_u32() as f32 / u32::MAX as f32) * 0.5;
        sparkle.value = v;
        let chance = self.rng.next_u32() as f32 / u32::MAX as f32;
        if chance < self.probability {
            self.current[idx] = sparkle;
        }
    }

    fn fade_sparkles(&mut self) {
        for pixel in self.current.iter_mut() {
            pixel.value = (pixel.value - self.fade).max(Self::BASE_BRIGHTNESS);
        }
    }
}

impl<const N: usize, R: RngCore> EffectIterator for SnowSparkle<N, R> {
    fn name(&self) -> &'static str {
        "SnowSparkle"
    }

    fn next_line(&mut self, buf: &mut [RGB8], _dt: u32) -> Option<usize> {
        self.fade_sparkles();
        // 0..frequency random sparkles per frame
        let chances = (self.rng.next_u32() % self.frequency as u32) as u8;
        for _ in 0..chances {
            self.generate_sparkle();
        }
        let len = core::cmp::min(N, buf.len());
        for i in 0..len {
            buf[i] = crate::utils::hsv_to_rgb8_pixel(self.current[i]);
        }
        Some(len)
    }

    fn pixel_count(&self) -> usize {
        N
    }
}
