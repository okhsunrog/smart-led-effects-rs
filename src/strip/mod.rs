mod breathe;
pub use breathe::*;
mod rainbow;
pub use rainbow::*;
mod snow_sparkle;
pub use snow_sparkle::*;
mod bounce;
pub use bounce::Bounce;
mod christmas;
pub use christmas::Christmas;
mod collision;
pub use collision::Collision;
mod cycle;
pub use cycle::Cycle;
mod cylon;
pub use cylon::Cylon;
mod fire;
pub use fire::Fire;
mod meteor;
pub use meteor::Meteor;
mod morse;
pub use morse::Morse;
mod progress;
pub use progress::ProgressBar;
mod running_lights;
pub use running_lights::RunningLights;
mod strobe;
pub use strobe::Strobe;
mod timer;
pub use timer::Timer;
mod twinkle;
pub use twinkle::Twinkle;
mod wipe;
pub use wipe::Wipe;

mod effects_trait;
pub use effects_trait::EffectIterator;

// Static list available in no_alloc mode
pub const LIST: &[&str] = &[
    "Breathe",
    "BreatheRandom",
    "Rainbow",
    "SnowSparkle",
    "Bounce",
    "Christmas",
    "Collision",
    "Cycle",
    "Cylon",
    "Fire",
    "Meteor",
    "Morse",
    "ProgressBar",
    "RunningLights",
    "Strobe",
    "Timer",
    "Twinkle",
    "Wipe",
];

#[cfg(any(feature = "std", feature = "alloc"))]
pub fn list() -> alloc::vec::Vec<alloc::string::String> {
    LIST.iter()
        .map(|s| alloc::string::String::from(*s))
        .collect()
}
