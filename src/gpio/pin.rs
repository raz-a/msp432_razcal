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

use crate::gpio::*;
use crate::peripheral_to_alias;
use crate::pin::Pin;
use core::sync::atomic::{compiler_fence, Ordering};

//
// Traits
//

/// A GPIO Pin instance that is configured as an input.
pub trait GpioPinInput {
    /// Reads the value of the GPIO pin.
    ///
    /// # Returns
    /// `true` if pin is high.
    /// `false` if pin is low.
    fn read(&self) -> bool;
}

/// A GPIO Pin instance that is configred as an output.
pub trait GpioPinOutput {
    /// Sets the GPIO Pin high.
    ///
    /// # Arguments
    /// `_port_sync_token` - Indicates that no other thread can access the 8-bit GPIO port that
    /// this pin belongs to.
    fn set(&mut self, _port_sync_token: &mut GpioPortInUseToken);

    /// Sets the GPIO Pin low.
    ///
    /// # Arguments
    /// `_port_sync_token` - Indicates that no other thread can access the 8-bit GPIO port that
    /// this pin belongs to.
    fn clear(&mut self, _port_sync_token: &mut GpioPortInUseToken);

    /// Toggles the GPIO Pin.
    ///
    /// # Arguments
    /// `_port_sync_token` - Indicates that no other thread can access the 8-bit GPIO port that
    /// this pin belongs to.
    fn toggle(&mut self, _port_sync_token: &mut GpioPortInUseToken);

    /// Sets the GPIO Pin high.
    ///
    /// # Unsafe
    /// This function is safe to use only if there are no active GPIO buses that have pins in the
    /// same 8-bit port as this pin.
    unsafe fn set_no_sync(&mut self);

    /// Sets the GPIO Pin low.
    ///
    /// # Unsafe
    /// This function is safe to use only if there are no active GPIO buses that have pins in the
    /// same 8-bit port as this pin.
    unsafe fn clear_no_sync(&mut self);

    /// Toggles the GPIO Pin.
    ///
    /// # Unsafe
    /// This function is safe to use only if there are no active GPIO buses that have pins in the
    /// same 8-bit port as this pin.
    unsafe fn toggle_no_sync(&mut self);
}

//
// Structures
//

/// Represents a pin configured for GPIO mode.
/// # Type Options
/// `GpioConfig` indicated the specific configuration mode the GPIO pin is in. Can be of type
/// `Disabled`, `GpioInConfig`, or `GpioOutConfig`.
pub struct GpioPin<GpioConfig> {
    /// The specfic GPIO configuration.
    _config: GpioConfig,

    /// Points to the corresponding bit-band alias for the input register.
    input: &'static u16,

    /// Points to the corresponding bit-band alias for the output register.
    output: &'static mut u16,

    /// Points to the corresponding bit-band alias for the direction register.
    direction: &'static mut u16,

    /// Points to the corresponding bit-band alias for the resistor enable register.
    resistor_enable: &'static mut u16,

    /// A bitmask indicating the port that this pin belongs to. Used for synchronizing with other
    /// GPIO components.
    port_in_use_mask: u16,

    /// The name of the pin in use.
    pin: Pin,
}

/// The following implements state modification for GPIO Pin configurations.
impl<GpioConfig> GpioPin<GpioConfig> {
    /// Convert this instance into a high-impedance input pin.
    ///
    /// # Arguments
    /// `_port_sync_token` - Indicates that no other thread can access the 8-bit GPIO port that
    /// this pin belongs to.
    ///
    /// # Returns
    /// A GPIO Pin instance configured in high-impedance input mode.
    pub fn to_input_highz(
        self,
        _port_sync_token: &mut GpioPortInUseToken,
    ) -> GpioPin<GpioInConfig<HighImpedance>> {
        //
        // UNSAFE: _port_sync_token ensures that no other thread can access the 8-bit GPIO port that
        // this pin belongs to.
        //

        unsafe { self.to_input_highz_no_sync() }
    }

