use crate::{strip::EffectIterator, RGB8};
use palette::{FromColor, Hsv, Srgb};
use rand_core::RngCore;

pub struct Wipe<'a, const N: usize, R: RngCore> {
    position: usize,
    data: &'a [RGB8],
    reverse: bool,
    end: usize,
    randomize: bool,
    rng: R,
    colour_mode: bool,
    fill_colour: Option<RGB8>,
}

impl<'a, const N: usize, R: RngCore> Wipe<'a, N, R> {
    pub fn new(rng: R, data: &'a [RGB8], reverse: bool) -> Self {
        let end = N + data.len();
        Self {
            position: if reverse { end } else { 0 },
            data,
            reverse,
            end,
            randomize: false,
            rng,
            colour_mode: false,
            fill_colour: None,
        }
    }

    pub fn colour_wipe(mut rng: R, colour: Option<RGB8>, reverse: bool) -> Self {
        let mut me = Self::new(rng, &[], reverse);
        match colour {
            Some(c) => me.fill_wipe(c),
            None => me.randomize_colour_wipe(),
        }
        me
    }

    fn fill_wipe(&mut self, colour: RGB8) {
        // use an internal single-colour mode: write zeros + colour + zeros based on indices
        self.colour_mode = true;
        self.fill_colour = Some(colour);
    }

    fn randomize_colour_wipe(&mut self) {
        // generate random colour
        let h = (self.rng.next_u32() as f32) / (u32::MAX as f32 + 1.0) * 360.0;
        let srgb8: Srgb<u8> = Srgb::from_color(Hsv::new(h, 1.0, 1.0)).into_format();
        self.fill_wipe(RGB8 {
            r: srgb8.red,
            g: srgb8.green,
            b: srgb8.blue,
        });
        self.randomize = true;
    }
}

impl<'a, const N: usize, R: RngCore> EffectIterator for Wipe<'a, N, R> {
    fn name(&self) -> &'static str {
        "Wipe"
    }

    fn next_line(&mut self, buf: &mut [RGB8], _dt: u32) -> Option<usize> {
        let used_len = if self.colour_mode { N } else { self.data.len() };
        self.end = N + used_len;
        let pos = self.position;
        let len = core::cmp::min(N, buf.len());
        for i in 0..len {
            let j = pos + i;
            buf[i] = if j < N {
                RGB8 { r: 0, g: 0, b: 0 }
            } else if j < N + used_len {
                if self.colour_mode {
                    self.fill_colour.unwrap_or(RGB8 { r: 0, g: 0, b: 0 })
                } else {
                    self.data[j - N]
                }
            } else {
                RGB8 { r: 0, g: 0, b: 0 }
            };
        }
        if self.reverse {
            if self.position == 0 {
                self.position = self.end;
                if self.randomize {
                    self.randomize_colour_wipe();
                }
            } else {
                self.position -= 1;
            }
        } else {
            self.position += 1;
            if self.position >= self.end {
                self.position = 0;
                if self.randomize {
                    self.randomize_colour_wipe();
                }
            }
        }
        Some(len)
    }

    fn pixel_count(&self) -> usize {
        N
    }
}
