#![no_main]
#![no_std]

use cortex_m_semihosting::{debug, hprintln};
use panic_semihosting as _;

// #[rtic::app(device = stm32wl_hal::pac, peripherals = true)]
#[rtic::app(device = stm32wl::stm32wle5, peripherals = true)]
const APP: () = {
    #[init]
    fn init(_: init::Context) {
        hprintln!("init").unwrap();
    }

    #[idle]
    fn idle(_: idle::Context) -> ! {
        static mut X: u32 = 0;

        // Safe access to local `static mut` variable
        let _x: &'static mut u32 = X;

        hprintln!("idle").unwrap();

        debug::exit(debug::EXIT_SUCCESS);

        loop {}
    }

    #[task(binds = RTC_WKUP)]
    fn wkup(_ctx: wkup::Context) {
        hprintln!("wkup").unwrap();
    }
};
