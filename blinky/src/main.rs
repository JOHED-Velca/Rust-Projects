#![no_std]
#![no_main]

use cortex_m_rt::entry;
use panic_halt as _;

use stm32l4xx_hal as hal;
use hal::{pac, prelude::*};

#[entry]
fn main() -> ! {
    let _cp = cortex_m::Peripherals::take().unwrap();
    let dp = pac::Peripherals::take().unwrap();

    let mut flash = dp.FLASH.constrain();
    let mut rcc   = dp.RCC.constrain();
    let mut pwr   = dp.PWR.constrain(&mut rcc.apb1r1);
    let _clocks   = rcc.cfgr.freeze(&mut flash.acr, &mut pwr);

    // GPIOA → LD2 = PA5
    let mut gpioa = dp.GPIOA.split(&mut rcc.ahb2);
    let mut led = gpioa.pa5.into_push_pull_output(&mut gpioa.moder, &mut gpioa.otyper);

    loop {
        let _ = led.set_high();                          // LED on
        for _ in 0..2_000_000 { cortex_m::asm::nop(); }  // crude delay

        let _ = led.set_low();                           // LED off
        for _ in 0..2_000_000 { cortex_m::asm::nop(); }  // crude delay
    }
}
