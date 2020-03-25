use crate::gpio::*;
use crate::peripheral_to_alias;
use crate::pin::Pin;

pub struct GpioPin<GpioConfig> {
    config: GpioConfig,
    input: &'static u16,
    output: &'static mut u16,
    direction: &'static mut u16,
    resistor_enable: &'static mut u16,
    pin: Pin,
}

pub fn gpio_pin_new(pin: Pin) -> GpioPin<Disabled> {
    let pin_offset = pin.get_pin_offset_in_port();
    let addr = get_port_address(&pin);
    let port = unsafe { &mut *(addr as *mut GpioPort) };

    set_pin_function_to_gpio(port, pin_offset);
    let input_addr = peripheral_to_alias(((&port.input) as *const u16) as u32, pin_offset);
    let output_addr = peripheral_to_alias(((&mut port.output) as *mut u16) as u32, pin_offset);
    let dir_addr = peripheral_to_alias(((&mut port.direction) as *mut u16) as u32, pin_offset);
    let res_addr =
        peripheral_to_alias(((&mut port.resistor_enable) as *mut u16) as u32, pin_offset);

    let gpio_pin = unsafe {
        GpioPin {
            config: Disabled,
            input: &*(input_addr as *const u16),
            output: &mut *(output_addr as *mut u16),
            direction: &mut *(dir_addr as *mut u16),
            resistor_enable: &mut *(res_addr as *mut u16),
            pin: pin,
        }
    };

    gpio_pin
}

impl<GpioConfig> GpioPin<GpioConfig> {
    pub fn to_input_highz(self) -> GpioPin<GpioIn<HighImpedance>> {
        *self.resistor_enable = 0;
        *self.direction = 0;
        GpioPin {
            config: GpioIn {
                input_mode: HighImpedance,
            },

            input: self.input,
            output: self.output,
            direction: self.direction,
            resistor_enable: self.resistor_enable,
            pin: self.pin,
        }
    }

    pub fn to_input_pullup(self) -> GpioPin<GpioIn<PullUp>> {
        *self.resistor_enable = 1;
        *self.direction = 0;
        *self.output = 1;
        GpioPin {
            config: GpioIn { input_mode: PullUp },
            input: self.input,
            output: self.output,
            direction: self.direction,
            resistor_enable: self.resistor_enable,
            pin: self.pin,
        }
    }

    pub fn to_input_pulldown(self) -> GpioPin<GpioIn<PullDown>> {
        *self.resistor_enable = 1;
        *self.direction = 0;
        *self.output = 0;
        GpioPin {
            config: GpioIn {
                input_mode: PullDown,
            },
            input: self.input,
            output: self.output,
            direction: self.direction,
            resistor_enable: self.resistor_enable,
            pin: self.pin,
        }
    }

    pub fn to_output_pushpull(self) -> GpioPin<GpioOut<PushPull>> {
        *self.output = 0;
        *self.direction = 1;
        GpioPin {
            config: GpioOut {
                output_mode: PushPull,
            },
            input: self.input,
            output: self.output,
            direction: self.direction,
            resistor_enable: self.resistor_enable,
            pin: self.pin,
        }
    }

    pub fn to_output_opencollector(self) -> GpioPin<GpioOut<OpenCollector>> {
        *self.output = 0;
        *self.direction = 1;
        *self.resistor_enable = 1;
        GpioPin {
            config: GpioOut {
                output_mode: OpenCollector,
            },
            input: self.input,
            output: self.output,
            direction: self.direction,
            resistor_enable: self.resistor_enable,
            pin: self.pin,
        }
    }
}

impl<InputMode> GpioPin<GpioIn<InputMode>> {
    pub fn read(&self) -> bool {
        *self.input != 0
    }
}

impl GpioPin<GpioOut<PushPull>> {
    pub fn set(&mut self) {
        *self.output = 1;
    }

    pub fn clear(&mut self) {
        *self.output = 0;
    }

    pub fn toggle(&mut self) {
        *self.output ^= 1;
    }
}

impl GpioPin<GpioOut<OpenCollector>> {
    pub fn set(&mut self) {
        *self.direction = 0;
        *self.output = 1;
    }

    pub fn clear(&mut self) {
        *self.output = 0;
        *self.direction = 1;
    }

    pub fn toggle(&mut self) {
        if *self.input == 0 {
            self.set();
        } else {
            self.clear();
        }
    }
}

fn set_pin_function_to_gpio(port: &mut GpioPort, pin_offset: u8) {
    // Set function select bits to 00 (GPIO).
    let sel0_addr = peripheral_to_alias(((&mut port.select_0) as *mut u16) as u32, pin_offset);

    let sel0_reg = unsafe { &mut *(sel0_addr as *mut u16) };

    let sel1_addr = peripheral_to_alias(((&mut port.select_1) as *mut u16) as u32, pin_offset);

    let sel1_reg = unsafe { &mut *(sel1_addr as *mut u16) };

    let select_status = (*sel1_reg << 1) | *sel0_reg;
    match select_status {
        1 => {
            // Clear Select 0.
            *sel0_reg = 0;
        }

        2 => {
            // Clear Select 1.
            *sel1_reg = 0;
        }

        3 => {
            // Use the Select Compliment register to ensure atomic clearing of both Select 0 and
            // Select 1.

            let selc_addr = peripheral_to_alias(
                ((&mut port.compliment_selection) as *mut u16) as u32,
                pin_offset,
            );
        }

        _ => {
            debug_assert_eq!(select_status, 0);
        }
    }
}
