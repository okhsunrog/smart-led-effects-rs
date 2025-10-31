use super::EffectIterator;
use palette::{FromColor, Hsv, Srgb};
use rand_core::RngCore;

/// Strobe effect
///
/// This effect flashes the whole strip in a given colour or random colour if None is supplied.
///
/// # Arguments
///
/// * `count` - The number of pixels in the strip.
/// * `colour` - The colour to flash. If `None` a random colour will be used.
/// * `period` - The period of the strobe.
/// * `decay` - The rate at which the colour fades. If `None` the default value of `0.02` per tick will be used.
///
/// # Examples
///
/// ```
/// use std::time::Duration;
/// use smart_led_effects::strip::Strobe;
///
/// let count = 10;
/// let colour = None;
/// let period = Duration::from_secs(1);
/// let decay = None;
///
/// let mut effect = Strobe::new(count, colour, period, decay);
/// ```
#[derive(Debug)]
pub struct Strobe<const N: usize, R: RngCore> {
    colour: Option<Hsv>,
    current_colour: Hsv,
    period_ticks: u32,
    fade_val: f32,
    elapsed_ticks: u32,
    rng: R,
}

impl<const N: usize, R: RngCore> Strobe<N, R> {
    pub fn new(rng: R, colour: Option<Srgb<u8>>, period_ticks: u32, decay: Option<f32>) -> Self {
        let colour = colour.map(|c| Hsv::from_color(c.into_format::<f32>()));
        let current_colour = match colour {
            Some(colour) => colour,
            None => Hsv::new(0.0, 0.0, 1.0),
        };
        Self {
            colour,
            current_colour,
            period_ticks,
            fade_val: decay.unwrap_or(0.02),
            elapsed_ticks: 0,
            rng,
        }
    }

    fn genereate_colour(&mut self) {
        // derive two floats in [0,1)
        let h = (self.rng.next_u32() as f32) / (u32::MAX as f32 + 1.0);
        let s = (self.rng.next_u32() as f32) / (u32::MAX as f32 + 1.0);
        self.current_colour = Hsv::new(h * 360.0, s, 1.0);
    }

    fn fade(&mut self) -> bool {
        self.current_colour.value -= self.fade_val;
        if self.current_colour.value <= 0.0 {
            self.current_colour.value = 0.0;
            true
        } else {
            false
        }
    }

    fn reset(&mut self) {
        match self.colour {
            Some(colour) => self.current_colour = colour,
            None => self.genereate_colour(),
        }
        self.current_colour.value = 1.0;
        self.elapsed_ticks = 0;
    }
}

impl<const N: usize, R: RngCore> EffectIterator for Strobe<N, R> {
    fn name(&self) -> &'static str {
        "Strobe"
    }

    fn next_line(&mut self, buf: &mut [crate::RGB8], dt_ticks: u32) -> Option<usize> {
        if self.fade() {
            self.elapsed_ticks = self.elapsed_ticks.saturating_add(dt_ticks);
            if self.elapsed_ticks >= self.period_ticks {
                self.reset();
            }
        }
        let len = core::cmp::min(N, buf.len());
        let px: Srgb<u8> = Srgb::from_color(self.current_colour).into_format();
        let out = crate::RGB8 {
            r: px.red,
            g: px.green,
            b: px.blue,
        };
        for slot in buf.iter_mut().take(len) { *slot = out; }
        Some(len)
    }

    fn pixel_count(&self) -> usize {
        N
    }
}
