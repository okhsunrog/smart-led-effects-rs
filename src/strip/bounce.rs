use crate::{strip::EffectIterator, RGB8};
use core::ops::Range;
use palette::{Darken, FromColor, Hsv, Srgb};
use rand_core::RngCore;

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
}

#[derive(Debug, Clone)]
struct Ball {
    position: f32,
    speed: f32, /* pixels per a second */
    colour: Srgb,
    direction: Direction,
    random_colour: bool,
    gravity: f32,
    bounciness: Range<f32>,
    speed_range: Range<f32>,
    current_bounciness: f32,
}

impl Ball {
    const DEFAULT_GRAVITY: f32 = 30.0; // pixels per a second ^ 2
    const DEFAULT_BOUNCINESS: Range<f32> = 0.2..0.8;
    const DEFAULT_SPEEDS: Range<f32> = 20.0..80.0;
    fn new_params(
        colour: Option<Srgb>,
        gravity: Option<f32>,
        bounciness: Option<Range<f32>>,
        speed: Option<Range<f32>>,
    ) -> Self {
        let is_random = colour.is_none();
        let colour = colour.unwrap_or(Srgb::from_color(Hsv::new(0.0, 1.0, 1.0)));
        Self {
            position: 0.0,
            speed: 0.0,
            colour,
            direction: Direction::Up,
            random_colour: is_random,
            gravity: gravity.unwrap_or(Self::DEFAULT_GRAVITY),
            bounciness: bounciness.unwrap_or(Self::DEFAULT_BOUNCINESS),
            speed_range: speed.unwrap_or(Self::DEFAULT_SPEEDS),
            current_bounciness: 0.0,
        }
    }

    fn reset<R: RngCore>(&mut self, rng: &mut R) {
        let sp_lo = self.speed_range.start;
        let sp_hi = self.speed_range.end;
        let r = (rng.next_u32() as f32) / (u32::MAX as f32 + 1.0);
        self.speed = sp_lo + r * (sp_hi - sp_lo);
        if self.random_colour {
            let hue = (rng.next_u32() as f32) / (u32::MAX as f32 + 1.0) * 360.0;
            self.colour = Srgb::from_color(Hsv::new(hue, 1.0, 1.0));
        }
        let b_lo = self.bounciness.start;
        let b_hi = self.bounciness.end;
        let r2 = (rng.next_u32() as f32) / (u32::MAX as f32 + 1.0);
        self.current_bounciness = b_lo + r2 * (b_hi - b_lo);
    }

    fn update<R: RngCore>(&mut self, dt_sec: f32, rng: &mut R) {
        let elapsed = dt_sec;
        match self.direction {
            Direction::Up => {
                let d1 = self.speed * elapsed / 2.0;
                self.speed -= self.gravity * elapsed;
                let d2 = self.speed * elapsed / 2.0;
                self.position += (d1 + d2).max(0.0);
                if self.speed <= 1.0 {
                    if self.position < 0.5 {
                        self.reset(rng);
                    } else {
                        self.direction = Direction::Down;
                    }
                }
            }
            Direction::Down => {
                let d1 = self.speed * elapsed / 2.0;
                self.speed += self.gravity * elapsed;
                let d2 = self.speed * elapsed / 2.0;
                self.position -= (d1 + d2).max(0.0);
                if self.position <= 0.0 {
                    self.direction = Direction::Up;
                    self.speed *= self.current_bounciness;
                }
            }
        }
    }
    fn location(&self) -> usize {
        self.position as usize
    }
}

/// # Bounce Effect
/// The bounce effect will generate a number of balls that bounce up and down the strip.
/// The balls positions are based on time, use a higher refresh rate for smoother movement.
/// The speeds, bounciness and colour can be tweaked using the arguments when instantiating the effect.
/// The bounce effect will generate a number of balls that bounce up and down the strip.
/// The balls positions are based on time, use a higher refresh rate for smoother movement.
/// The speeds, bounciness and colour can be tweaked using the arguments when instantiating the effect.
///
/// When created with default parameters, the effect will generate 3 balls with random colours, speeds and bounciness.
/// When a ball stops bouncing, it will be reset with new random parameters.
///
/// # Example
///
/// Basic usage:
///
/// ```rust
/// let mut effect = strip::Bounce::new(55, None, None, None, None, None);
/// let pixels = effect.next().unwrap();
/// ```
///
/// # Arguments
///
/// - `count` - The number of LEDs in the strip.
/// - `colour` - The colour of the balls. If None, the colour will be randomised for each ball.
/// - `balls` - The number of balls to generate. If None, 3 balls will be generated.
/// - `gravity` - The gravity of the balls. If None, the default value will be used.
/// - `bounciness` - The bounciness of the balls. If None, the default range will be used.
/// - `speed` - The speed range of the balls. If None, the default range will be used.
pub struct Bounce<const N: usize, const M: usize, R: RngCore> {
    balls: [Ball; M],
    rng: R,
}

impl<const N: usize, const M: usize, R: RngCore> Bounce<N, M, R> {
    pub fn new(
        mut rng: R,
        colour: Option<Srgb>,
        gravity: Option<f32>,
        bounciness: Option<Range<f32>>,
        speed: Option<Range<f32>>,
    ) -> Self {
        let mut balls: [Ball; M] = core::array::from_fn(|_| {
            Ball::new_params(colour, gravity, bounciness.clone(), speed.clone())
        });
        for b in balls.iter_mut() {
            b.reset(&mut rng);
        }
        Self { balls, rng }
    }
}

impl<const N: usize, const M: usize, R: RngCore> EffectIterator for Bounce<N, M, R> {
    fn name(&self) -> &'static str {
        "Bounce"
    }
    fn next_line(&mut self, buf: &mut [RGB8], dt_ticks: u32) -> Option<usize> {
        let len = core::cmp::min(N, buf.len());
        for slot in buf.iter_mut().take(len) { *slot = RGB8 { r:0, g:0, b:0 }; }
        let dt_sec = dt_ticks as f32 / 1000.0;
        for ball in self.balls.iter_mut() {
            ball.update(dt_sec, &mut self.rng);
            let pixel = ball.location();
            let mut tail_len = ((ball.speed * 0.5) as usize).saturating_add(1);
            if tail_len > pixel {
                tail_len = pixel;
            }
            for i in 0..(tail_len as i32) {
                let pos: i32 = match ball.direction {
                    Direction::Up => ball.location() as i32 - i,
                    Direction::Down => ball.location() as i32 + i,
                };
                if pos >= 0 && (pos as usize) < len {
                    let mut colour: Srgb = ball.colour.into_format();
                    colour = colour.darken_fixed(i as f32 / tail_len as f32);
                    let p = colour.into_format::<u8>();
                    buf[pos as usize] = RGB8 {
                        r: p.red,
                        g: p.green,
                        b: p.blue,
                    };
                }
            }
            if pixel < len {
                let p = ball.colour.into_format::<u8>();
                buf[pixel] = RGB8 {
                    r: p.red,
                    g: p.green,
                    b: p.blue,
                };
            }
        }
        Some(len)
    }

    fn pixel_count(&self) -> usize {
        N
    }
}
