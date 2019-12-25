
use crate::pin::Pin;
use crate::Reg16;

/// TODO: Add support for more thsn just port 2

const P2OUT_BITBAND: usize = 0x4209_8060;
const P2DIR_BITBAND: usize = 0x4209_80A0;

#[repr(C)]
struct Port {
    input: Reg16,
    output: Reg16,
    resistor_enable: Reg16,
    drive_strength: Reg16,
    select_0: Reg16,
    select_1: Reg16,
    interrupt_vector_low: Reg16,
    reserved: (Reg16, Reg16, Reg16),
    compliment_selection: Reg16,
    interrupt_edge_select: Reg16,
    interrupt_enable: Reg16,
    interrupt_flag: Reg16,
    interrupt_vector_high: Reg16
}

pub trait GpioOut {
    fn new(pin: Pin) -> Self;
    fn get_current_state(&self) -> bool;
    fn set(&mut self, value: bool);
    fn toggle(&mut self);
}

pub struct PushPullGpioOut {
    out_bitband_addr: &'static mut u8,
    dir_bitband_addr: &'static mut u8,
    _pin: Pin
}

impl GpioOut for PushPullGpioOut {
    fn new(pin: Pin) -> Self {
        let out_value = P2OUT_BITBAND + ((pin.get_pin_offset_in_port() as usize) - 8) * core::mem::size_of::<u32>();
        let dir_value = P2DIR_BITBAND + ((pin.get_pin_offset_in_port() as usize) - 8) * core::mem::size_of::<u32>();
        let gpio_out = unsafe {
            PushPullGpioOut {
                out_bitband_addr: &mut *(out_value as *mut u8),
                dir_bitband_addr: &mut *(dir_value as *mut u8),
                _pin: pin
            }
        };

        unsafe {
            core::ptr::write_volatile(gpio_out.dir_bitband_addr, 1);
        }

        gpio_out
    }

    fn get_current_state(&self) -> bool {
        unsafe {
            core::ptr::read_volatile(self.out_bitband_addr) != 0
        }
    }

    fn set(&mut self, value: bool) {
        unsafe {
            core::ptr::write_volatile(self.out_bitband_addr, value as u8);
        }
    }

    fn toggle(&mut self) {
        let value = unsafe {
            core::ptr::read_volatile(self.out_bitband_addr) == 0
        };

        self.set(value);
    }
}
