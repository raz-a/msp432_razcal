//! # Pin
//! The `pin` module includes structures and functions to utilize GPIO as single independent pins.

//
// TODO: Seal traits.
//

//
// TODO: Get rid of _port_sync_token and get sync internally.
//

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
use core::sync::atomic::{compiler_fence, AtomicU16, Ordering};

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
pub struct GpioPin<Mode: GpioMode> {
    /// The specfic GPIO configuration.
    _config: Mode,

    /// The mask for the pin within the port.
    pin_mask: u16,

    /// The GPIO port registers.
    port_regs: &'static mut GpioPort,
}

/// The following implements state modification for GPIO Pin configurations.
impl<Mode: GpioMode> GpioPin<Mode> {
    /// Convert this instance into a high-impedance input pin.
    ///
    /// # Returns
    /// A GPIO Pin instance configured in high-impedance input mode.
    pub fn to_input_highz(self) -> GpioPin<GpioIn<HighImpedance>> {
        let resistor_enable = AtomicU16::from_mut(&mut self.port_regs.resistor_enable);
        let direction = AtomicU16::from_mut(&mut self.port_regs.direction);

        resistor_enable.fetch_and(!self.pin_mask, Ordering::Relaxed);
        direction.fetch_and(!self.pin_mask, Ordering::Relaxed);

        GpioPin {
            _config: GpioIn {
                _input_mode: HighImpedance,
            },

            pin_mask: self.pin_mask,
            port_regs: self.port_regs,
        }
    }

    /// Convert this instance into a input pin with a pull-up resistor.
    ///
    /// # Returns
    /// A GPIO Pin instance configured in pull-up input mode.
    pub fn to_input_pullup(self) -> GpioPin<GpioIn<PullUp>> {
        let resistor_enable = AtomicU16::from_mut(&mut self.port_regs.resistor_enable);
        let direction = AtomicU16::from_mut(&mut self.port_regs.direction);
        let output = AtomicU16::from_mut(&mut self.port_regs.output);

        resistor_enable.fetch_or(self.pin_mask, Ordering::Relaxed);
        direction.fetch_and(!self.pin_mask, Ordering::Relaxed);
        output.fetch_or(self.pin_mask, Ordering::Relaxed);

        GpioPin {
            _config: GpioIn {
                _input_mode: PullUp,
            },

            pin_mask: self.pin_mask,
            port_regs: self.port_regs,
        }
    }

    /// Convert this instance into a input pin with a pull-down resistor.
    ///
    /// # Returns
    /// A GPIO Pin instance configured in pull-down input mode.
    pub fn to_input_pulldown(self) -> GpioPin<GpioIn<PullDown>> {
        let resistor_enable = AtomicU16::from_mut(&mut self.port_regs.resistor_enable);
        let direction = AtomicU16::from_mut(&mut self.port_regs.direction);
        let output = AtomicU16::from_mut(&mut self.port_regs.output);

        resistor_enable.fetch_or(self.pin_mask, Ordering::Relaxed);
        direction.fetch_and(!self.pin_mask, Ordering::Relaxed);
        output.fetch_and(!self.pin_mask, Ordering::Relaxed);

        GpioPin {
            _config: GpioIn {
                _input_mode: PullDown,
            },

            pin_mask: self.pin_mask,
            port_regs: self.port_regs,
        }
    }

    /// Convert this instance into a output pin in push-pull configuration.
    ///
    /// # Returns
    /// A GPIO Pin instance configured in push-pull output mode.
    pub fn to_output_pushpull(self) -> GpioPin<GpioOut<PushPull>> {
        let direction = AtomicU16::from_mut(&mut self.port_regs.direction);
        let output = AtomicU16::from_mut(&mut self.port_regs.output);

        output.fetch_and(!self.pin_mask, Ordering::Relaxed);
        direction.fetch_or(self.pin_mask, Ordering::Relaxed);

        GpioPin {
            _config: GpioOut {
                _output_mode: PushPull,
            },

            pin_mask: self.pin_mask,
            port_regs: self.port_regs,
        }
    }

