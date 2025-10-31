use crate::{strip::EffectIterator, RGB8};
use palette::{Mix, Srgb};

pub struct Timer<const N: usize> {
    total_ticks: u32,
    start_colour: Srgb,
    end_colour: Srgb,
    gradient: bool,
    pixels_per_tick: f32,
    elapsed_ticks: u32,
    running: bool,
}

impl<const N: usize> Timer<N> {
    const DEFAULT_START_COLOUR: Srgb = Srgb::new(0.0, 0.0, 1.0);
    const DEFAULT_END_COLOUR: Srgb = Srgb::new(1.0, 0.0, 0.0);
    pub fn new(
        total_ticks: u32,
        start_colour: Option<Srgb>,
        end_colour: Option<Srgb>,
        gradient: Option<bool>,
        start: bool,
    ) -> Self {
        Self {
            total_ticks,
            start_colour: start_colour.unwrap_or(Self::DEFAULT_START_COLOUR),
            end_colour: end_colour.unwrap_or(Self::DEFAULT_END_COLOUR),
            gradient: gradient.unwrap_or(false),
            pixels_per_tick: N as f32 / total_ticks.max(1) as f32,
            elapsed_ticks: 0,
            running: start,
        }
    }

    pub fn start(&mut self) {
        self.elapsed_ticks = 0;
        self.running = true;
    }
    pub fn stop(&mut self) {
        self.running = false;
    }
    pub fn reset(&mut self) {
        self.elapsed_ticks = 0;
    }
}

impl<const N: usize> EffectIterator for Timer<N> {
    fn name(&self) -> &'static str {
        "Timer"
    }

    fn next_line(&mut self, buf: &mut [RGB8], dt_ticks: u32) -> Option<usize> {
        let len = core::cmp::min(N, buf.len());
        for slot in buf.iter_mut().take(len) { *slot = RGB8 { r:0, g:0, b:0 }; }
        if !self.running {
            return Some(len);
        }
        self.elapsed_ticks = self.elapsed_ticks.saturating_add(dt_ticks);
        if self.elapsed_ticks >= self.total_ticks {
            self.reset();
            return Some(len);
        }
        let elapsed = self.elapsed_ticks as f32;
        let progressed = (self.pixels_per_tick * elapsed) as usize;
        let pixels = N.saturating_sub(progressed);
        if self.gradient {
            for (i, slot) in buf.iter_mut().enumerate().take(core::cmp::min(pixels, len)) {
                let p: Srgb<u8> = self
                    .end_colour
                    .mix(self.start_colour, i as f32 / N as f32)
                    .into_format();
                *slot = RGB8 {
                    r: p.red,
                    g: p.green,
                    b: p.blue,
                };
            }
        } else {
            let mix = elapsed / self.total_ticks as f32;
            let p: Srgb<u8> = self.start_colour.mix(self.end_colour, mix).into_format();
            let px = RGB8 {
                r: p.red,
                g: p.green,
                b: p.blue,
            };
            for slot in buf.iter_mut().take(core::cmp::min(pixels, len)) { *slot = px; }
        }
        Some(len)
    }

    fn pixel_count(&self) -> usize {
        N
    }
}
