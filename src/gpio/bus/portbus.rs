//! # PortBus
//! The `portbus` module includes structures and functions to utilize a port as a GPIO bus.

//
// Dependencies
//

use crate::gpio::{GpioInConfig, GpioPort, HighImpedance};
use crate::pin::Port;

//
// Constants
//

const ALL_PINS_MASK: u16 = 0xFFFF;

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

/// The following implements state modification for GPIO Port Bus configurations.
impl<GpioConfig> GpioPortBus<GpioConfig> {
    /// Convert this port into a high-impedance input bus.
    ///
    /// # Returns
    /// A GPIO Port Bus instance configured in high-impedance input mode.
    pub fn to_input_highz(mut self) -> GpioPortBus<GpioInConfig<HighImpedance>> {
        self.port_regs.resistor_enable.set_halfword(0);
        self.port_regs.direction.set_halfword(0);
        GpioPortBus {
            _config: GpioInConfig {
                _input_mode: HighImpedance,
            },

            port_regs: self.port_regs,
            port: self.port,
        }
    }
}
