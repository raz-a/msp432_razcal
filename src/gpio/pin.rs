//! # Pin
//! The `pin` module includes structures and functions to utilize GPIO as single independent pins.

//
// TODO: Sync problems to resolve.
// 1. Transformation functions. What should happen if there is sync error? GpioPin is consumed...
//      - Should the error return itself?
//
// 2. Write functions. We want a compile time guarantee (for the safe functions) that these functions
//      can only be called if ITS port lock is held.
//
//      - Create "synced" versions of each type. These synced version can be held in a GpioPortInUseToken.
//                ^ Trait = GpioSyncedComponent
//      - Remove GpioPortSync.
//      - Create global functions that take in GpioComponent <new trait> types and will return a set
//        of GpioSyncedComponents. These components have functions that are allowed to be called only when
//        synced.
//
//      OR - Just [debug only] runtime check that the correct token is being used?

//
// TODO: Interrupts for Inputs
//

//
// TODO: Drive strength for Outputs
//

//
// Dependencies
//

use crate::gpio::*;
use crate::pin::IdentifiablePin;
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
pub struct GpioPin<Pin: IdentifiablePin, Mode: GpioMode> {
    /// The specfic GPIO configuration.
    _config: Mode,

    /// The actual pin.
    pin: Pin,
}

/// The following implements state modification for GPIO Pin configurations.
impl<Pin: IdentifiablePin, Mode: GpioMode> GpioPin<Pin, Mode> {
    /// Convert this instance into a high-impedance input pin.
    ///
    /// # Returns
    /// A GPIO Pin instance configured in high-impedance input mode.
    pub fn to_input_highz(self) -> GpioPin<Pin, GpioIn<HighImpedance>> {
        let port_regs = get_gpio_port(self.pin.get_port_name());
        let pin_mask = 1 << self.pin.get_offset();

        port_regs
            .resistor_enable
            .fetch_and(!pin_mask, Ordering::Relaxed);
        port_regs.direction.fetch_and(!pin_mask, Ordering::Relaxed);

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
        let pin_mask = 1 << self.pin.get_offset();

        port_regs
            .resistor_enable
            .fetch_or(pin_mask, Ordering::Relaxed);
        port_regs.direction.fetch_and(!pin_mask, Ordering::Relaxed);
        port_regs.output.fetch_or(pin_mask, Ordering::Relaxed);

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
        let pin_mask = 1 << self.pin.get_offset();

        port_regs
            .resistor_enable
            .fetch_or(pin_mask, Ordering::Relaxed);
        port_regs.direction.fetch_and(!pin_mask, Ordering::Relaxed);
        port_regs.output.fetch_and(!pin_mask, Ordering::Relaxed);

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
        let pin_mask = 1 << self.pin.get_offset();

        port_regs.output.fetch_and(!pin_mask, Ordering::Relaxed);
        port_regs.direction.fetch_or(pin_mask, Ordering::Relaxed);

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
        let pin_mask = 1 << self.pin.get_offset();

        port_regs.output.fetch_and(!pin_mask, Ordering::Relaxed);
        port_regs.direction.fetch_or(pin_mask, Ordering::Relaxed);
        port_regs
            .resistor_enable
            .fetch_or(pin_mask, Ordering::Relaxed);

        GpioPin {
            _config: GpioOut {
                _output_mode: OpenCollector,
            },

            pin: self.pin,
        }
    }
}

impl<Pin: IdentifiablePin, InputMode: GpioInputMode> GpioPinInput
    for GpioPin<Pin, GpioIn<InputMode>>
{
    /// Reads the value of the GPIO pin.
    ///
    /// # Returns
    /// `true` if pin is high.
    /// `false` if pin is low.
    fn read(&self) -> bool {
        let port_regs = get_gpio_port(self.pin.get_port_name());
        let pin_mask = 1 << self.pin.get_offset();

        (port_regs.input & pin_mask) != 0
    }
}

