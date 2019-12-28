
use crate::pin::{Pin, PortName};

/// TODO: Atmoic accesses when possible

const PORT_MODULE: usize = 0x4000_4C00;
const PORT_J_OFFSET: usize = 0x120;

#[repr(C)]
struct Port {
    input: u16,
    output: u16,
    direction: u16,
    resistor_enable: u16,
    drive_strength: u16,
    select_0: u16,
    select_1: u16,
    interrupt_vector_low: u16,
    reserved: (u16, u16, u16),
    compliment_selection: u16,
    interrupt_edge_select: u16,
    interrupt_enable: u16,
    interrupt_flag: u16,
    interrupt_vector_high: u16
}

pub trait GpioOut {
    fn new(pin: Pin) -> Self;
    fn get_current_state(&self) -> bool;
    fn set(&mut self, value: bool);
    fn toggle(&mut self);
}

pub struct PushPullGpioOut {
    port: &'static mut Port,
    mask: u16,
    _pin: Pin
}

impl GpioOut for PushPullGpioOut {
    fn new(pin: Pin) -> Self {
        let port_number = pin.get_port();
        let mask: u16 = 1 << pin.get_pin_offset_in_port();

        let addr = if port_number == PortName::port_j as u8 {
            PORT_MODULE + PORT_J_OFFSET
        
        } else {
            PORT_MODULE + core::mem::size_of::<Port>() * port_number as usize
        };

        let gpio_out = unsafe {
            PushPullGpioOut {
                port: &mut *(addr as *mut Port),
                mask: mask,
                _pin: pin
            }
        };

        gpio_out.port.direction &= !gpio_out.mask;
        gpio_out
    }

    fn get_current_state(&self) -> bool {
        self.port.output & self.mask != 0
    }

    fn set(&mut self, value: bool) {
        if value {
            self.port.output |= self.mask
        
        } else {
            self.port.output &= !self.mask
        }
    }

    fn toggle(&mut self) {
        let value = self.port.output & self.mask;
        self.set(value == 0);
    }
}
