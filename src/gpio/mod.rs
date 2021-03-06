//! # GPIO
//! The `gpio` module includes structures and functions to utilize General Purpose Input and Output
//! (GPIO) pins.

// TODO: Bus/pin interaction.

//
// Internal Modules
//

mod bus;
mod pin;

//
// Reexports
//

pub use bus::*;
pub use pin::*;

use crate::registers::{ReadOnly, ReadWrite, Reserved, PERIPHERAL_BASE};

//
// Dependencies
//

//
// Represents the different GPIO typestate configurations.
//

/// Represents a GPIO configuration mode.
pub trait GpioMode: private::Sealed {}

/// Represents a GPIO Input configuration mode.
pub trait GpioInputMode: private::Sealed {}

/// Represents a GPIO Output configuration mode.
pub trait GpioOutputMode: private::Sealed {}

/// A zero-sized typestate indicating a Disabled GPIO instance configuration. This is the
/// default typestate when a new GPIO instance is created.
pub struct Disabled;
impl GpioMode for Disabled {}

/// A zero-sized typestate indicating a GPIO instance input configuration.
/// # Type Options
/// `InputMode` indicates the type of input configuration. Can be of type `HighImpedance`,
/// `PullUp`, or `PullDown`.
pub struct GpioIn<InputMode: GpioInputMode> {
    _input_mode: InputMode,
}

impl<InputMode: GpioInputMode> GpioMode for GpioIn<InputMode> {}

/// A zero-sized typestate indicating a high-Z GPIO instance input configuration.
pub struct HighImpedance;
impl GpioInputMode for HighImpedance {}

/// A zero-sized typestate indicating a pull-up resistor GPIO instance input configuration.
pub struct PullUp;
impl GpioInputMode for PullUp {}

/// A zero-sized typestate indicating a pull-down resistor GPIO instance input configuration.
pub struct PullDown;
impl GpioInputMode for PullDown {}

/// A zero-sized typestate indicating a GPIO instance output configuration.
/// # Type Options
/// `OutputMode` indicates the type of output configuration. Can be of type `PushPull` or
/// `OpenCollector`.
pub struct GpioOut<OutputMode: GpioOutputMode> {
    _output_mode: OutputMode,
}

impl<OutputMode: GpioOutputMode> GpioMode for GpioOut<OutputMode> {}

/// A zero-sized typestate indicating a push-pull GPIO instance output configuration.
pub struct PushPull;
impl GpioOutputMode for PushPull {}

/// A zero-sized typestate indicating an open collector GPIO instance output configuration.
pub struct OpenCollector;
impl GpioOutputMode for OpenCollector {}

//
// Consts
//

/// Base address of the GPIO Port modules.
const PORT_MODULE: u32 = PERIPHERAL_BASE + 0x4C00;
const PORT_A: u32 = PORT_MODULE;
const PORT_B: u32 = PORT_A + core::mem::size_of::<GpioPort>() as u32;
const PORT_C: u32 = PORT_B + core::mem::size_of::<GpioPort>() as u32;
const PORT_D: u32 = PORT_C + core::mem::size_of::<GpioPort>() as u32;
const PORT_E: u32 = PORT_D + core::mem::size_of::<GpioPort>() as u32;
const PORT_J: u32 = PORT_MODULE + 0x120;

//
// Globals
//

#[repr(C)]
/// GPIO Register layout
struct GpioPort {
    /// Level of the GPIO pins.
    input: ReadOnly<u16>,

    /// Drives the level of the GPIO pins when the direction bit for a corresponding pin is 1.
    /// If direction = 0 and resistor_enable = 1, indicates the level of the internal resistor
    /// (pull-up = 1, pull-down = 0)
    output: ReadWrite<u16>,

    /// The direction of the pins. Input = 0, Output = 1.
    direction: ReadWrite<u16>,

    /// If 1, enables either the pull-up or pull-down resistor for the corresponding pins.
    /// Does nothing when direction = 0.
    resistor_enable: ReadWrite<u16>,

    /// If the specific port supports high drive strength, enables high drive strength mode.
    /// Otherwise does nothing.
    drive_strength: ReadWrite<u16>,

    /// The lower bit of the function select for a given pin.
    select_0: ReadWrite<u16>,

    /// The upper bit of the function select for a given pin.
    select_1: ReadWrite<u16>,

    /// Unused.
    reserved: (Reserved<u16>, Reserved<u16>, Reserved<u16>, Reserved<u16>),

    /// If 1 is written, inverts both bits of the function select for a given pin.
    complement_selection: ReadWrite<u16>,

    /// If 0, the interrupt flag will be set on a low to high transition.
    /// If 1, the interrupt flag will be set on a high to low transition.
    interrupt_edge_select: ReadWrite<u16>,

    /// Enables interrupts for a given pin.
    interrupt_enable: ReadWrite<u16>,

    /// Indicates whether a high to low or low to high transition occured when interrupts are
    /// enabled for a given pin.
    interrupt_flag: ReadOnly<u16>,

    /// Unused.
    reserved2: Reserved<u16>,
}

//
// Module private functions.
//

/// Gets the GPIO port address for the port provided.
///
/// # Arguments
/// `port` - Provides the port to get the GPIO port address for.
///
/// # Returns
/// The address of the port that the provided pin belongs to.
fn get_gpio_port(port_name: char) -> &'static GpioPort {
    let addr = match port_name {
        'A' => PORT_A,
        'B' => PORT_B,
        'C' => PORT_C,
        'D' => PORT_D,
        'E' => PORT_E,
        'J' => PORT_J,
        _ => 0,
    };

    unsafe { &*(addr as *const GpioPort) }
}

//
// For sealed traits.
//

mod private {
    pub trait Sealed {}
}

impl private::Sealed for Disabled {}
impl<InputMode: GpioInputMode> private::Sealed for GpioIn<InputMode> {}
impl<OutputMode: GpioOutputMode> private::Sealed for GpioOut<OutputMode> {}

impl private::Sealed for HighImpedance {}
impl private::Sealed for PullUp {}
impl private::Sealed for PullDown {}

impl private::Sealed for PushPull {}
impl private::Sealed for OpenCollector {}
