//! # PortBus
//! The `portbus` module includes structures and functions to utilize a port as a GPIO bus.

//
// Dependencies
//

use super::{GpioBusInput, GpioBusOutput};
use crate::gpio::{
    GpioInConfig, GpioOutConfig, GpioPort, GpioPortInUseToken, GpioPortSync, HighImpedance,
    OpenCollector, PullDown, PullUp, PushPull,
};
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

    /// Convert this port into an input bus with pull-up resistors.
    ///
    /// # Returns
    /// A GPIO Port Bus instance configured in input mode with pull-up resistors.
    pub fn to_input_pullup(mut self) -> GpioPortBus<GpioInConfig<PullUp>> {
        self.port_regs.resistor_enable.set_halfword(ALL_PINS_MASK);
        self.port_regs.direction.set_halfword(0);
        self.port_regs.output.set_halfword(ALL_PINS_MASK);
        GpioPortBus {
            _config: GpioInConfig {
                _input_mode: PullUp,
            },

            port_regs: self.port_regs,
            port: self.port,
        }
    }

    /// Convert this port into an input bus with pull-down resistors.
    ///
    /// # Returns
    /// A GPIO Port Bus instance configured in input mode with pull-down resistors.
    pub fn to_input_pulldown(mut self) -> GpioPortBus<GpioInConfig<PullDown>> {
        self.port_regs.resistor_enable.set_halfword(ALL_PINS_MASK);
        self.port_regs.direction.set_halfword(0);
        self.port_regs.output.set_halfword(0);
        GpioPortBus {
            _config: GpioInConfig {
                _input_mode: PullDown,
            },

            port_regs: self.port_regs,
            port: self.port,
        }
    }

    /// Convert this port into an output bus with push-pull configuration.
    ///
    /// # Returns
    /// A GPIO Port Bus instance configured in output mode with push-pull configuration.
    pub fn to_output_pushpull(mut self) -> GpioPortBus<GpioOutConfig<PushPull>> {
        self.port_regs.output.set_halfword(0);
        self.port_regs.direction.set_halfword(ALL_PINS_MASK);
        GpioPortBus {
            _config: GpioOutConfig {
                _output_mode: PushPull,
            },

            port_regs: self.port_regs,
            port: self.port,
        }
    }

    /// Convert this port into an output bus with open collector configuration.
    ///
    /// # Returns
    /// A GPIO Port Bus instance configured in output mode with open collector configuration.
    pub fn to_output_opencollector(mut self) -> GpioPortBus<GpioOutConfig<OpenCollector>> {
        self.port_regs.output.set_halfword(0);
        self.port_regs.direction.set_halfword(ALL_PINS_MASK);
        self.port_regs.resistor_enable.set_halfword(ALL_PINS_MASK);
        GpioPortBus {
            _config: GpioOutConfig {
                _output_mode: OpenCollector,
            },

            port_regs: self.port_regs,
            port: self.port,
        }
    }
}

impl<GpioConfig> GpioPortSync for GpioPortBus<GpioConfig> {
    /// Attempts to obtain a GpioPortInUseToken. Because this bug owns the entire port, this is
    /// effectively a no-op.
    ///
    /// # Returns
    /// `Some(GpioPortInUseToken)`
    fn get_port_in_use_token(&self) -> Option<GpioPortInUseToken> {
        // Set the free mask to 0 so no bits will be changed when the token is dropped.
        Some(GpioPortInUseToken { free_mask: 0 })
    }
}

impl<InputMode> GpioBusInput for GpioPortBus<GpioInConfig<InputMode>> {
    /// Reads the value of the GPIO Bus.
    ///
    /// # Returns
    /// Value of the GPIO Bus.
    fn read(&self) -> usize {
        self.port_regs.input.get_halfword() as usize
    }
}

impl<OutputMode> GpioBusInput for GpioPortBus<GpioOutConfig<OutputMode>> {
    /// Reads the value of the GPIO Bus.
    ///
    /// # Returns
    /// Value of the GPIO Bus.
    fn read(&self) -> usize {
        self.port_regs.input.get_halfword() as usize
    }
}

impl GpioBusOutput for GpioPortBus<GpioOutConfig<PushPull>> {
    /// Sets the value of the GPIO Bus.
    ///
    /// # Arguments
    /// `value` - The value to write to the bus.
    /// `_port_sync_token` - Indicates that no other thread can access the GPIO port(s) this bus
    /// belongs to.
    fn write(&mut self, value: usize, _port_sync_token: &mut GpioPortInUseToken) {
        unsafe { self.write_no_sync(value) };
    }

    /// Sets bits on the GPIO Bus.
    ///
    /// # Arguments
    /// `set_mask` - The bits to set on the bus.
    /// `_port_sync_token` - Indicates that no other thread can access the GPIO port(s) this bus
    /// belongs to.
    fn set_bits(&mut self, set_mask: usize, _port_sync_token: &mut GpioPortInUseToken) {
        unsafe { self.set_bits_no_sync(set_mask) };
    }

    /// Clears bits on the GPIO Bus.
    ///
    /// # Arguments
    /// `clear_mask` - The bits to clear on the bus.
    /// `_port_sync_token` - Indicates that no other thread can access the GPIO port(s) this bus
    /// belongs to.
    fn clear_bits(&mut self, clear_mask: usize, _port_sync_token: &mut GpioPortInUseToken) {
        unsafe { self.clear_bits_no_sync(clear_mask) };
    }

