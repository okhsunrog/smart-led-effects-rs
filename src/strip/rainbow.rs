use crate::{strip::EffectIterator, RGB8};
use palette::{Hsv, ShiftHueAssign};

pub struct Rainbow<const N: usize> {
    last_state: [Hsv; N],
    step_size: f32,
}

impl<const N: usize> Rainbow<N> {
    pub fn new(steps: Option<usize>) -> Self {
        let step = steps.unwrap_or(360);
        let step_size = 360.0 / step as f32;
        let separation = 360.0 / N as f32;
        let mut color = Hsv::new(0.0, 1.0, 1.0);
        let mut arr: [Hsv; N] = core::array::from_fn(|i| {
            if i > 0 {
                color.shift_hue_assign(separation);
            }
            color
        });
        // Ensure the first element is Hsv::new(0,1,1)
        arr[0] = Hsv::new(0.0, 1.0, 1.0);
        Self { last_state: arr, step_size }
    }
}

impl<const N: usize> EffectIterator for Rainbow<N> {
    fn name(&self) -> &'static str {
        "Rainbow"
    }

    fn next_line(&mut self, buf: &mut [RGB8], _dt: u32) -> Option<usize> {
        for pixel in self.last_state.iter_mut() {
            pixel.shift_hue_assign(self.step_size);
        }
        let len = core::cmp::min(N, buf.len());
        for i in 0..len {
            let rgb = crate::utils::hsv_to_rgb8_pixel(self.last_state[i]);
            buf[i] = rgb;
        }
        Some(len)
    }

    fn pixel_count(&self) -> usize {
        N
    }
}