impl<Pin: IdentifiablePin, OutputMode: GpioOutputMode> GpioPinInput
    for GpioPin<Pin, GpioOut<OutputMode>>
{
    /// Reads the value of the GPIO pin.
    ///
    /// # Returns
    /// `true` if pin is high.
    /// `false` if pinis low.
    fn read(&self) -> bool {
        let port_regs = get_gpio_port(self.pin.get_port_name());
        let pin_mask = 1 << self.pin.get_offset();

        (port_regs.input & pin_mask) != 0
    }
}

impl<Pin: IdentifiablePin> GpioPinOutput for GpioPin<Pin, GpioOut<PushPull>> {
    /// Sets the GPIO Pin high.
    fn set(&mut self) {
        let port_regs = get_gpio_port(self.pin.get_port_name());
        let pin_mask = 1 << self.pin.get_offset();

        port_regs.output.fetch_or(pin_mask, Ordering::Relaxed);
    }

    /// Sets the GPIO Pin low.
    fn clear(&mut self) {
        let port_regs = get_gpio_port(self.pin.get_port_name());
        let pin_mask = 1 << self.pin.get_offset();

        port_regs.output.fetch_and(!pin_mask, Ordering::Relaxed);
    }

    /// Toggles the GPIO Pin.
    fn toggle(&mut self) {
        let port_regs = get_gpio_port(self.pin.get_port_name());
        let pin_mask = 1 << self.pin.get_offset();

        port_regs.output.fetch_xor(pin_mask, Ordering::Relaxed);
    }
}

impl<Pin: IdentifiablePin> GpioPinOutput for GpioPin<Pin, GpioOut<OpenCollector>> {
    /// Sets the GPIO Pin high.
    fn set(&mut self) {
        let port_regs = get_gpio_port(self.pin.get_port_name());
        let pin_mask = 1 << self.pin.get_offset();

        port_regs.direction.fetch_and(!pin_mask, Ordering::Relaxed);
        compiler_fence(Ordering::Release);
        port_regs.output.fetch_or(pin_mask, Ordering::Relaxed);
    }

    /// Sets the GPIO Pin low.
    fn clear(&mut self) {
        let port_regs = get_gpio_port(self.pin.get_port_name());
        let pin_mask = 1 << self.pin.get_offset();

        port_regs.output.fetch_and(!pin_mask, Ordering::Relaxed);
        compiler_fence(Ordering::Release);
        port_regs.direction.fetch_or(pin_mask, Ordering::Relaxed);
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

//
// Public functions
//

/// Allocates a new GPIO configured Pin.
///
/// # Arguments
/// `pin` - Provides the pin to be configred for GPIO.
///
/// # Returns
/// A GPIO Pin in the `Disabled` configuration.
pub fn gpio_pin_new<Pin: IdentifiablePin>(pin: Pin) -> GpioPin<Pin, Disabled> {
    set_pin_function_to_gpio(get_gpio_port(pin.get_port_name()), pin.get_offset());

    GpioPin {
        _config: Disabled,
        pin: pin,
    }
}

//
// Private functions
//

/// Configures a pin to GPIO mode.
///
/// # Arguments
/// `port` - Provides the GPIO Port registers to configure the pins.
/// `pin_offset` - Provides the offset in the port for the pin to configure.
fn set_pin_function_to_gpio(port: &mut GpioPort, pin_offset: u8) {
    // Set function select bits to 00 (GPIO).
    let pin_mask = 1 << pin_offset;
    let mut select_status = 0u16;
    if (port.select_0.load(Ordering::Relaxed) & pin_mask) != 0 {
        select_status |= 1;
    }

    if (port.select_1.load(Ordering::Relaxed) & pin_mask) != 0 {
        select_status |= 2;
    }

    match select_status {
        // Clear Select 0.
        1 => {
            port.select_0.fetch_and(!pin_mask, Ordering::Relaxed);
        }

        // Clear Select 1.
        2 => {
            port.select_1.fetch_and(!pin_mask, Ordering::Relaxed);
        }

        // Use the Select Compliment register to ensure atomic clearing of both Select 0 and 1.
        3 => {
            port.complement_selection
                .fetch_or(pin_mask, Ordering::Relaxed);
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

impl<Pin: IdentifiablePin, Mode: GpioMode> private::Sealed for GpioPin<Pin, Mode> {}