    /// Convert this instance into a input pin with a pull-up resistor.
    ///
    /// # Arguments
    /// `_port_sync_token` - Indicates that no other thread can access the 8-bit GPIO port that
    /// this pin belongs to.
    ///
    /// # Returns
    /// A GPIO Pin instance configured in pull-up input mode.
    pub fn to_input_pullup(
        self,
        _port_sync_token: &mut GpioPortInUseToken,
    ) -> GpioPin<GpioInConfig<PullUp>> {
        //
        // UNSAFE: _port_sync_token ensures that no other thread can access the 8-bit GPIO port that
        // this pin belongs to.
        //

        unsafe { self.to_input_pullup_no_sync() }
    }

    /// Convert this instance into a input pin with a pull-down resistor.
    ///
    /// # Arguments
    /// `_port_sync_token` - Indicates that no other thread can access the 8-bit GPIO port that
    /// this pin belongs to.
    ///
    /// # Returns
    /// A GPIO Pin instance configured in pull-down input mode.
    pub fn to_input_pulldown(
        self,
        _port_sync_token: &mut GpioPortInUseToken,
    ) -> GpioPin<GpioInConfig<PullDown>> {
        //
        // UNSAFE: _port_sync_token ensures that no other thread can access the 8-bit GPIO port that
        // this pin belongs to.
        //

        unsafe { self.to_input_pulldown_no_sync() }
    }

    /// Convert this instance into a output pin in push-pull configuration.
    ///
    /// # Arguments
    /// `_port_sync_token` - Indicates that no other thread can access the 8-bit GPIO port that
    /// this pin belongs to.
    ///
    /// # Returns
    /// A GPIO Pin instance configured in push-pull output mode.
    pub fn to_output_pushpull(
        self,
        _port_sync_token: &mut GpioPortInUseToken,
    ) -> GpioPin<GpioOutConfig<PushPull>> {
        //
        // UNSAFE: _port_sync_token ensures that no other thread can access the 8-bit GPIO port that
        // this pin belongs to.
        //

        unsafe { self.to_output_pushpull_no_sync() }
    }

    /// Convert this instance into a output pin in open collector configuration.
    ///
    /// # Arguments
    /// `_port_sync_token` - Indicates that no other thread can access the 8-bit GPIO port that
    /// this pin belongs to.
    ///
    /// # Returns
    /// A GPIO Pin instance configured in open collector output mode.
    pub fn to_output_opencollector(
        self,
        _port_sync_token: &mut GpioPortInUseToken,
    ) -> GpioPin<GpioOutConfig<OpenCollector>> {
        //
        // UNSAFE: _port_sync_token ensures that no other thread can access the 8-bit GPIO port that
        // this pin belongs to.
        //

        unsafe { self.to_output_opencollector_no_sync() }
    }

    /// Convert this instance into a high-impedance input pin.
    ///
    /// # Returns
    /// A GPIO Pin instance configured in high-impedance input mode.
    ///
    /// # Unsafe
    /// Caller must ensure there are no active GPIO buses that have pins in the same 8-bit port as
    /// this pin.
    pub unsafe fn to_input_highz_no_sync(self) -> GpioPin<GpioInConfig<HighImpedance>> {
        *self.resistor_enable = 0;
        *self.direction = 0;
        GpioPin {
            _config: GpioInConfig {
                _input_mode: HighImpedance,
            },

            input: self.input,
            output: self.output,
            direction: self.direction,
            resistor_enable: self.resistor_enable,
            port_in_use_mask: self.port_in_use_mask,
            pin: self.pin,
        }
    }

