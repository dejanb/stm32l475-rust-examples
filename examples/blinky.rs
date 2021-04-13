//! Prints "Hello, world!" on the host console and blinks LEDs

#![no_main]
#![no_std]

#[allow(unused_extern_crates)]
use panic_halt as _;

use cortex_m_rt::entry;
use stm32l4xx_hal as hal;
use crate::hal::{prelude::*, stm32};
use crate::hal::delay::Delay;

use rtt_target::{rprintln, rtt_init_print};

#[entry]
fn main() -> ! {

    rtt_init_print!();

    if let (Some(dp), Some(cp)) = (
        stm32::Peripherals::take(),
        cortex_m::peripheral::Peripherals::take(),
    ) {
        let mut flash = dp.FLASH.constrain();
        let mut rcc = dp.RCC.constrain();
        let mut pwr = dp.PWR.constrain(&mut rcc.apb1r1);

        // Try a different clock configuration
        let clocks = rcc.cfgr.hclk(8.mhz()).freeze(&mut flash.acr, &mut pwr);

        let mut timer = Delay::new(cp.SYST, clocks);

        let mut gpioa = dp.GPIOA.split(&mut rcc.ahb2);
        let mut ld1 = gpioa
            .pa5
            .into_push_pull_output(&mut gpioa.moder, &mut gpioa.otyper);

        let mut gpiob = dp.GPIOB.split(&mut rcc.ahb2);
        let mut ld2 = gpiob
            .pb14
            .into_push_pull_output(&mut gpiob.moder, &mut gpiob.otyper);

        loop {
            rprintln!("Hello, world!");
            timer.delay_ms(1000_u32);
            ld1.set_high().ok();
            ld2.set_low().ok();
            timer.delay_ms(1000_u32);
            ld1.set_low().ok();
            ld2.set_high().ok();
        }

    }

    loop {
    }
}
