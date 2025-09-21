
#![no_std]
#![no_main]

use cortex_m_rt::entry;
use panic_halt as _;
use stm32l4xx_hal::{
    prelude::*,
    pac,
    pwm::{self, PwmExt1},
    time::Hertz,
};

#[entry]
fn main() -> ! {
    // Get access to the device specific peripherals from the peripheral access crate
    let dp = pac::Peripherals::take().unwrap();

    // Constrain and freeze the clock
    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();
    let mut pwr = dp.PWR.constrain(&mut rcc.apb1r1);
    let clocks = rcc.cfgr.freeze(&mut flash.acr, &mut pwr);

    // Acquire GPIOA
    let mut gpioa = dp.GPIOA.split(&mut rcc.ahb2);

    // Configure PA0-PA3 as alternate function (TIM2_CH1-CH4)
    let pa0 = gpioa.pa0.into_alternate::<1>(&mut gpioa.moder, &mut gpioa.otyper, &mut gpioa.afrl);
    let pa1 = gpioa.pa1.into_alternate::<1>(&mut gpioa.moder, &mut gpioa.otyper, &mut gpioa.afrl);
    let pa2 = gpioa.pa2.into_alternate::<1>(&mut gpioa.moder, &mut gpioa.otyper, &mut gpioa.afrl);
    let pa3 = gpioa.pa3.into_alternate::<1>(&mut gpioa.moder, &mut gpioa.otyper, &mut gpioa.afrl);

    // Set up PWM on TIM2 (all channels, but use only PA0)
    let (mut ch1, _, _, _) = pwm::tim2(
        dp.TIM2,
        (pa0, pa1, pa2, pa3),
        Hertz(50),
        clocks,
        &mut rcc.apb1r1,
    );

    let max_duty = ch1.get_max_duty();
    let min_pulse = max_duty / 20; // 1ms
    let max_pulse = max_duty / 10; // 2ms
    let mid_pulse = (min_pulse + max_pulse) / 2;
    ch1.set_duty(mid_pulse);
    ch1.enable();

    loop {
        // You can update ch1.set_duty(...) here to move the servo
    }
}
