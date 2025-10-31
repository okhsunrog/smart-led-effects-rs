use crate::{strip::EffectIterator, RGB8};

// A minimal Morse effect that slides pre-encoded bits across the strip using Wipe-like logic,
// but without heap. The message must be provided pre-encoded as bits slice (1=on, 0=off).
pub struct Morse<'a, const N: usize> {
    data: &'a [u8], // sequence of 0/1
    position: usize,
    reverse: bool,
    colour: RGB8,
}

impl<'a, const N: usize> Morse<'a, N> {
    pub fn new_bits(data: &'a [u8], colour: Option<RGB8>, reverse: bool) -> Self {
        Self {
            data,
            position: if reverse { N + data.len() } else { 0 },
            reverse,
            colour: colour.unwrap_or(RGB8 { r: 255, g: 0, b: 0 }),
        }
    }
}

impl<'a, const N: usize> EffectIterator for Morse<'a, N> {
    fn name(&self) -> &'static str {
        "Morse"
    }

    fn next_line(&mut self, buf: &mut [RGB8], _dt: u32) -> Option<usize> {
        let end = N + self.data.len();
        let pos = self.position;
        let len = core::cmp::min(N, buf.len());
        for (i, slot) in buf.iter_mut().enumerate().take(len) {
            let j = pos + i;
            *slot = if j < N {
                RGB8 { r: 0, g: 0, b: 0 }
            } else if j < N + self.data.len() {
                if self.data[j - N] == 1 { self.colour } else { RGB8 { r: 0, g: 0, b: 0 } }
            } else {
                RGB8 { r: 0, g: 0, b: 0 }
            };
        }
        if self.reverse {
            if self.position == 0 {
                self.position = end;
            } else {
                self.position -= 1;
            }
        } else {
            self.position += 1;
            if self.position > end {
                self.position = 0;
            }
        }
        Some(len)
    }

    fn pixel_count(&self) -> usize {
        N
    }
}
