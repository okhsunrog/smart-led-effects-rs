use crate::{strip::EffectIterator, RGB8};
use rand_core::RngCore;

pub struct Fire<const N: usize, R: RngCore> {
    cooling: u8,
    sparking: u8,
    heat: [u8; N],
    rng: R,
}

impl<const N: usize, R: RngCore> Fire<N, R> {
    const DEFAULT_COOLING: u8 = 40;
    const DEFAULT_SPARKING: u8 = 120;
    pub fn new(rng: R, cooling: Option<u8>, sparking: Option<u8>) -> Self {
        let base_cool = cooling.unwrap_or(Self::DEFAULT_COOLING) as f32;
        let cooling = (((base_cool * 10.0) / N as f32) + 2.0) as u8;
        Self {
            cooling,
            sparking: sparking.unwrap_or(Self::DEFAULT_SPARKING),
            heat: [0; N],
            rng,
        }
    }

    fn heat_to_colour(val: u8) -> RGB8 {
        let (r, g, b) = if val >= 0x85 {
            let heat_ramp = 3u8.saturating_mul(val - 0x85);
            (255, 255, heat_ramp)
        } else if val >= 0x40 {
            let heat_ramp = 3u8.saturating_mul(val - 0x40);
            (255, heat_ramp, 0)
        } else {
            let heat_ramp = 3u8.saturating_mul(val);
            (heat_ramp, 0, 0)
        };
        RGB8 { r, g, b }
    }
}

impl<const N: usize, R: RngCore> EffectIterator for Fire<N, R> {
    fn name(&self) -> &'static str {
        "Fire"
    }

    fn next_line(&mut self, buf: &mut [RGB8], _dt: u32) -> Option<usize> {
        // apply cooling
        for spark in self.heat.iter_mut() {
            let x = (self.rng.next_u32() % self.cooling as u32) as u8;
            *spark = spark.saturating_sub(x);
        }
        // apply heating diffusion
        for i in (2..N).rev() {
            self.heat[i] = (self.heat[i - 1]
                .saturating_add(self.heat[i - 2])
                .saturating_add(self.heat[i - 2]))
                / 3;
        }
        // generate sparks
        if ((self.rng.next_u32() % 255) as u8) < self.sparking {
            let y = (self.rng.next_u32() as usize) % ((N.max(1) / 7) + 1);
            let add = 160 + (self.rng.next_u32() % 95) as u8; // 160..255
            self.heat[y] = self.heat[y].saturating_add(add);
        }
        // write colours
        let len = core::cmp::min(N, buf.len());
        for i in 0..len {
            buf[i] = Self::heat_to_colour(self.heat[i]);
        }
        Some(len)
    }

    fn pixel_count(&self) -> usize {
        N
    }
}
