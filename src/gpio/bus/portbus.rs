//! # PortBus
//! The `portbus` module includes structures and functions to utilize a port as a GPIO bus.

//
// Dependencies
//

use crate::{
    gpio::{
        get_gpio_port, Disabled, GpioIn, GpioInputMode, GpioMode, GpioOut, GpioOutputMode,
        HighImpedance, OpenCollector, PullDown, PullUp, PushPull,
    },
    pin::PortX,
};

use super::{private, GpioBusInput, GpioBusOutput};

//
// Constants
//

const ALL_PINS_MASK: u16 = 0xFFFF;

//
// Structures
//

/// Represents a port configured as a GPIO Bus.
pub struct GpioPortBus<Port: PortX, Mode: GpioMode> {
    /// The specfic GPIO configuration.
    _config: Mode,

    //
    // The actual port.
    //
    port: Port,
}

/// The following implements state modification for GPIO Port Bus configurations.
impl<Port: PortX, Mode: GpioMode> GpioPortBus<Port, Mode> {
    /// Convert this port into a high-impedance input bus.
    ///
    /// # Returns
    /// A GPIO Port Bus instance configured in high-impedance input mode.
    pub fn to_input_highz(self) -> GpioPortBus<Port, GpioIn<HighImpedance>> {
        let port_regs = get_gpio_port(self.port.get_port_name());

        port_regs.resistor_enable.write(0);
        port_regs.direction.write(0);

        GpioPortBus {
            _config: GpioIn {
                _input_mode: HighImpedance,
            },

            port: self.port,
        }
    }

    /// Convert this port into an input bus with pull-up resistors.
    ///
    /// # Returns
    /// A GPIO Port Bus instance configured in input mode with pull-up resistors.
    pub fn to_input_pullup(self) -> GpioPortBus<Port, GpioIn<PullUp>> {
        let port_regs = get_gpio_port(self.port.get_port_name());

        port_regs.resistor_enable.write(ALL_PINS_MASK);
        port_regs.direction.write(0);
        port_regs.output.write(ALL_PINS_MASK);

        GpioPortBus {
            _config: GpioIn {
                _input_mode: PullUp,
            },

            port: self.port,
        }
    }

    /// Convert this port into an input bus with pull-down resistors.
    ///
    /// # Returns
    /// A GPIO Port Bus instance configured in input mode with pull-down resistors.
    pub fn to_input_pulldown(self) -> GpioPortBus<Port, GpioIn<PullDown>> {
        let port_regs = get_gpio_port(self.port.get_port_name());

        port_regs.resistor_enable.write(ALL_PINS_MASK);
        port_regs.direction.write(0);
        port_regs.output.write(0);

        GpioPortBus {
            _config: GpioIn {
                _input_mode: PullDown,
            },

            port: self.port,
        }
    }

    /// Convert this port into an output bus with push-pull configuration.
    ///
    /// # Returns
    /// A GPIO Port Bus instance configured in output mode with push-pull configuration.
    pub fn to_output_pushpull(self) -> GpioPortBus<Port, GpioOut<PushPull>> {
        let port_regs = get_gpio_port(self.port.get_port_name());

        port_regs.output.write(0);
        port_regs.direction.write(ALL_PINS_MASK);

        GpioPortBus {
            _config: GpioOut {
                _output_mode: PushPull,
            },

            port: self.port,
        }
    }

    /// Convert this port into an output bus with open collector configuration.
    ///
    /// # Returns
    /// A GPIO Port Bus instance configured in output mode with open collector configuration.
    pub fn to_output_opencollector(self) -> GpioPortBus<Port, GpioOut<OpenCollector>> {
        let port_regs = get_gpio_port(self.port.get_port_name());

        port_regs.output.write(0);
        port_regs.direction.write(ALL_PINS_MASK);
        port_regs.resistor_enable.write(ALL_PINS_MASK);

        GpioPortBus {
            _config: GpioOut {
                _output_mode: OpenCollector,
            },

            port: self.port,
        }
    }
}

impl<Port: PortX, InputMode: GpioInputMode> GpioBusInput for GpioPortBus<Port, GpioIn<InputMode>> {
    /// Reads the value of the GPIO Bus.
    ///
    /// # Returns
    /// Value of the GPIO Bus.
    fn read(&self) -> usize {
        let port_regs = get_gpio_port(self.port.get_port_name());
        port_regs.input.read() as usize
    }
}

impl<Port: PortX, OutputMode: GpioOutputMode> GpioBusInput
    for GpioPortBus<Port, GpioOut<OutputMode>>
{
    /// Reads the value of the GPIO Bus.
    ///
    /// # Returns
    /// Value of the GPIO Bus.
    fn read(&self) -> usize {
        let port_regs = get_gpio_port(self.port.get_port_name());
        port_regs.input.read() as usize
    }
}

impl<Port: PortX> GpioBusOutput for GpioPortBus<Port, GpioOut<PushPull>> {
    /// Sets the value of the GPIO Bus.
    ///
    /// # Arguments
    /// `value` - The value to write to the bus.
    fn write(&mut self, value: usize) {
        let port_regs = get_gpio_port(self.port.get_port_name());
        port_regs.output.write(value as u16);
    }

    /// Sets bits on the GPIO Bus.
    ///
    /// # Arguments
    /// `set_mask` - The bits to set on the bus.
    fn set_bits(&mut self, set_mask: usize) {
        let port_regs = get_gpio_port(self.port.get_port_name());
        port_regs.output.modify(|value| value | set_mask as u16);
    }

    /// Clears bits on the GPIO Bus.
    ///
    /// # Arguments
    /// `clear_mask` - The bits to clear on the bus.
    fn clear_bits(&mut self, clear_mask: usize) {
        let port_regs = get_gpio_port(self.port.get_port_name());
        port_regs.output.modify(|value| value & !clear_mask as u16);
    }

    /// Toggles bits on the GPIO Bus.
    ///
    /// # Arguments
    /// `toggle_mask` - The bits to toggle on the bus.
    fn toggle_bits(&mut self, toggle_mask: usize) {
        let port_regs = get_gpio_port(self.port.get_port_name());
        port_regs.output.modify(|value| value ^ toggle_mask as u16);
    }
}

//
// Note: GpioPortBus<Port, GpioOut<OpenCollector>> is not implemented as the output value cannot
// be changed atomically.
//

impl<Port: PortX> GpioPortBus<Port, Disabled> {
    /// Allocates a new GPIO configured Port.
    ///
    /// # Arguments
    /// `port` - Provides the port to be configred for GPIO.
    ///
    /// # Returns
    /// A GPIO Port in the `Disabled` configuration.
    pub fn new(port: Port) -> Self {
        Self {
            _config: Disabled,
            port: port,
        }
    }
}

impl<Port: PortX, Mode: GpioMode> private::Sealed for GpioPortBus<Port, Mode> {}