    /// Convert this instance into a input pin with a pull-up resistor.
    ///
    /// # Returns
    /// A GPIO Pin instance configured in pull-up input mode.
    ///
    /// # Unsafe
    /// Caller must ensure there are no active GPIO buses that have pins in the same 8-bit port as
    /// this pin.
    pub unsafe fn to_input_pullup_no_sync(self) -> GpioPin<GpioInConfig<PullUp>> {
        *self.resistor_enable = 1;
        *self.direction = 0;
        *self.output = 1;
        GpioPin {
            _config: GpioInConfig {
                _input_mode: PullUp,
            },
            input: self.input,
            output: self.output,
            direction: self.direction,
            resistor_enable: self.resistor_enable,
            port_in_use_mask: self.port_in_use_mask,
            pin: self.pin,
        }
    }

    /// Convert this instance into a input pin with a pull-down resistor.
    ///
    /// # Returns
    /// A GPIO Pin instance configured in pull-down input mode.
    ///
    /// # Unsafe
    /// Caller must ensure there are no active GPIO buses that have pins in the same 8-bit port as
    /// this pin.
    pub unsafe fn to_input_pulldown_no_sync(self) -> GpioPin<GpioInConfig<PullDown>> {
        *self.resistor_enable = 1;
        *self.direction = 0;
        *self.output = 0;
        GpioPin {
            _config: GpioInConfig {
                _input_mode: PullDown,
            },
            input: self.input,
            output: self.output,
            direction: self.direction,
            resistor_enable: self.resistor_enable,
            port_in_use_mask: self.port_in_use_mask,
            pin: self.pin,
        }
    }

    /// Convert this instance into a output pin in push-pull configuration.
    ///
    /// # Returns
    /// A GPIO Pin instance configured in push-pull output mode.
    ///
    /// # Unsafe
    /// Caller must ensure there are no active GPIO buses that have pins in the same 8-bit port as
    /// this pin.
    pub unsafe fn to_output_pushpull_no_sync(self) -> GpioPin<GpioOutConfig<PushPull>> {
        *self.output = 0;
        *self.direction = 1;
        GpioPin {
            _config: GpioOutConfig {
                _output_mode: PushPull,
            },
            input: self.input,
            output: self.output,
            direction: self.direction,
            resistor_enable: self.resistor_enable,
            port_in_use_mask: self.port_in_use_mask,
            pin: self.pin,
        }
    }

    /// Convert this instance into a output pin in open collector configuration.
    ///
    /// # Returns
    /// A GPIO Pin instance configured in open collector output mode.
    ///
    /// # Unsafe
    /// Caller must ensure there are no active GPIO buses that have pins in the same 8-bit port as
    /// this pin.
    pub unsafe fn to_output_opencollector_no_sync(self) -> GpioPin<GpioOutConfig<OpenCollector>> {
        *self.output = 0;
        *self.direction = 1;
        *self.resistor_enable = 1;
        GpioPin {
            _config: GpioOutConfig {
                _output_mode: OpenCollector,
            },
            input: self.input,
            output: self.output,
            direction: self.direction,
            resistor_enable: self.resistor_enable,
            port_in_use_mask: self.port_in_use_mask,
            pin: self.pin,
        }
    }
}

impl<GpioConfig> GpioPortSync for GpioPin<GpioConfig> {
    /// Attempts to obtain a GpioPortInUseToken. This function will not succeed nested calls.
    ///
    /// # Returns
    /// `Some(GpioPortInUseToken)` if GPIO port was free and obtained by the caller.
    /// `None` if GPIO port was in use.
    fn get_port_in_use_token(&self) -> Option<GpioPortInUseToken> {
        let previous_value =
            unsafe { GPIO_PORT_IN_USE_LOCK.fetch_or(self.port_in_use_mask, Ordering::Relaxed) };

        if previous_value & self.port_in_use_mask != 0 {
            return None;
        }

        Some(GpioPortInUseToken {
            free_mask: self.port_in_use_mask,
        })
    }
}

impl<InputMode> GpioPinInput for GpioPin<GpioInConfig<InputMode>> {
    /// Reads the value of the GPIO pin.
    ///
    /// # Returns
    /// `true` if pin is high.
    /// `false` if pinis low.
    fn read(&self) -> bool {
        *self.input != 0
    }
}

