use crate::{strip::EffectIterator, RGB8};
use palette::Srgb;
use rand_core::RngCore;

pub struct Meteor<const N: usize, R: RngCore> {
    colour: Srgb,
    size: usize,
    position: usize,
    fade: f32,
    current: [Srgb; N],
    random_colour: bool,
    rng: R,
}

impl<const N: usize, R: RngCore> Meteor<N, R> {
    const DEFAULT_SIZE: usize = 4;
    const DEFAULT_FADE: f32 = 0.3;
    const DEFAULT_COLOUR: Srgb<u8> = Srgb::<u8>::new(255, 255, 255);

    pub fn new(rng: R, colour: Option<Srgb<u8>>, size: Option<usize>, fade: Option<f32>) -> Self {
        Self {
            colour: colour.unwrap_or(Self::DEFAULT_COLOUR).into_format(),
            size: size.unwrap_or(Self::DEFAULT_SIZE).min(N).max(1),
            position: 0,
            fade: fade.unwrap_or(Self::DEFAULT_FADE),
            current: [Srgb::new(0.0, 0.0, 0.0); N],
            random_colour: colour.is_none(),
            rng,
        }
    }
}

impl<const N: usize, R: RngCore> EffectIterator for Meteor<N, R> {
    fn name(&self) -> &'static str {
        "Meteor"
    }

    fn next_line(&mut self, buf: &mut [RGB8], _dt: u32) -> Option<usize> {
        // fade current trail randomly
        for pixel in self.current.iter_mut() {
            if (self.rng.next_u32() & 1) == 1 {
                pixel.red *= 1.0 - self.fade;
                pixel.green *= 1.0 - self.fade;
                pixel.blue *= 1.0 - self.fade;
            }
        }
        // draw meteor head and trailing solid size
        for i in 0..self.size {
            if self.position >= i && (self.position - i) < N {
                self.current[self.position - i] = self.colour;
            }
        }
        self.position += 1;
        if self.position > 2 * N {
            if self.random_colour {
                // random float components in [0,1)
                let rf = (self.rng.next_u32() as f32) / (u32::MAX as f32 + 1.0);
                let gf = (self.rng.next_u32() as f32) / (u32::MAX as f32 + 1.0);
                let bf = (self.rng.next_u32() as f32) / (u32::MAX as f32 + 1.0);
                self.colour = Srgb::new(rf, gf, bf);
            }
            self.position = 0;
        }
        // write to output
        let len = core::cmp::min(N, buf.len());
        for (i, slot) in buf.iter_mut().enumerate().take(len) {
            let p = self.current[i].into_format::<u8>();
            *slot = RGB8 { r: p.red, g: p.green, b: p.blue };
        }
        Some(len)
    }

    fn pixel_count(&self) -> usize {
        N
    }
}