    /// Toggles bits on the GPIO Bus.
    ///
    /// # Arguments
    /// `toggle_mask` - The bits to toggle on the bus.
    /// `_port_sync_token` - Indicates that no other thread can access the GPIO port(s) this bus
    /// belongs to.
    fn toggle_bits(&mut self, toggle_mask: usize, _port_sync_token: &mut GpioPortInUseToken) {
        unsafe { self.toggle_bits_no_sync(toggle_mask) };
    }

    /// Sets the value of the GPIO Bus.
    ///
    /// # Arguments
    /// `value` - The value to write to the bus.
    ///
    /// # Unsafe
    /// This function is safe to use only if there are no active GPIO pins or buses that are in the
    /// same port as this bus.
    unsafe fn write_no_sync(&mut self, value: usize) {
        self.port_regs.output.set_halfword(value as u16);
    }

    /// Sets bits on the GPIO Bus.
    ///
    /// # Arguments
    /// `set_mask` - The bits to set on the bus.
    ///
    /// # Unsafe
    /// This function is safe to use only if there are no active GPIO pins or buses that are in the
    /// same port as this bus.
    unsafe fn set_bits_no_sync(&mut self, set_mask: usize) {
        let value = self.port_regs.output.get_halfword() | set_mask as u16;
        self.port_regs.output.set_halfword(value);
    }

    /// Clears bits on the GPIO Bus.
    ///
    /// # Arguments
    /// `clear_mask` - The bits to clear on the bus.
    ///
    /// # Unsafe
    /// This function is safe to use only if there are no active GPIO pins or buses that are in the
    /// same port as this bus.
    unsafe fn clear_bits_no_sync(&mut self, clear_mask: usize) {
        let value = self.port_regs.output.get_halfword() & !clear_mask as u16;
        self.port_regs.output.set_halfword(value);
    }

    /// Toggles bits on the GPIO Bus.
    ///
    /// # Arguments
    /// `toggle_mask` - The bits to toggle on the bus.
    ///
    /// # Unsafe
    /// This function is safe to use only if there are no active GPIO pins or buses that are in the
    /// same port as this bus.
    unsafe fn toggle_bits_no_sync(&mut self, toggle_mask: usize) {
        let value = self.port_regs.output.get_halfword() ^ toggle_mask as u16;
        self.port_regs.output.set_halfword(value);
    }
}

impl GpioBusOutput for GpioPortBus<GpioOutConfig<OpenCollector>> {
    /// Sets the value of the GPIO Bus.
    ///
    /// # Arguments
    /// `value` - The value to write to the bus.
    /// `_port_sync_token` - Indicates that no other thread can access the GPIO port(s) this bus
    /// belongs to.
    fn write(&mut self, value: usize, _port_sync_token: &mut GpioPortInUseToken) {
        unsafe { self.write_no_sync(value) };
    }

    /// Sets bits on the GPIO Bus.
    ///
    /// # Arguments
    /// `set_mask` - The bits to set on the bus.
    /// `_port_sync_token` - Indicates that no other thread can access the GPIO port(s) this bus
    /// belongs to.
    fn set_bits(&mut self, set_mask: usize, _port_sync_token: &mut GpioPortInUseToken) {
        unsafe { self.set_bits_no_sync(set_mask) };
    }

    /// Clears bits on the GPIO Bus.
    ///
    /// # Arguments
    /// `clear_mask` - The bits to clear on the bus.
    /// `_port_sync_token` - Indicates that no other thread can access the GPIO port(s) this bus
    /// belongs to.
    fn clear_bits(&mut self, clear_mask: usize, _port_sync_token: &mut GpioPortInUseToken) {
        unsafe { self.clear_bits_no_sync(clear_mask) };
    }

    /// Toggles bits on the GPIO Bus.
    ///
    /// # Arguments
    /// `toggle_mask` - The bits to toggle on the bus.
    /// `_port_sync_token` - Indicates that no other thread can access the GPIO port(s) this bus
    /// belongs to.
    fn toggle_bits(&mut self, toggle_mask: usize, _port_sync_token: &mut GpioPortInUseToken) {
        unsafe { self.toggle_bits_no_sync(toggle_mask) };
    }

    /// Sets the value of the GPIO Bus.
    ///
    /// # Arguments
    /// `value` - The value to write to the bus.
    ///
    /// # Unsafe
    /// This function is safe to use only if there are no active GPIO pins or buses that are in the
    /// same port as this bus.
    unsafe fn write_no_sync(&mut self, value: usize) {
        todo!();
    }

    /// Sets bits on the GPIO Bus.
    ///
    /// # Arguments
    /// `set_mask` - The bits to set on the bus.
    ///
    /// # Unsafe
    /// This function is safe to use only if there are no active GPIO pins or buses that are in the
    /// same port as this bus.
    unsafe fn set_bits_no_sync(&mut self, set_mask: usize) {
        todo!();
    }

    /// Clears bits on the GPIO Bus.
    ///
    /// # Arguments
    /// `clear_mask` - The bits to clear on the bus.
    ///
    /// # Unsafe
    /// This function is safe to use only if there are no active GPIO pins or buses that are in the
    /// same port as this bus.
    unsafe fn clear_bits_no_sync(&mut self, clear_mask: usize) {
        todo!();
    }

    /// Toggles bits on the GPIO Bus.
    ///
    /// # Arguments
    /// `toggle_mask` - The bits to toggle on the bus.
    ///
    /// # Unsafe
    /// This function is safe to use only if there are no active GPIO pins or buses that are in the
    /// same port as this bus.
    unsafe fn toggle_bits_no_sync(&mut self, toggle_mask: usize) {
        todo!();
    }
}
