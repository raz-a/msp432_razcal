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

mod pin;
mod port;
mod port_section;

//
// Reexports
//

pub use pin::*;
pub use port::*;
pub use port_section::*;

//
// Dependencies
//

#[cfg(not(any(
    razcal_msp432_package = "vqfn",
    razcal_msp432_package = "nfbga",
    razcal_msp432_package = "lqfp"
)))]
compile_error!("Msp432 package must be defined.");

//
// Traits.
//

/// Describes an entity that is a component of a port.
pub trait PortComponent: private::Sealed {
    /// Gets the mask of this entity within its owning port.
    fn get_port_mask(&self) -> u16;

    /// Gets the inverse of the mask of this entity within its owning port.
    fn get_port_clear_mask(&self) -> u16;
}

mod private {
    pub trait Sealed {}
}
