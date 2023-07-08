#[cfg(feature = "cortex_m")]
pub use cortex_m::interrupt::free;

#[cfg(feature = "avr")]
pub use avr_device::interrupt::free;

#[cfg(not(any(feature = "cortex_m", feature = "avr")))]
pub fn free<F, R>(f: F) -> R
where
    F: FnOnce(bare_metal::CriticalSection) -> R,
{
    f(unsafe { bare_metal::CriticalSection::new() })
}
