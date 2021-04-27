//! # Pin
//! The `pin` module includes structures and functions to utilize GPIO as single independent pins.

//
// TODO: Interrupts for Inputs
//

//
// TODO: Drive strength for Outputs
//

//
// Dependencies
//

use crate::{
    gpio::*,
    pin::{PinIdWithMode, PinMode, PinX},
};
use core::sync::atomic::{compiler_fence, Ordering};

//
// Traits
//

/// A GPIO Pin instance that is configured as an input.
pub trait GpioPinInput: private::Sealed {
    /// Reads the value of the GPIO pin.
    ///
    /// # Returns
    /// `true` if pin is high.
    /// `false` if pin is low.
    fn read(&self) -> bool;
}

/// A GPIO Pin instance that is configred as an output.
pub trait GpioPinOutput: private::Sealed {
    /// Sets the GPIO Pin high.
    fn set(&mut self);

    /// Sets the GPIO Pin low.
    fn clear(&mut self);

    /// Toggles the GPIO Pin.
    fn toggle(&mut self);
}

//
// Structures
//

/// Represents a pin configured for GPIO mode.
/// # Type Options
/// `GpioConfig` indicated the specific configuration mode the GPIO pin is in. Can be of type
/// `Disabled`, `GpioInConfig`, or `GpioOutConfig`.
pub struct GpioPin<Pin: PinX, Mode: GpioMode> {
    /// The specfic GPIO configuration.
    _config: Mode,

    /// The actual pin.
    pin: Pin,
}

/// The following implements state modification for GPIO Pin configurations.
impl<Pin: PinX, Mode: GpioMode> GpioPin<Pin, Mode> {
    /// Convert this instance into a high-impedance input pin.
    ///
    /// # Returns
    /// A GPIO Pin instance configured in high-impedance input mode.
    pub fn to_input_highz(self) -> GpioPin<Pin, GpioIn<HighImpedance>> {
        let port_regs = get_gpio_port(self.pin.get_port_name());

        port_regs
            .resistor_enable
            .get_bitband(self.pin.get_offset())
            .write(false);

        port_regs
            .direction
            .get_bitband(self.pin.get_offset())
            .write(false);

        GpioPin {
            _config: GpioIn {
                _input_mode: HighImpedance,
            },

            pin: self.pin,
        }
    }

    /// Convert this instance into a input pin with a pull-up resistor.
    ///
    /// # Returns
    /// A GPIO Pin instance configured in pull-up input mode.
    pub fn to_input_pullup(self) -> GpioPin<Pin, GpioIn<PullUp>> {
        let port_regs = get_gpio_port(self.pin.get_port_name());

        port_regs
            .resistor_enable
            .get_bitband(self.pin.get_offset())
            .write(true);

        port_regs
            .direction
            .get_bitband(self.pin.get_offset())
            .write(false);

        port_regs
            .output
            .get_bitband(self.pin.get_offset())
            .write(true);

        GpioPin {
            _config: GpioIn {
                _input_mode: PullUp,
            },

            pin: self.pin,
        }
    }

    /// Convert this instance into a input pin with a pull-down resistor.
    ///
    /// # Returns
    /// A GPIO Pin instance configured in pull-down input mode.
    pub fn to_input_pulldown(self) -> GpioPin<Pin, GpioIn<PullDown>> {
        let port_regs = get_gpio_port(self.pin.get_port_name());

        port_regs
            .resistor_enable
            .get_bitband(self.pin.get_offset())
            .write(true);

        port_regs
            .direction
            .get_bitband(self.pin.get_offset())
            .write(false);

        port_regs
            .output
            .get_bitband(self.pin.get_offset())
            .write(false);

        GpioPin {
            _config: GpioIn {
                _input_mode: PullDown,
            },

            pin: self.pin,
        }
    }

    /// Convert this instance into a output pin in push-pull configuration.
    ///
    /// # Returns
    /// A GPIO Pin instance configured in push-pull output mode.
    pub fn to_output_pushpull(self) -> GpioPin<Pin, GpioOut<PushPull>> {
        let port_regs = get_gpio_port(self.pin.get_port_name());

        port_regs
            .output
            .get_bitband(self.pin.get_offset())
            .write(false);

        port_regs
            .direction
            .get_bitband(self.pin.get_offset())
            .write(true);

        GpioPin {
            _config: GpioOut {
                _output_mode: PushPull,
            },

            pin: self.pin,
        }
    }

    /// Convert this instance into a output pin in open collector configuration.
    ///
    /// # Returns
    /// A GPIO Pin instance configured in open collector output mode.
    pub fn to_output_opencollector(self) -> GpioPin<Pin, GpioOut<OpenCollector>> {
        let port_regs = get_gpio_port(self.pin.get_port_name());

        port_regs
            .output
            .get_bitband(self.pin.get_offset())
            .write(false);

        port_regs
            .direction
            .get_bitband(self.pin.get_offset())
            .write(true);

        port_regs
            .resistor_enable
            .get_bitband(self.pin.get_offset())
            .write(true);

        GpioPin {
            _config: GpioOut {
                _output_mode: OpenCollector,
            },

            pin: self.pin,
        }
    }

