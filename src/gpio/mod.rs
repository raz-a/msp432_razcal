mod bus;
mod pin;

pub use bus::*;
pub use pin::*;

use super::PERIPHERAL_BASE;
use crate::pin::*;
use crate::registers::Reg16;
use core::sync::atomic::{AtomicU16, Ordering};

// Typestates

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

// Synchronization

pub struct GpioPortInUseToken {
    free_mask: u16,
}

impl Drop for GpioPortInUseToken {
    fn drop(&mut self) {
        unsafe { GPIO_PORT_IN_USE_LOCK.fetch_nand(self.free_mask, Ordering::Relaxed) };
    }
}

pub trait GpioPortSync {
    fn get_port_in_use_token(&self) -> Option<GpioPortInUseToken>;
}

// Consts

const PORT_MODULE: u32 = PERIPHERAL_BASE + 0x4C00;
const PORT_J_OFFSET: u32 = 0x120;

// Globals

static mut GPIO_PORT_IN_USE_LOCK: AtomicU16 = AtomicU16::new(0);

// Gpio Register layout

#[repr(C)]
struct GpioPort {
    input: Reg16,
    output: Reg16,
    direction: Reg16,
    resistor_enable: Reg16,
    drive_strength: Reg16,
    select_0: Reg16,
    select_1: Reg16,
    reserved: (u16, u16, u16, u16),
    complement_selection: Reg16,
    interrupt_edge_select: Reg16,
    interrupt_enable: Reg16,
    interrupt_flag: Reg16,
    reserved2: u16,
}

// Module private functions.

fn get_port_address(pin: &Pin) -> u32 {
    let port_number = pin.get_port() as u32;
    if port_number == PortName::PortJ as u32 {
        PORT_MODULE + PORT_J_OFFSET
    } else {
        PORT_MODULE + (core::mem::size_of::<GpioPort>() as u32) * port_number
    }
}