impl<OutputMode> GpioPinInput for GpioPin<GpioOutConfig<OutputMode>> {
    /// Reads the value of the GPIO pin.
    ///
    /// # Returns
    /// `true` if pin is high.
    /// `false` if pinis low.
    fn read(&self) -> bool {
        *self.input != 0
    }
}

impl GpioPinOutput for GpioPin<GpioOutConfig<PushPull>> {
    /// Sets the GPIO Pin high.
    ///
    /// # Arguments
    /// `_port_sync_token` - Indicates that no other thread can access the 8-bit GPIO port that
    /// this pin belongs to.
    fn set(&mut self, _port_sync_token: &mut GpioPortInUseToken) {
        //
        // UNSAFE: _port_sync_token ensures that no other thread can access the 8-bit GPIO port that
        // this pin belongs to.
        //

        unsafe { self.set_no_sync() };
    }

    /// Sets the GPIO Pin low.
    ///
    /// # Arguments
    /// `_port_sync_token` - Indicates that no other thread can access the 8-bit GPIO port that
    /// this pin belongs to.
    fn clear(&mut self, _port_sync_token: &mut GpioPortInUseToken) {
        //
        // UNSAFE: _port_sync_token ensures that no other thread can access the 8-bit GPIO port that
        // this pin belongs to.
        //

        unsafe { self.clear_no_sync() };
    }

    /// Toggles the GPIO Pin.
    /// # Arguments
    ///
    /// `_port_sync_token` - Indicates that no other thread can access the 8-bit GPIO port that
    /// this pin belongs to.
    fn toggle(&mut self, _port_sync_token: &mut GpioPortInUseToken) {
        //
        // UNSAFE: _port_sync_token ensures that no other thread can access the 8-bit GPIO port that
        // this pin belongs to.
        //

        unsafe { self.toggle_no_sync() };
    }

    /// Sets the GPIO Pin high.
    ///
    /// # Unsafe
    /// This function is safe to use only if there are no active GPIO buses that have pins in the
    /// same 8-bit port as this pin.
    unsafe fn set_no_sync(&mut self) {
        *self.output = 1;
    }

    /// Sets the GPIO Pin low.
    ///
    /// # Unsafe
    /// This function is safe to use only if there are no active GPIO buses that have pins in the
    /// same 8-bit port as this pin.
    unsafe fn clear_no_sync(&mut self) {
        *self.output = 0;
    }

    /// Toggles the GPIO Pin.
    ///
    /// # Unsafe
    /// This function is safe to use only if there are no active GPIO buses that have pins in the
    /// same 8-bit port as this pin.
    unsafe fn toggle_no_sync(&mut self) {
        *self.output ^= 1;
    }
}

impl GpioPinOutput for GpioPin<GpioOutConfig<OpenCollector>> {
    /// Sets the GPIO Pin high.
    ///
    /// # Arguments
    /// `_port_sync_token` - Indicates that no other thread can access the 8-bit GPIO port that
    /// this pin belongs to.
    fn set(&mut self, _port_sync_token: &mut GpioPortInUseToken) {
        //
        // UNSAFE: _port_sync_token ensures that no other thread can access the 8-bit GPIO port that
        // this pin belongs to.
        //

        unsafe { self.set_no_sync() };
    }

    /// Sets the GPIO Pin low.
    ///
    /// # Arguments
    /// `_port_sync_token` - Indicates that no other thread can access the 8-bit GPIO port that
    /// this pin belongs to.
    fn clear(&mut self, _port_sync_token: &mut GpioPortInUseToken) {
        //
        // UNSAFE: _port_sync_token ensures that no other thread can access the 8-bit GPIO port that
        // this pin belongs to.
        //

        unsafe { self.clear_no_sync() };
    }