    /// Break down the GPIO Pin back to its original Pin structure.
    ///
    /// # Returns
    /// The Pin structure contained by the GPIO Pin.
    pub fn extract_pin(self) -> Pin {
        self.to_input_highz().pin
    }
}

impl<Pin: PinX, InputMode: GpioInputMode> GpioPinInput for GpioPin<Pin, GpioIn<InputMode>> {
    /// Reads the value of the GPIO pin.
    ///
    /// # Returns
    /// `true` if pin is high.
    /// `false` if pin is low.
    fn read(&self) -> bool {
        let port_regs = get_gpio_port(self.pin.get_port_name());
        port_regs.input.get_bitband(self.pin.get_offset()).read()
    }
}

impl<Pin: PinX, OutputMode: GpioOutputMode> GpioPinInput for GpioPin<Pin, GpioOut<OutputMode>> {
    /// Reads the value of the GPIO pin.
    ///
    /// # Returns
    /// `true` if pin is high.
    /// `false` if pinis low.
    fn read(&self) -> bool {
        let port_regs = get_gpio_port(self.pin.get_port_name());
        port_regs.input.get_bitband(self.pin.get_offset()).read()
    }
}

impl<Pin: PinX> GpioPinOutput for GpioPin<Pin, GpioOut<PushPull>> {
    /// Sets the GPIO Pin high.
    fn set(&mut self) {
        let port_regs = get_gpio_port(self.pin.get_port_name());

        port_regs
            .output
            .get_bitband(self.pin.get_offset())
            .write(true);
    }

    /// Sets the GPIO Pin low.
    fn clear(&mut self) {
        let port_regs = get_gpio_port(self.pin.get_port_name());

        port_regs
            .output
            .get_bitband(self.pin.get_offset())
            .write(false);
    }

    /// Toggles the GPIO Pin.
    fn toggle(&mut self) {
        let port_regs = get_gpio_port(self.pin.get_port_name());

        port_regs
            .output
            .get_bitband(self.pin.get_offset())
            .modify(|value| !value);
    }
}

impl<Pin: PinX> GpioPinOutput for GpioPin<Pin, GpioOut<OpenCollector>> {
    /// Sets the GPIO Pin high.
    fn set(&mut self) {
        let port_regs = get_gpio_port(self.pin.get_port_name());

        port_regs
            .direction
            .get_bitband(self.pin.get_offset())
            .write(false);

        compiler_fence(Ordering::Relaxed);

        port_regs
            .output
            .get_bitband(self.pin.get_offset())
            .write(true);
    }

    /// Sets the GPIO Pin low.
    fn clear(&mut self) {
        let port_regs = get_gpio_port(self.pin.get_port_name());

        port_regs
            .output
            .get_bitband(self.pin.get_offset())
            .write(false);

        compiler_fence(Ordering::Relaxed);

        port_regs
            .direction
            .get_bitband(self.pin.get_offset())
            .write(true);
    }

    /// Toggles the GPIO Pin.
    fn toggle(&mut self) {
        if self.read() {
            self.clear();
        } else {
            self.set();
        }
    }
}

impl<Pin: PinX> GpioPin<Pin, Disabled> {
    /// Allocates a new GPIO configured Pin.
    ///
    /// # Arguments
    /// `pin` - Provides the pin to be configred for GPIO.
    ///
    /// # Returns
    /// A GPIO Pin in the `Disabled` configuration.
    pub fn new(pin: Pin) -> Self {
        Self {
            _config: Disabled,
            pin: pin,
        }
    }
}

//
// Crate functions
//

/// Configures a pin to a given mode.
///
/// # Arguments
/// `pin` - Provides the pin to configure
/// `desired_mode` - Provides the desired mode of the pin.
pub(crate) fn set_pin_function<Pin: PinIdWithMode>(pin: Pin, desired_mode: PinMode) {
    let port = get_gpio_port(pin.get_port_name());

    let select_status = (desired_mode as usize) ^ (pin.get_mode() as usize);

    match select_status {
        // Toggle Select 0.
        1 => {
            port.select_0
                .get_bitband(pin.get_offset())
                .modify(|value| !value);
        }

        // Toggle Select 1.
        2 => {
            port.select_1
                .get_bitband(pin.get_offset())
                .modify(|value| !value);
        }

        // Use the Select Compliment register to ensure atomic toggling of both Select 0 and 1.
        3 => {
            port.complement_selection
                .get_bitband(pin.get_offset())
                .modify(|value| !value);
        }

        _ => debug_assert_eq!(select_status, 0),
    }
}

//
// For sealed traits.
//

mod private {
    pub trait Sealed {}
}

impl<Pin: PinX, Mode: GpioMode> private::Sealed for GpioPin<Pin, Mode> {}
