#![no_std]
#![no_main]

pub mod startup;
pub mod utils;
pub mod register;

use stm32f303xc_pac::Peripherals;

use core::panic::PanicInfo;
/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    // On a panic, loop forever
    loop {
        continue;
    }
}

fn main() -> ! {
    // Just a test
    let _per = Peripherals::take().unwrap();

    loop {}
}
