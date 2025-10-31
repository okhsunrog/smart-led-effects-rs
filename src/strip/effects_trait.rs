use crate::RGB8;

#[cfg(feature = "std")]
extern crate std;

/// Core, no_alloc effect interface: render into a caller-provided buffer.
pub trait EffectIterator {
    fn name(&self) -> &'static str;

    /// Advance the effect by `dt_ticks` (units chosen by the caller) and
    /// render the next frame into `buf`.
    /// Returns number of pixels written, or None if effect is finished.
    fn next_line(&mut self, buf: &mut [RGB8], dt_ticks: u32) -> Option<usize>;

    /// Number of pixels this effect controls
    fn pixel_count(&self) -> usize;

    /// Convenience helper (alloc feature): allocate a buffer and render into it.
    #[cfg(any(feature = "std", feature = "alloc"))]
    fn next(&mut self, dt_ticks: u32) -> Option<alloc::vec::Vec<RGB8>> {
        let count = self.pixel_count();
        let mut buf = alloc::vec![RGB8 { r: 0, g: 0, b: 0 }; count];
        self.next_line(&mut buf, dt_ticks)?;
        Some(buf)
    }
}
