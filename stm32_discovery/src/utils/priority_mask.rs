//! Priority mask register

use core::arch::asm;

/// All exceptions with configurable priority are ...
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PriorityMask {
    /// Active
    Active,
    /// Inactive
    Inactive,
}

impl PriorityMask {
    /// All exceptions with configurable priority are active
    #[inline]
    pub fn is_active(self) -> bool {
        self == PriorityMask::Active
    }

    /// All exceptions with configurable priority are inactive
    #[inline]
    pub fn is_inactive(self) -> bool {
        self == PriorityMask::Inactive
    }
}

/// Reads the CPU register
#[inline]
pub fn read() -> PriorityMask {
    let r: u32;
    unsafe { asm!("mrs {}, PRIMASK", out(reg) r, options(nomem, nostack, preserves_flags)) };
    if r & (1 << 0) == (1 << 0) {
        PriorityMask::Inactive
    } else {
        PriorityMask::Active
    }
}
