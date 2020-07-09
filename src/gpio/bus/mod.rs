//! # Bus
//! The `bus` module includes structures and function to utilize GPIO as groups of pins.

//
// TODO: Interrupts for Inputs
//

//
// TODO: Drive strength for Outputs
//

//
// Internal Modules
//

mod portbus;

//
// Reexports
//

pub use portbus::*;

//
// Dependencies
//

use crate::gpio::*;

//
// Traits
//

/// A GPIO Bus instance that is configured as an input.
pub trait GpioBusInput {
    /// Reads the value of the GPIO Bus.
    ///
    /// # Returns
    /// Value of the GPIO Bus.
    fn read(&self) -> usize;
}

/// A GPIO Bus instance ths is configured as an output.
pub trait GpioBusOutput {
    /// Sets the value of the GPIO Bus.
    ///
    /// # Arguments
    /// `value` - The value to write to the bus.
    /// `_port_sync_token` - Indicates that no other thread can access the GPIO port(s) this bus
    /// belongs to.
    fn write(&mut self, value: usize, _port_sync_token: &mut GpioPortInUseToken);

    /// Sets bits on the GPIO Bus.
    ///
    /// # Arguments
    /// `set_mask` - The bits to set on the bus.
    /// `_port_sync_token` - Indicates that no other thread can access the GPIO port(s) this bus
    /// belongs to.
    fn set_bits(&mut self, set_mask: usize, _port_sync_token: &mut GpioPortInUseToken);

    /// Clears bits on the GPIO Bus.
    ///
    /// # Arguments
    /// `clear_mask` - The bits to clear on the bus.
    /// `_port_sync_token` - Indicates that no other thread can access the GPIO port(s) this bus
    /// belongs to.
    fn clear_bits(&mut self, clear_mask: usize, _port_sync_token: &mut GpioPortInUseToken);

    /// Toggles bits on the GPIO Bus.
    ///
    /// # Arguments
    /// `toggle_mask` - The bits to toggle on the bus.
    /// `_port_sync_token` - Indicates that no other thread can access the GPIO port(s) this bus
    /// belongs to.
    fn toggle_bits(&mut self, toggle_mask: usize, _port_sync_token: &mut GpioPortInUseToken);

    /// Sets the value of the GPIO Bus.
    ///
    /// # Arguments
    /// `value` - The value to write to the bus.
    ///
    /// # Unsafe
    /// This function is safe to use only if there are no active GPIO pins or buses that are in the
    /// same port as this bus.
    unsafe fn write_no_sync(&mut self, value: usize);

    /// Sets bits on the GPIO Bus.
    ///
    /// # Arguments
    /// `set_mask` - The bits to set on the bus.
    ///
    /// # Unsafe
    /// This function is safe to use only if there are no active GPIO pins or buses that are in the
    /// same port as this bus.
    unsafe fn set_bits_no_sync(&mut self, set_mask: usize);

    /// Clears bits on the GPIO Bus.
    ///
    /// # Arguments
    /// `clear_mask` - The bits to clear on the bus.
    ///
    /// # Unsafe
    /// This function is safe to use only if there are no active GPIO pins or buses that are in the
    /// same port as this bus.
    unsafe fn clear_bits_no_sync(&mut self, clear_mask: usize);

    /// Toggles bits on the GPIO Bus.
    ///
    /// # Arguments
    /// `toggle_mask` - The bits to toggle on the bus.
    ///
    /// # Unsafe
    /// This function is safe to use only if there are no active GPIO pins or buses that are in the
    /// same port as this bus.
    unsafe fn toggle_bits_no_sync(&mut self, toggle_mask: usize);
}
