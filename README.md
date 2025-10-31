# Smart LED Effects

This supplies a collection of effects for usage with individually addressable LED strips such as the WS2812b.
Each effect renders into a caller-provided buffer of RGB8 pixels (from smart-leds-trait), which you then write with your LED driver.

The EffectIterator trait defines:
    - `name()`
    - `next_line(&mut self, out: &mut [RGB8], dt_ticks: u32) -> Option<usize>`

`name` returns the effect name.
`next_line` advances the effect by `dt_ticks` (time units defined by your app) and fills `out`. All current effects loop indefinitely and return `Some(len)` where `len` is the number of pixels written.

This crate borrows heavily from [fastLED](https://github.com/FastLED/FastLED) and [tweaking4all](https://www.tweaking4all.com/hardware/arduino/adruino-led-strip-effects/). The majority of the effect art is taken straight from here, andd reimplemented in Rust.

## Dimensionality

Currently only works for strips/loops. But someday the plan is to extend it.

## Effects

    - Breathe
    - Bounce
    - Collision
    - Cylon
    - Fire
    - Meteor
    - Morse
    - ProgressBar
    - Rainbow
    - RunningLights
    - SnowSparkle
    - Strobe
    - Timer
    - Twinkle
    - Wipe

## Example Usage

```rust
use smart_led_effects::{strip::{self, EffectIterator}, RGB8};

const COUNT: usize = 55;
let mut effect = strip::Rainbow::<COUNT>::new(None);
let mut buf = [RGB8 { r:0, g:0, b:0 }; COUNT];

// in your frame/timer loop
let dt_ms = 16; // time since last frame in ms
let _ = effect.next_line(&mut buf, dt_ms);
// write `buf` via your SmartLedsWrite driver
```

## References

 - [Palette](https://crates.io/crates/palette)
 - [fastLED](https://github.com/FastLED/FastLED)
 - [tweaking4all](https://www.tweaking4all.com/hardware/arduino/adruino-led-strip-effects/)

