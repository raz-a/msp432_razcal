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

//
// Reexports
//

pub use pin::*;
pub use port::*;

//
// Dependencies
//

#[cfg(not(any(
    razcal_msp432_package = "vqfn",
    razcal_msp432_package = "nfbga",
    razcal_msp432_package = "lqfp"
)))]
compile_error!("Msp432 package must be defined.");
