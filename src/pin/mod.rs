//! # Pin
//! The `pin` module includes structures and functions to wrap microcontroller pins in borrowable
//! structures.

//
// Assure configuration variables are set.
//

#[cfg(not(all(razcal_gpio_port_size = "8", razcal_gpio_port_size = "16")))]
compile_error!("razcal_gpio_port_size should be defined as both 8 and 16 for MSP432");

//
// Internal Modules
//

mod names;
mod newpin;
mod pin;
mod port;

//
// Reexports
//

pub use names::*;
pub use newpin::*;
pub use pin::*;
pub use port::*;

//
// Dependencies
//

use core::sync::atomic::AtomicU16;

#[cfg(not(any(
    razcal_msp432_package = "vqfn",
    razcal_msp432_package = "nfbga",
    razcal_msp432_package = "lqfp"
)))]
compile_error!("Msp432 package must be defined.");

//
// Globals
//

/// Represents the pins available for the given controller.
#[cfg(razcal_msp432_package = "vqfn")]
static mut PORT_PINS_AVAILABLE: [AtomicU16; 6] = [
    AtomicU16::new(0x0FFF),
    AtomicU16::new(0xFCFF),
    AtomicU16::new(0xC0FF),
    AtomicU16::new(0x03FF),
    AtomicU16::new(0x0000),
    AtomicU16::new(0x003F),
];

/// Represents the pins available for the given controller.
#[cfg(razcal_msp432_package = "nfbga")]
static mut PORT_PINS_AVAILABLE: [AtomicU16; 6] = [
    AtomicU16::new(0xFFFF),
    AtomicU16::new(0xFFFF),
    AtomicU16::new(0xFFFF),
    AtomicU16::new(0x03FF),
    AtomicU16::new(0x0000),
    AtomicU16::new(0x003F),
];

/// Represents the pins available for the given controller.
#[cfg(razcal_msp432_package = "lqfp")]
static mut PORT_PINS_AVAILABLE: [AtomicU16; 6] = [
    AtomicU16::new(0xFFFF),
    AtomicU16::new(0xFFFF),
    AtomicU16::new(0xFFFF),
    AtomicU16::new(0xFFFF),
    AtomicU16::new(0xFFFF),
    AtomicU16::new(0x003F),
];
