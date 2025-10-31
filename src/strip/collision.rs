use crate::{strip::EffectIterator, RGB8};
use palette::{Darken, FromColor, Hsv, Mix, Srgb};
use rand_core::RngCore;

#[derive(Debug, Clone, Copy)]
pub struct Particle {
    position: i32,
    colour: Srgb,
    reverse: bool,
    speed: usize,
    size: usize,
}

impl Particle {
    pub fn new<R: RngCore>(position: i32, reverse: bool, rng: &mut R) -> Self {
        let h = (rng.next_u32() as f32) / (u32::MAX as f32 + 1.0) * 360.0;
        let size = 1 + (rng.next_u32() as usize % 3);
        Particle {
            position,
            colour: Srgb::from_color(Hsv::new(h, 1.0, 1.0)),
            reverse,
            speed: 1,
            size,
        }
    }

    pub fn collide(&self, other: &Particle) -> Option<(Particle, Particle)> {
        if (self.position - other.position).abs() > 1 || self.position - other.position < -1 {
            return None;
        }
        let mut rhs = *other;
        let mut lhs = *self;
        lhs.reverse = !lhs.reverse;
        rhs.reverse = !rhs.reverse;

        let scaling_factor = 1.0 - lhs.size as f32 / (lhs.size + rhs.size) as f32;
        let mix = lhs.colour.mix(rhs.colour, scaling_factor / 2.0);
        lhs.colour = mix;
        rhs.colour = mix;

        Some((lhs, rhs))
    }
}

pub struct Collision<const N: usize, R: RngCore> {
    particles: [Particle; 2],
    shatter: bool,
    shattered: bool,
    current: [Srgb; N],
    rng: R,
}

impl<const N: usize, R: RngCore> Collision<N, R> {
    pub fn new(mut rng: R, shatter: Option<bool>) -> Self {
        let p1 = Particle::new(0, false, &mut rng);
        let p2 = Particle::new(N as i32, true, &mut rng);
        Self {
            particles: [p1, p2],
            shatter: shatter.unwrap_or(true),
            shattered: false,
            current: [Srgb::new(0.0, 0.0, 0.0); N],
            rng,
        }
    }

    pub fn reset(&mut self) {
        let p1 = Particle::new(0, false, &mut self.rng);
        let p2 = Particle::new(N as i32 - 1, true, &mut self.rng);
        self.particles = [p1, p2];
        self.shattered = false;
    }

    pub fn check_for_collision(&mut self) -> bool {
        if let Some((lhs, rhs)) = self.particles[0].collide(&self.particles[1]) {
            self.particles[0] = lhs;
            self.particles[1] = rhs;

            true
        } else {
            false
        }
    }

    pub fn shatter(&mut self) {
        if !self.shatter {
            return;
        }
        self.shattered = true;

        self.current[N / 2] = Srgb::new(1.0, 1.0, 1.0);

        let mut hsv = Hsv::from_color(self.particles[0].colour);
        hsv.value = 1.0;
        let normalize = 1.0 / N as f32;

        for i in 0..(N / 2) {
            if (self.rng.next_u32() & 1) == 1 {
                let hsv = hsv.darken(1.0 - normalize * i as f32);
                self.current[i] = Srgb::from_color(hsv);
            }
        }
        for i in (N / 2)..(N) {
            if (self.rng.next_u32() & 1) == 1 {
                let hsv = hsv.darken(normalize * i as f32);
                self.current[i] = Srgb::from_color(hsv);
            }
        }
    }

    pub fn move_particles(&mut self, out: &mut [RGB8]) {
        for pixel in out.iter_mut() {
            *pixel = RGB8 { r: 0, g: 0, b: 0 };
        }
        for particle in self.particles.iter_mut() {
            if particle.position >= 0 && particle.position < N as i32 {
                for i in 0..particle.size {
                    if particle.reverse {
                        if particle.position + i as i32 >= 0
                            && i as i32 + particle.position < N as i32
                        {
                            let p = particle.colour.into_format::<u8>();
                            out[(particle.position + i as i32) as usize] = RGB8 {
                                r: p.red,
                                g: p.green,
                                b: p.blue,
                            };
                        }
                    } else if particle.position - i as i32 >= 0
                        && (particle.position - i as i32) < N as i32
                    {
                        let p = particle.colour.into_format::<u8>();
                        out[(particle.position - i as i32) as usize] = RGB8 {
                            r: p.red,
                            g: p.green,
                            b: p.blue,
                        };
                    }
                }
                let p = particle.colour.into_format::<u8>();
                out[particle.position as usize] = RGB8 {
                    r: p.red,
                    g: p.green,
                    b: p.blue,
                };
            }
        }
    }
}

impl<const N: usize, R: RngCore> EffectIterator for Collision<N, R> {
    fn name(&self) -> &'static str {
        "Collision"
    }

    fn next_line(&mut self, buf: &mut [RGB8], _dt: u32) -> Option<usize> {
        if !self.shattered {
            for particle in self.particles.iter_mut() {
                if particle.reverse {
                    particle.position -= particle.speed as i32;
                } else {
                    particle.position += particle.speed as i32;
                }
            }

            if self.check_for_collision() {
                self.shatter();
            }

            if self.particles[0].position < 0 && self.particles[1].position >= N as i32 {
                self.reset();
            }
            let len = core::cmp::min(N, buf.len());
            self.move_particles(&mut buf[..len]);
            Some(len)
        } else {
            for pixel in self.current.iter_mut() {
                if (self.rng.next_u32() & 1) == 1 {
                    *pixel = pixel.darken(0.1);
                }
            }

            const RESET_VAL: f32 = 0.01;

            for pixel in self.current.iter() {
                if pixel.red > RESET_VAL || pixel.green > RESET_VAL || pixel.blue > RESET_VAL {
                    let len = core::cmp::min(N, buf.len());
                    for i in 0..len {
                        let p = self.current[i].into_format::<u8>();
                        buf[i] = RGB8 {
                            r: p.red,
                            g: p.green,
                            b: p.blue,
                        };
                    }
                    return Some(len);
                }
            }
            self.reset();
            let len = core::cmp::min(N, buf.len());
            for i in 0..len {
                buf[i] = RGB8 { r: 0, g: 0, b: 0 };
            }
            Some(len)
        }
    }

    fn pixel_count(&self) -> usize {
        N
    }
}
