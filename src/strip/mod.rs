// Core no_std effects (ported)
mod breathe;
pub use breathe::*;
mod rainbow;
pub use rainbow::*;
mod snow_sparkle;
pub use snow_sparkle::*;

// The remaining effects currently rely on std/alloc and are gated
// until they are refactored to no_std.
#[cfg(feature = "std")]
mod bounce;
#[cfg(feature = "std")]
pub use bounce::Bounce;
#[cfg(feature = "std")]
mod christmas;
#[cfg(feature = "std")]
pub use christmas::Christmas;
#[cfg(feature = "std")]
mod collision;
#[cfg(feature = "std")]
pub use collision::Collision;
#[cfg(feature = "std")]
mod cycle;
#[cfg(feature = "std")]
pub use cycle::Cycle;
#[cfg(feature = "std")]
mod cylon;
#[cfg(feature = "std")]
pub use cylon::Cylon;
#[cfg(feature = "std")]
mod fire;
#[cfg(feature = "std")]
pub use fire::Fire;
#[cfg(feature = "std")]
mod meteor;
#[cfg(feature = "std")]
pub use meteor::Meteor;
#[cfg(feature = "std")]
mod morse;
#[cfg(feature = "std")]
pub use morse::Morse;
#[cfg(feature = "std")]
mod progress;
#[cfg(feature = "std")]
pub use progress::ProgressBar;
#[cfg(feature = "std")]
mod running_lights;
#[cfg(feature = "std")]
pub use running_lights::RunningLights;
#[cfg(feature = "std")]
mod strobe;
#[cfg(feature = "std")]
pub use strobe::Strobe;
#[cfg(feature = "std")]
mod timer;
#[cfg(feature = "std")]
pub use timer::Timer;
#[cfg(feature = "std")]
mod twinkle;
#[cfg(feature = "std")]
pub use twinkle::Twinkle;
#[cfg(feature = "std")]
mod wipe;
#[cfg(feature = "std")]
pub use wipe::Wipe;

mod effects_trait;
pub use effects_trait::EffectIterator;

// Static list available in no_alloc mode
pub const LIST: &[&str] = &[
    "Breathe",
    "Rainbow",
    "SnowSparkle",
    // std-gated effects omitted in no_std builds
];

#[cfg(any(feature = "std", feature = "alloc"))]
pub fn list() -> alloc::vec::Vec<alloc::string::String> {
    LIST.iter().map(|s| alloc::string::String::from(*s)).collect()
}

// Registry and boxed factories only available with alloc/std and depend on std-ported effects
#[cfg(feature = "std")]
pub fn get_default_effect(_count: usize, _name: &str) -> Option<alloc::boxed::Box<dyn EffectIterator>> {
    None
}

#[cfg(feature = "std")]
pub fn get_all_default_effects(_count: usize) -> alloc::vec::Vec<alloc::boxed::Box<dyn EffectIterator>> {
    alloc::vec::Vec::new()
}
