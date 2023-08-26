use stm32f303xc_pac::Peripherals;
use core::{sync::atomic::{AtomicU32, Ordering}, arch::asm};

static mut TICKS: AtomicU32 = AtomicU32::new(0);

#[no_mangle]
pub unsafe extern "C" fn systick_handler() {
    TICKS.fetch_add(1, Ordering::Relaxed);
}

pub fn millis() -> u32 {
    unsafe { TICKS.load(Ordering::Relaxed) }
}

pub fn delay(delay_ms: u32) {
    unsafe {
        let until = TICKS.load(Ordering::Relaxed) + delay_ms;
        while TICKS.load(Ordering::Relaxed) < until { 
            asm!("nop")
        }
    }
}

pub fn systick_init(per: &mut Peripherals, sys_freq: u32) {
    // 1ms tick
    per.STK.load.modify(|_, w| unsafe { w.bits((sys_freq / 1000) as u32 - 1) });
    // Explicitly set start value of 0 (undefined on reset)
    per.STK.val.reset();
    // Enable systick interrupt, source processor clock
    per.STK.ctrl.modify(|_, w| w.tickint().set_bit());
    per.STK.ctrl.modify(|_, w| w.clksource().set_bit());
    // TODO: Set SysTick interrupt priority (default: 0, the highest)
    // Enable systick
    per.STK.ctrl.modify(|_, w| w.enable().set_bit());
    // Enable SYSCFG
    per.RCC.apb2enr.modify(|_, w| w.syscfgen().enabled() );
}