use critical_section::{set_impl, Impl, RawRestoreState};
use super::priority_mask;

struct SingleCoreCriticalSection;
set_impl!(SingleCoreCriticalSection);


use core::arch::asm;
use core::sync::atomic::{compiler_fence, Ordering};

// Interrupts

/// Trait for enums of external interrupt numbers.
///
/// This trait should be implemented by a peripheral access crate (PAC)
/// on its enum of available external interrupts for a specific device.
/// Each variant must convert to a u16 of its interrupt number,
/// which is its exception number - 16.
///
/// # Safety
///
/// This trait must only be implemented on enums of device interrupts. Each
/// enum variant must represent a distinct value (no duplicates are permitted),
/// and must always return the same value (do not change at runtime).
///
/// These requirements ensure safe nesting of critical sections.
pub unsafe trait InterruptNumber: Copy {
    /// Return the interrupt number associated with this variant.
    ///
    /// See trait documentation for safety requirements.
    fn number(self) -> u16;
}

/// Disables all interrupts in the current core.
#[inline]
pub fn disable() {
    unsafe {
        asm!("cpsid i", options(nomem, nostack, preserves_flags));
    }

    // Ensure no subsequent memory accesses are reordered to before interrupts are disabled.
    compiler_fence(Ordering::SeqCst);
}

/// Enables all the interrupts in the current core.
///
/// # Safety
///
/// - Do not call this function inside a critical section.
#[inline]
pub unsafe fn enable() {
    // Ensure no preceeding memory accesses are reordered to after interrupts are enabled.
    compiler_fence(Ordering::SeqCst);

    asm!("cpsie i", options(nomem, nostack, preserves_flags));
}

/// Execute closure `f` with interrupts disabled in the current core.
///
/// This method does not synchronise multiple cores and may disable required
/// interrupts on some platforms; see the `critical-section` crate for a cross-platform
/// way to enter a critical section which provides a `CriticalSection` token.
///
/// This crate provides an implementation for `critical-section` suitable for single-core systems,
/// based on disabling all interrupts. It can be enabled with the `critical-section-single-core` feature.
#[inline]
pub fn free<F, R>(f: F) -> R
where
    F: FnOnce() -> R,
{
    let primask = priority_mask::read();

    // disable interrupts
    disable();

    let r = f();

    // If the interrupts were active before our `disable` call, then re-enable
    // them. Otherwise, keep them disabled
    if primask.is_active() {
        unsafe { enable() }
    }

    r
}

unsafe impl Impl for SingleCoreCriticalSection {
    unsafe fn acquire() -> RawRestoreState {
        let was_active = priority_mask::read().is_active();
        disable();
        was_active
    }

    unsafe fn release(was_active: RawRestoreState) {
        // Only re-enable interrupts if they were enabled before the critical section.
        if was_active {
            enable()
        }
    }
}
