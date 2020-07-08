//! # PortBus
//! The `portbus` module includes structures and functions to utilize a port as a GPIO bus.

//
// Dependencies
//

use crate::gpio::GpioPort;
use crate::pin::Port;

//
// Structures
//

/// Represents a port configured as a GPIO Bus.
/// `GpioConfig` indicated the specific configuration mode the GPIO bus is in. Can be of type
/// `Disabled`, `GpioInConfig`, or `GpioOutConfig`.
pub struct GpioPortBus<GpioConfig> {
    /// The specfic GPIO configuration.
    _config: GpioConfig,

    //
    // Points to the corresponding port registers.
    //
    port_regs: GpioPort,

    //
    // The port in use.
    //
    port: Port,
}