    /// Toggles the GPIO Pin.
    /// # Arguments
    ///
    /// `_port_sync_token` - Indicates that no other thread can access the 8-bit GPIO port that
    /// this pin belongs to.
    fn toggle(&mut self, _port_sync_token: &mut GpioPortInUseToken) {
        //
        // UNSAFE: _port_sync_token ensures that no other thread can access the 8-bit GPIO port that
        // this pin belongs to.
        //

        unsafe { self.toggle_no_sync() };
    }

    /// Sets the GPIO Pin high.
    ///
    /// # Unsafe
    /// This function is safe to use only if there are no active GPIO buses that have pins in the
    /// same 8-bit port as this pin.
    unsafe fn set_no_sync(&mut self) {
        *self.direction = 0;
        compiler_fence(Ordering::SeqCst);
        *self.output = 1;
    }

    /// Sets the GPIO Pin low.
    ///
    /// # Unsafe
    /// This function is safe to use only if there are no active GPIO buses that have pins in the
    /// same 8-bit port as this pin.
    unsafe fn clear_no_sync(&mut self) {
        *self.output = 0;
        compiler_fence(Ordering::SeqCst);
        *self.direction = 1;
    }

    /// Toggles the GPIO Pin.
    ///
    /// # Unsafe
    /// This function is safe to use only if there are no active GPIO buses that have pins in the
    /// same 8-bit port as this pin.
    unsafe fn toggle_no_sync(&mut self) {
        if *self.input == 0 {
            self.set_no_sync();
        } else {
            self.clear_no_sync();
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
pub fn gpio_pin_new(pin: Pin) -> GpioPin<Disabled> {
    let pin_offset = pin.get_pin_offset_in_port() as u8;
    let addr = get_port_address(pin.get_port());
    let port = unsafe { &mut *(addr as *mut GpioPort) };

    set_pin_function_to_gpio(port, pin_offset);
    let input_addr = peripheral_to_alias(port.input.get_halfword_ptr() as u32, pin_offset);
    let output_addr = peripheral_to_alias(port.output.get_halfword_ptr_mut() as u32, pin_offset);
    let dir_addr = peripheral_to_alias(port.direction.get_halfword_ptr_mut() as u32, pin_offset);
    let res_addr = peripheral_to_alias(
        port.resistor_enable.get_halfword_ptr_mut() as u32,
        pin_offset,
    );

    let in_use_shift = pin.get_port().get_16_bit_port_index() as u8 * 2;
    let in_use_mask = if pin_offset > 7 { 0x2 } else { 0x1 };

    let in_use = in_use_mask << in_use_shift;
    let gpio_pin = unsafe {
        GpioPin {
            _config: Disabled,
            input: &*(input_addr as *const u16),
            output: &mut *(output_addr as *mut u16),
            direction: &mut *(dir_addr as *mut u16),
            resistor_enable: &mut *(res_addr as *mut u16),
            port_in_use_mask: in_use,
            pin: pin,
        }
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
    let sel0_addr = peripheral_to_alias(port.select_0.get_halfword_ptr_mut() as u32, pin_offset);

    let sel0_reg = unsafe { &mut *(sel0_addr as *mut u16) };

    let sel1_addr = peripheral_to_alias(port.select_0.get_halfword_ptr_mut() as u32, pin_offset);

    let sel1_reg = unsafe { &mut *(sel1_addr as *mut u16) };

    let select_status = (*sel1_reg << 1) | *sel0_reg;
    match select_status {
        // Clear Select 0.
        1 => *sel0_reg = 0,

        // Clear Select 1.
        2 => *sel1_reg = 0,

        // Use the Select Compliment register to ensure atomic clearing of both Select 0 and 1.
        3 => {
            let selc_addr = peripheral_to_alias(
                port.complement_selection.get_halfword_ptr_mut() as u32,
                pin_offset,
            );

            let selc_reg = unsafe { &mut *(selc_addr as *mut u16) };
            *selc_reg = 1;
        }

        _ => debug_assert_eq!(select_status, 0),
    }
}
