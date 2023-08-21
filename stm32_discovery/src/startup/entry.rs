use crate::main;
use core::ptr::{read, write_volatile};
use core::mem::zeroed;


#[no_mangle]
pub unsafe extern "C" fn reset_handler() -> ! {
    extern "C" {
        // These symbols come from `linker.ld`
        static mut _sbss: u32; // Start of .bss section
        static mut _ebss: u32; // End of .bss section
        static mut _sdata: u32; // Start of .data section
        static mut _edata: u32; // End of .data section
        static _sidata: u32; // Start of .rodata section
    }

    // Initialize (Zero) BSS
    unsafe {
        let mut sbss: *mut u32 = &mut _sbss;
        let ebss: *mut u32 = &mut _ebss;

        while sbss < ebss {
            write_volatile(sbss, zeroed());
            sbss = sbss.offset(1);
        }
    }

    // Initialize Data
    unsafe {
        let mut sdata: *mut u32 = &mut _sdata;
        let edata: *mut u32 = &mut _edata;
        let mut sidata: *const u32 = &_sidata;

        while sdata < edata {
            write_volatile(sdata, read(sidata));
            sdata = sdata.offset(1);
            sidata = sidata.offset(1);
        }
    }

    // Call user's main function
    main()
}

