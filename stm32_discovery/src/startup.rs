pub mod interrupts;
pub mod entry;
use stm32f303xc_pac::Peripherals;

pub fn system_init(per: &mut Peripherals) {
    // Enable FPU set CP10 and CP11 Full Access
    per.FPU_CPACR.cpacr.write(|w| unsafe { w.cp().bits(0b11) });
	
    // Reset the RCC clock configuration to the default reset state
    per.RCC.cr.reset();
    per.RCC.cr.modify(|_, w| w.hsion().on());

    // Reset CFGR register
    per.RCC.cfgr.reset();

	// Reset PREDIV1[3:0] bits 
    per.RCC.cfgr2.modify(|_, w| w.prediv().div1());

	// Reset USARTSW[1:0], I2CSW and TIMs bits 
    per.RCC.cfgr3.modify(|r, w| unsafe { w.bits(r.bits() & 0xFF00FCCC) });

	// Disable all interrupts 
    per.RCC.cir.reset();

    if cfg!(feature = "hsi") {
        set_sys_clock_hsi(per);
    }
    else {
        // Configure the System clock source, PLL Multiplier and Divider factors, 
        // AHB/APBx prescalers and Flash settings
        // Sets up with HSE (high speed external) or HSE_BYPASS (from st-link) clock
        set_sys_clock_hse(per);
    }

}

pub fn set_sys_clock_hsi(per: &mut Peripherals) {
    // Enable Prefetch Buffer and set Flash Latency
    per.FLASH.acr.write(|w| w.prftbe().enabled());
    per.FLASH.acr.write(|w| w.latency().ws1());

    per.RCC.cfgr.reset();
    // HCLK = SYSCLK
    per.RCC.cfgr.write(|w| w.hpre().div1());
    // PCLK = HCLK
    per.RCC.cfgr.write(|w| w.ppre1().div2());
    per.RCC.cfgr.write(|w| w.ppre2().div1());
    // PLL configuration
    per.RCC.cfgr.write(|w| w.pllsrc().hsi_div2());
    per.RCC.cfgr.write(|w| w.pllxtpre().div1());
    per.RCC.cfgr.write(|w| w.pllmul().mul16());

    // Enable PLL
    per.RCC.cr.write(|w| w.pllon().set_bit());

    // Wait for PLL to become ready
    while per.RCC.cr.read().pllrdy().is_not_ready() {}
    // Select PLL as system clock source
    per.RCC.cfgr.write(|w| w.sw().pll());
    // Wait for PLL to be selected as System Clock Source
    while !per.RCC.cfgr.read().sws().is_pll() {}
}

pub fn set_sys_clock_hse(per: &mut Peripherals) {
    if cfg!(feature = "hse_bypass") {
        // HSE on and HSE oscillator bypassed with external clock
        per.RCC.cr.modify(|_, w| w.hseon().on());
        per.RCC.cr.modify(|_, w| w.hsebyp().bypassed());
    }
    // Wait till HSE is ready
    while per.RCC.cr.read().hserdy().is_not_ready() {}

    // Enable Prefetch Buffer and set Flash Latency
    per.FLASH.acr.modify(|_, w| w.prftbe().enabled());
    per.FLASH.acr.modify(|_, w| w.latency().ws2());

    // HCLK = SYSCLK
    per.RCC.cfgr.modify(|_, w| w.hpre().div1());
    // PCLK2 = HCLK
    per.RCC.cfgr.modify(|_, w| w.ppre2().div1());
    // PCLK1 = HCLK
    per.RCC.cfgr.modify(|_, w| w.ppre1().div2());

    // PLL configuration: PLLCLK = HSE * 9 = 72 MHz
    per.RCC.cfgr.modify(|_, w| w.pllsrc().hse_div_prediv());
    per.RCC.cfgr.modify(|_, w| w.pllxtpre().div1());
    per.RCC.cfgr.modify(|_, w| w.pllmul().mul9());
    // Enable PLL
    per.RCC.cr.modify(|_, w| w.pllon().set_bit());

    // Wait for PLL to become ready
    while per.RCC.cr.read().pllrdy().is_not_ready() {}

    // Select PLL as system clock source
    per.RCC.cfgr.modify(|_, w| w.sw().pll());
    // Wait for PLL to be selected as System Clock Source
    while !per.RCC.cfgr.read().sws().is_pll() {}

}



