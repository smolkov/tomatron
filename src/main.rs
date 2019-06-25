#![no_main]
#![no_std]

use panic_halt;

use microbit::hal::delay::Delay;
use microbit::hal::prelude::*;

use cortex_m_rt::entry;
// use automation_bit::AutomationBit;

#[entry]
fn main() -> ! {
    if let Some(p) = microbit::Peripherals::take() {
        let gpio = p.GPIO.split();
        let mut  relay  = gpio.pin16.into_push_pull_output().downgrade();
        let in01        = gpio.pin8.into_floating_input().downgrade();
        let in02        = gpio.pin13.into_floating_input().downgrade();
        let mut out01   = gpio.pin14.into_push_pull_output().downgrade();
        let mut out02   = gpio.pin15.into_push_pull_output().downgrade();
        let a01         = gpio.pin2.into_floating_input().downgrade();
        let a02         = gpio.pin1.into_floating_input().downgrade();
        let a03         = gpio.pin0.into_floating_input().downgrade();
        let mut delay = Delay::new(p.TIMER0);
        let mut led = gpio.pin10.into_push_pull_output();
        let _ = gpio.pin4.into_push_pull_output();
        out01.set_high();
        out02.set_high();
        loop {
            relay.set_high();
            led.set_low();
            delay.delay_ms(1_000_u16);
            led.set_high();
            delay.delay_ms(1_000_u16);
            relay.set_low();
            delay.delay_ms(2_000_u16);
        }
    }

    loop {
        continue;
    }
}