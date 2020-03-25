mod bus;
mod pin;

pub use bus::*;
pub use pin::*;

use super::PERIPHERAL_BASE;
use crate::pin::*;

pub struct Disabled;

pub struct HighImpedance;
pub struct PullUp;
pub struct PullDown;

pub struct GpioInConfig<InputMode> {
    _input_mode: InputMode,
}

pub struct PushPull;
pub struct OpenCollector;

pub struct GpioOutConfig<OutputMode> {
    _output_mode: OutputMode,
}

const PORT_MODULE: u32 = PERIPHERAL_BASE + 0x4C00;
const PORT_J_OFFSET: u32 = 0x120;

#[repr(C)]
struct GpioPort {
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
    interrupt_vector_high: u16,
}

fn get_port_address(pin: &Pin) -> u32 {
    let port_number = pin.get_port() as u32;
    if port_number == PortName::PortJ as u32 {
        PORT_MODULE + PORT_J_OFFSET
    } else {
        PORT_MODULE + (core::mem::size_of::<GpioPort>() as u32) * port_number
    }
}
