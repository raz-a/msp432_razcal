//! # Bus
//! The `bus` module includes structures and function to utilize GPIO as groups of pins.

// TODO: Seal traits.

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
// Traits
//

/// A GPIO Bus instance that is configured as an input.
pub trait GpioBusInput: private::Sealed {
    /// Reads the value of the GPIO Bus.
    ///
    /// # Returns
    /// Value of the GPIO Bus.
    fn read(&self) -> usize;
}

/// A GPIO Bus instance ths is configured as an output.
pub trait GpioBusOutput: private::Sealed {
    /// Sets the value of the GPIO Bus.
    ///
    /// # Arguments
    /// `value` - The value to write to the bus.
    fn write(&mut self, value: usize);

    /// Sets bits on the GPIO Bus.
    ///
    /// # Arguments
    /// `set_mask` - The bits to set on the bus.
    fn set_bits(&mut self, set_mask: usize);

    /// Clears bits on the GPIO Bus.
    ///
    /// # Arguments
    /// `clear_mask` - The bits to clear on the bus.
    fn clear_bits(&mut self, clear_mask: usize);

    /// Toggles bits on the GPIO Bus.
    ///
    /// # Arguments
    /// `toggle_mask` - The bits to toggle on the bus.
    fn toggle_bits(&mut self, toggle_mask: usize);
}

//
// For sealed traits.
//

mod private {
    pub trait Sealed {}
}
