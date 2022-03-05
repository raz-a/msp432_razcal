#![no_std]
#![no_main]

use core::panic::PanicInfo;
use msp432_razcal::{
    gpio::{GpioPin, GpioPinOutput},
    pin::McuPinSet,
};

#[link_section = ".vector_table.reset"]
#[no_mangle]
pub fn main() -> ! {
    if let Some(pins) = McuPinSet::get_mcu_pins() {
        let mut gpio_pin = GpioPin::new(pins.pa0).to_output_pushpull();
        loop {
            gpio_pin.toggle();
        }
    }

    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo<'_>) -> ! {
    loop {}
}
