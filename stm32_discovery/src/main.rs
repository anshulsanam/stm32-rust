#![no_std]
#![no_main]

pub mod startup;
pub mod utils;
pub mod systick;

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
    // Get our peripherals
    let mut per = Peripherals::take().unwrap();
    // Set up clock and stuff
    startup::system_init(&mut per);
    // Start ticking at 1ms
    systick::systick_init(&mut per, 72000000); // CPU frequency, 64 Mhz

    // Enable clock for GPIO port E, set 9 as output
    per.RCC.ahbenr.modify(|_, w| w.iopeen().enabled());
    per.GPIOE.moder.modify(|_, w| w.moder9().output());

    loop {
        per.GPIOE.bsrr.write(|w| w.br9().set_bit());
        systick::delay(100);
        per.GPIOE.bsrr.write(|w| w.bs9().set_bit());
        systick::delay(100);
    }
}
