#![no_std]
#![no_main]

use cortex_m_rt::entry;
use panic_halt as _;

use stm32l4xx_hal as hal;
use hal::{pac, prelude::*, delay::Delay};

#[entry]
fn main() -> ! {

    // Core & device peripherals
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = pac::Peripherals::take().unwrap();

    // Flash, RCC, and Power (PWR needs APB1R1 bus token on this HAL version)
    let mut flash = dp.FLASH.constrain();
    let mut rcc   = dp.RCC.constrain();
    let mut pwr   = dp.PWR.constrain(&mut rcc.apb1r1);
    
    // Safe default clocks (HSI16). Keep it simple for now.
    let clocks = rcc.cfgr.freeze(&mut flash.acr, &mut pwr);

    // GPIOA → LD2 = PA5 on Nucleo-L476RG
    let mut gpioa = dp.GPIOA.split(&mut rcc.ahb2);
    let mut led = gpioa.pa5.into_push_pull_output(&mut gpioa.moder, &mut gpioa.otyper);

    // SysTick-based delay tied to the frozen clocks
    let mut delay = Delay::new(cp.SYST, clocks);

    loop {
        let _ = led.set_high();          // LED on
        delay.delay_ms(500_u32);

        let _ = led.set_low();           // LED off
        delay.delay_ms(500_u32);
    }
}
