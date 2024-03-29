
#![deny(unsafe_code)]
#![deny(warnings)]
#![no_main]
#![no_std]

// panic-handler crate
extern crate panic_semihosting;

use rtfm::app;

#[app(device = nrf51)]
const APP: () = {
    #[init]
    fn init() {}
};