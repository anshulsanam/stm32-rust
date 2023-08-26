use crate::startup::entry::reset_handler;
use stm32f303xc_pac::Interrupt;
use crate::utils::critical_section::InterruptNumber;
use crate::systick::systick_handler;

unsafe impl InterruptNumber for Interrupt {
    #[inline(always)]
    fn number(self) -> u16 {
        self as u16
    }
}

pub union Vector {
    handler: unsafe extern "C" fn(),
    reserved: usize,
}

#[link_section = "_reset"]
#[no_mangle]
pub static __RESET_VECTOR: unsafe extern "C" fn() = reset_handler;

#[no_mangle]
pub static __SYSTICK_TIMER: unsafe extern "C" fn() = systick_handler;


#[link_section = ".vectors.exceptions"]
#[no_mangle]
pub static __EXCEPTIONS: [Vector; 15] = [
    // Exception 1: Reset handler the entry point for program
    Vector { 
        handler: __RESET_VECTOR,
    },
    // Exception 2: Non Maskable Interrupt.
    Vector {
        reserved: 0,
    },
    // Exception 3: Hard Fault Interrupt.
    Vector {
        reserved: 0,
    },
    // Exception 4: Memory Management Interrupt [not on Cortex-M0 variants].
    Vector {
        reserved: 0,
    },
    // Exception 5: Bus Fault Interrupt [not on Cortex-M0 variants].
    Vector { reserved: 0 },
    // Exception 6: Usage Fault Interrupt [not on Cortex-M0 variants].
    Vector {
        reserved: 0,
    },
    // 7-10: Reserved
    Vector { reserved: 0 },
    Vector { reserved: 0 },
    Vector { reserved: 0 },
    Vector { reserved: 0 },
    // Exception 11: SV Call Interrupt.
    Vector { reserved: 0 },
    // Exception 12: Debug Monitor Interrupt [not on Cortex-M0 variants].
    Vector {
        reserved: 0,
    },
    // 13: Reserved
    Vector { reserved: 0 },
    // Exception 14: Pend SV Interrupt [not on Cortex-M0 variants].
    Vector { reserved: 0 },
    // Exception 15: System Tick Interrupt.
    Vector { handler: __SYSTICK_TIMER },
];

// If we are not targeting a specific device we bind all the potential device specific interrupts
// to the default handler
#[link_section = ".vectors.interrupts"]
#[no_mangle]
pub static __INTERRUPTS: [unsafe extern "C" fn(); 74] = [{
    extern "C" {
        fn default_handler();
    }

    default_handler
}; 74];

#[no_mangle]
pub unsafe extern "C" fn default_handler() {
    #[allow(clippy::empty_loop)]
    loop {}
}