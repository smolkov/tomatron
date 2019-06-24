//! Initialization code
#![no_main]
#![no_std]

pub use nrf51_hal as hal;

pub use nb::*;

pub use crate::hal::nrf51::*;
use crate::hal::gpio::{
    Output,
    PushPull,
    gpio::Parts,
    Input,
    Floating,
};
use crate::hal::gpio::gpio::PIN;

// use crate::hal::gpio::{
//     Input,
//     Floating,
//     Output,
//     PushPull,
//     gpio::Parts,
// };

// crate::hal::gpio::gpio::PIN;

type RELAY = PIN<Output<PushPull>>;
type OUTPUT = PIN<Output<PushPull>>;
type INPUT = PIN<Input<Floating>>;
type ANALOG = PIN<Input<Floating>>;

pub struct AutomationBit {
    pub relay:RELAY,
    pub out01:OUTPUT,
    pub out02:OUTPUT,
    pub in01: INPUT,
    pub in02: INPUT,
    pub a01:  ANALOG,
    pub a02:  ANALOG,
    pub a03:  ANALOG,
}
impl AutomationBit {
    pub fn new(gpio: Parts) ->AutomationBit {
        let relay   = gpio.pin16.into_push_pull_output().downgrade();
        let in01    = gpio.pin8.into_floating_input().downgrade();
        let in02    = gpio.pin13.into_floating_input().downgrade();
        let out01   = gpio.pin14.into_push_pull_output().downgrade();
        let out02   = gpio.pin15.into_push_pull_output().downgrade();
        let a01     = gpio.pin2.into_floating_input().downgrade();
        let a02     = gpio.pin1.into_floating_input().downgrade();
        let a03     = gpio.pin0.into_floating_input().downgrade();
        AutomationBit {
            relay,out01,out02,
            in01,in02,a01,a02,a03
        }

    }
}
