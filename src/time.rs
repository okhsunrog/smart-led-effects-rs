#![cfg(feature = "time-embassy")]
//! Optional helpers integrating with embassy-time to compute dt ticks.

#[cfg(feature = "time-embassy")]
pub mod embassy {
    use embassy_time::{Instant, Duration};

    pub struct DtMs {
        last: Instant,
    }

    impl DtMs {
        pub fn new() -> Self { Self { last: Instant::now() } }
        /// Returns elapsed milliseconds since last call and updates internal instant.
        pub fn tick(&mut self) -> u32 {
            let now = Instant::now();
            let dt = now - self.last;
            self.last = now;
            dt.as_millis() as u32
        }
        pub async fn sleep_ms(ms: u64) { embassy_time::Timer::after(Duration::from_millis(ms)).await; }
    }
}

