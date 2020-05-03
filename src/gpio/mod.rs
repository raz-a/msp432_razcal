//! # Gpio
//! The `gpio` module includes structures and functions to utilize General Purpose Input and Output
//! (GPIO) pins.

//
// Internal Modules
//

mod bus;
mod pin;

//
// Reexports
//

pub use bus::*;
pub use gpio_typestates::*;
pub use pin::*;

//
// Dependencies
//

use super::PERIPHERAL_BASE;
use crate::pin::*;
use crate::registers::Reg16;
use core::sync::atomic::{AtomicU16, Ordering};

/// Represents the different GPIO typestate configurations
mod gpio_typestates {
    /// A zero-sized typestate indicating a Disabled GPIO instance configuration. This is the
    /// default typestate when a new GPIO instance is created.
    pub struct Disabled;

    /// A zero-sized typestate indicating a GPIO instance input configuration.
    /// # Type Options
    /// `InputMode` indicates the type of input configuration. Can be of type `HighImpedance`,
    /// `PullUp`, or `PullDown`.
    pub struct GpioInConfig<InputMode> {
        _input_mode: InputMode,
    }

    /// A zero-sized typestate indicating a high-Z GPIO instance input configuration.
    pub struct HighImpedance;

    /// A zero-sized typestate indicating a pull-up resistor GPIO instance input configuration.
    pub struct PullUp;

    /// A zero-sized typestate indicating a pull-down resistor GPIO instance input configuration.
    pub struct PullDown;

    /// A zero-sized typestate indicating a GPIO instance output configuration.
    /// # Type Options
    /// `OutputMode` indicates the type of output configuration. Can be of type `PushPull` or
    /// `OpenCollector`.
    pub struct GpioOutConfig<OutputMode> {
        _output_mode: OutputMode,
    }

    /// A zero-sized typestate indicating a push-pull GPIO instance output configuration.
    pub struct PushPull;

    /// A zero-sized typestate indicating an open collector GPIO instance output configuration.
    pub struct OpenCollector;
}

//
// Synchronization
//

/// A token representing that the port owned by a given GPIO instance is in use by the holder.
pub struct GpioPortInUseToken {
    /// The mask used to restore the `In Use` global keeping track of port exclusive usage.
    free_mask: u16,
}

impl Drop for GpioPortInUseToken {
    fn drop(&mut self) {
        //
        // UNSAFE: Mutable global is being written to. Safety ensured by using an atomic operation.
        //

        unsafe { GPIO_PORT_IN_USE_LOCK.fetch_nand(self.free_mask, Ordering::Relaxed) };
    }
}

/// A GPIO instance that can synchronize usage with other instances on the same 8-bit or 16-bit
/// port.
pub trait GpioPortSync {
    /// Attempts to obtain a GpioPortInUseToken. This function will not succeed nested calls.
    ///
    /// # Returns
    /// `Some(GpioPortInUseToken)` if GPIO port was free and obtained by the caller.
    /// `None` if GPIO port was in use.
    fn get_port_in_use_token(&self) -> Option<GpioPortInUseToken>;
}

//
// Consts
//

/// Base address of the GPIO Port modules.
const PORT_MODULE: u32 = PERIPHERAL_BASE + 0x4C00;

/// Offset from the Base Port module address to PortJ.
const PORT_J_OFFSET: u32 = 0x120;

//
// Globals
//

/// Represents which ports may have non atomic operations currentoly acting on them.
/// It is organized as follows:
/// 0b000000  X     X      X     X     X     X     X     X     X     X     X
///         PortJ Port10 Port9 Port8 Port7 Port6 Port5 Port4 Port3 Port2 Port1
///               |---PortE---||--PortD--| |--PortC--| |--PortB--| |--PortA--|
static mut GPIO_PORT_IN_USE_LOCK: AtomicU16 = AtomicU16::new(0);

#[repr(C)]
/// Gpio Register layout
struct GpioPort {
    /// Level of the GPIO pins.
    input: Reg16,

    /// Drives the level of the GPIO pins when the direction bit for a corresponding pin is 1.
    /// If direction = 0 and resistor_enable = 1, indicates the level of the internal resistor
    /// (pull-up = 1, pull-down = 0)
    output: Reg16,

    /// The direction of the pins. Input = 0, Output = 1.
    direction: Reg16,

    /// If 1, enables either the pull-up or pull-down resistor for the corresponding pins.
    /// Does nothing when direction = 0.
    resistor_enable: Reg16,

    /// If the specific port supports high drive strength, enables high drive strength mode.
    /// Otherwise does nothing.
    drive_strength: Reg16,

    /// The lower bit of the function select for a given pin.
    select_0: Reg16,

    /// The upper bit of the function select for a given pin.
    select_1: Reg16,

    /// Unused.
    reserved: (u16, u16, u16, u16),

    /// If 1 is written, inverts both bits of the function select for a given pin.
    complement_selection: Reg16,

    /// If 0, the interrupt flag will be set on a low to high transition.
    /// If 1, the interrupt flag will be set on a high to low transition.
    interrupt_edge_select: Reg16,

    /// Enables interrupts for a given pin.
    interrupt_enable: Reg16,

    /// Indicates whether a high to low or low to high transition occured when interrupts are
    /// enabled for a given pin.
    interrupt_flag: Reg16,

    /// Unused.
    reserved2: u16,
}

//
// Module private functions.
//

/// Gets the port address for the pin provided.
///
/// # Arguments
/// `pin` - Provides a reference to the pin.
///
/// # Returns
/// The address of the port that the provided pin belongs to.
fn get_port_address(pin: &Pin) -> u32 {
    let port_number = pin.get_port() as u32;
    if port_number == PortName::PortJ as u32 {
        PORT_MODULE + PORT_J_OFFSET
    } else {
        PORT_MODULE + (core::mem::size_of::<GpioPort>() as u32) * port_number
    }
}
