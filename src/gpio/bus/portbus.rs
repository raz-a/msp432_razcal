//! # PortBus
//! The `portbus` module includes structures and functions to utilize a port as a GPIO bus.

//
// Dependencies
//

use super::{GpioBusInput, GpioBusOutput};
use crate::gpio::{
    get_port_address, Disabled, GpioInConfig, GpioOutConfig, GpioPort, GpioPortInUseToken,
    GpioPortSync, HighImpedance, OpenCollector, PullDown, PullUp, PushPull, GPIO_PORT_IN_USE_LOCK,
};
use crate::pin::Port;
use core::sync::atomic::{compiler_fence, Ordering};

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
    port_regs: &'static mut GpioPort,

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
    pub fn to_input_highz(self) -> GpioPortBus<GpioInConfig<HighImpedance>> {
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
    pub fn to_input_pullup(self) -> GpioPortBus<GpioInConfig<PullUp>> {
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
    pub fn to_input_pulldown(self) -> GpioPortBus<GpioInConfig<PullDown>> {
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
    pub fn to_output_pushpull(self) -> GpioPortBus<GpioOutConfig<PushPull>> {
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
    pub fn to_output_opencollector(self) -> GpioPortBus<GpioOutConfig<OpenCollector>> {
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
        let current_state = self.port_regs.direction.get_halfword();

        let cleared_bits = current_state & !value as u16;
        self.port_regs.direction.set_halfword(cleared_bits);

        compiler_fence(Ordering::SeqCst);

        self.port_regs.output.set_halfword(value as u16);

        compiler_fence(Ordering::SeqCst);

        let set_bits = cleared_bits | value as u16;
        self.port_regs.direction.set_halfword(set_bits);

        debug_assert_eq!(
            self.port_regs.direction.get_halfword(),
            !self.port_regs.output.get_halfword()
        );
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
        // Set bits as input before changing the output register.
        let dir_value = self.port_regs.direction.get_halfword() & !set_mask as u16;
        self.port_regs.direction.set_halfword(dir_value);

        compiler_fence(Ordering::SeqCst);

        let out_value = self.port_regs.output.get_halfword() | set_mask as u16;
        self.port_regs.output.set_halfword(out_value);

        debug_assert_eq!(
            self.port_regs.direction.get_halfword(),
            !self.port_regs.output.get_halfword()
        );
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
        // Set bits as pull-down before changing direction.
        let out_value = self.port_regs.output.get_halfword() & !clear_mask as u16;
        self.port_regs.output.set_halfword(out_value);

        compiler_fence(Ordering::SeqCst);

        let dir_value = self.port_regs.direction.get_halfword() | clear_mask as u16;
        self.port_regs.direction.set_halfword(dir_value);

        debug_assert_eq!(
            self.port_regs.direction.get_halfword(),
            !self.port_regs.output.get_halfword()
        );
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
        self.write_no_sync(value as usize);
    }
}

//
// Public functions.
//

pub fn gpio_port_bus_new(port: Port) -> GpioPortBus<Disabled> {
    let addr = get_port_address(port.get_name());
    let gpio_port = unsafe { &mut *(addr as *mut GpioPort) };

    //
    // No need to mark GPIOPort as "in use" as the corresponding drop logic to ensure this gets
    // undone + the typestate changes would create very unsafe code.
    // This relies on the fact that no other pins on this port could ever be in use during this
    // bus's lifetime as it owns the port structure.
    //

    // Configure pins to GPIO mode.
    let sel0 = gpio_port.select_0.get_halfword();
    let sel1 = gpio_port.select_1.get_halfword();

    // Use the Select Complement reigster for bits with both Select 0 and 1 set.
    let selc = sel0 & sel1;
    gpio_port.complement_selection.set_halfword(selc);

    compiler_fence(Ordering::SeqCst);

    // Clear the appropriate remaing Select bits.
    gpio_port.select_0.set_halfword(0);
    gpio_port.select_1.set_halfword(0);

    GpioPortBus {
        _config: Disabled,
        port: port,
        port_regs: gpio_port,
    }
}
