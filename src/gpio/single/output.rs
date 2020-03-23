use crate::gpio::*;
use crate::pin::Pin;
use crate::peripheral_to_alias;
use super::set_pin_function_to_gpio;

//
// Output
//

pub trait GpioOut {

    fn set(&mut self, value: bool);
    fn toggle(&mut self);
}

pub struct PushPullGpioOut {
    output: &'static mut u16,
    _pin: Pin
}

impl GpioSingle for PushPullGpioOut {
    fn new(pin: Pin) -> Self {
        let pin_offset = pin.get_pin_offset_in_port();
        let addr = get_port_address(&pin);
        let port = unsafe {
            &mut *(addr as *mut GpioPort)
        };

        set_pin_function_to_gpio(port, pin_offset);
        let output_addr = peripheral_to_alias(((&mut port.output) as *mut u16) as u32, pin_offset);
        let gpio_out = unsafe {
            PushPullGpioOut {
                output: &mut *(output_addr as *mut u16),
                _pin: pin
            }
        };

        let direction_addr =
            peripheral_to_alias(((&mut port.direction) as *mut u16) as u32, pin_offset);

        let direction = unsafe {
            &mut *(direction_addr as *mut u16)
        };

        *direction = 1;
        *gpio_out.output = 0;
        gpio_out
    }

    fn get_current_state(&self) -> bool {
        *self.output != 0
    }
}

impl GpioOut for PushPullGpioOut {
    fn set(&mut self, value: bool) {
        *self.output = value as u16
    }

    fn toggle(&mut self) {
        *self.output ^= 1
    }
}