    /// Convert this instance into a output pin in open collector configuration.
    ///
    /// # Returns
    /// A GPIO Pin instance configured in open collector output mode.
    pub fn to_output_opencollector(self) -> GpioPin<GpioOut<OpenCollector>> {
        let resistor_enable = AtomicU16::from_mut(&mut self.port_regs.resistor_enable);
        let direction = AtomicU16::from_mut(&mut self.port_regs.direction);
        let output = AtomicU16::from_mut(&mut self.port_regs.output);

        output.fetch_and(!self.pin_mask, Ordering::Relaxed);
        direction.fetch_or(self.pin_mask, Ordering::Relaxed);
        resistor_enable.fetch_or(self.pin_mask, Ordering::Relaxed);

        GpioPin {
            _config: GpioOut {
                _output_mode: OpenCollector,
            },

            pin_mask: self.pin_mask,
            port_regs: self.port_regs,
        }
    }
}

impl<InputMode: GpioInputMode> GpioPinInput for GpioPin<GpioIn<InputMode>> {
    /// Reads the value of the GPIO pin.
    ///
    /// # Returns
    /// `true` if pin is high.
    /// `false` if pin is low.
    fn read(&self) -> bool {
        (self.port_regs.input & self.pin_mask) != 0
    }
}

impl<OutputMode: GpioOutputMode> GpioPinInput for GpioPin<GpioOut<OutputMode>> {
    /// Reads the value of the GPIO pin.
    ///
    /// # Returns
    /// `true` if pin is high.
    /// `false` if pinis low.
    fn read(&self) -> bool {
        (self.port_regs.input & self.pin_mask) != 0
    }
}

impl GpioPinOutput for GpioPin<GpioOut<PushPull>> {
    /// Sets the GPIO Pin high.
    fn set(&mut self) {
        let output = AtomicU16::from_mut(&mut self.port_regs.output);
        output.fetch_or(self.pin_mask, Ordering::Relaxed);
    }

    /// Sets the GPIO Pin low.
    fn clear(&mut self) {
        let output = AtomicU16::from_mut(&mut self.port_regs.output);
        output.fetch_and(!self.pin_mask, Ordering::Relaxed);
    }

    /// Toggles the GPIO Pin.
    fn toggle(&mut self) {
        let output = AtomicU16::from_mut(&mut self.port_regs.output);
        output.fetch_xor(self.pin_mask, Ordering::Relaxed);
    }
}

impl GpioPinOutput for GpioPin<GpioOut<OpenCollector>> {
    /// Sets the GPIO Pin high.
    fn set(&mut self) {
        let direction = AtomicU16::from_mut(&mut self.port_regs.direction);
        let output = AtomicU16::from_mut(&mut self.port_regs.output);

        direction.fetch_and(!self.pin_mask, Ordering::Relaxed);
        compiler_fence(Ordering::SeqCst);
        output.fetch_or(self.pin_mask, Ordering::Relaxed);
    }

    /// Sets the GPIO Pin low.
    fn clear(&mut self) {
        let direction = AtomicU16::from_mut(&mut self.port_regs.direction);
        let output = AtomicU16::from_mut(&mut self.port_regs.output);

        output.fetch_and(!self.pin_mask, Ordering::Relaxed);
        compiler_fence(Ordering::SeqCst);
        direction.fetch_or(self.pin_mask, Ordering::Relaxed);
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
pub fn gpio_pin_new<PinId: IdentifiablePin>(pin: PinId) -> GpioPin<Disabled> {
    let addr = get_port_address(pin.get_port_name());
    let port = unsafe { &mut *(addr as *mut GpioPort) };
    let pin_offset = pin.get_offset();

    set_pin_function_to_gpio(port, pin_offset);
    let gpio_pin = GpioPin {
        _config: Disabled,
        pin_mask: 1 << pin_offset,
        port_regs: port,
    };

    gpio_pin
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
    let pin_mask = 1u16 << pin_offset;
    let sel0 = AtomicU16::from_mut(&mut port.select_0);
    let sel1 = AtomicU16::from_mut(&mut port.select_1);

    let mut select_status = 0u16;
    if (sel0.load(Ordering::Relaxed) & pin_mask) != 0 {
        select_status |= 1;
    }

    if (sel1.load(Ordering::Relaxed) & pin_mask) != 0 {
        select_status |= 2;
    }

    match select_status {
        // Clear Select 0.
        1 => {
            sel0.fetch_and(!pin_mask, Ordering::Relaxed);
        }

        // Clear Select 1.
        2 => {
            sel1.fetch_and(!pin_mask, Ordering::Relaxed);
        }

        // Use the Select Compliment register to ensure atomic clearing of both Select 0 and 1.
        3 => {
            let selc = AtomicU16::from_mut(&mut port.complement_selection);
            selc.fetch_or(pin_mask, Ordering::Relaxed);
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

impl<Mode: GpioMode> private::Sealed for GpioPin<Mode> {}
