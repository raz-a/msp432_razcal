use crate::gpio::*;
use crate::peripheral_to_alias;
use crate::pin::Pin;
use core::sync::atomic::Ordering;

pub struct GpioPin<GpioConfig> {
    _config: GpioConfig,
    input: &'static u16,
    output: &'static mut u16,
    direction: &'static mut u16,
    resistor_enable: &'static mut u16,
    port_in_use_mask: u16,
    pin: Pin,
}

pub trait GpioPinInput {
    fn read(&self) -> bool;
}

pub trait GpioPinOutput {
    // Safe write functions.

    fn set(&mut self, _port_sync_token: &mut GpioPortInUseToken);
    fn clear(&mut self, _port_sync_token: &mut GpioPortInUseToken);
    fn toggle(&mut self, _port_sync_token: &mut GpioPortInUseToken);

    // Unsafe write functions. These are safe to use only if there are no active
    // GPIO buses that have pins in the same 8-bit port as this pin.

    unsafe fn set_no_sync(&mut self);
    unsafe fn clear_no_sync(&mut self);
    unsafe fn toggle_no_sync(&mut self);
}

pub fn gpio_pin_new(pin: Pin) -> GpioPin<Disabled> {
    let pin_offset = pin.get_pin_offset_in_port();
    let addr = get_port_address(&pin);
    let port = unsafe { &mut *(addr as *mut GpioPort) };

    set_pin_function_to_gpio(port, pin_offset);
    let input_addr = peripheral_to_alias(port.input.get_halfword_ptr() as u32, pin_offset);
    let output_addr = peripheral_to_alias(port.output.get_halfword_ptr_mut() as u32, pin_offset);

    let dir_addr = peripheral_to_alias(port.direction.get_halfword_ptr_mut() as u32, pin_offset);
    let res_addr = peripheral_to_alias(
        port.resistor_enable.get_halfword_ptr_mut() as u32,
        pin_offset,
    );

    let in_use_shift = pin.get_port() * 2;
    let in_use_mask = if pin.get_pin_offset_in_port() > 7 {
        0x2
    } else {
        0x1
    };

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

impl<GpioConfig> GpioPin<GpioConfig> {
    // Safe state modification functions.

    pub fn to_input_highz(
        self,
        _port_sync_token: &mut GpioPortInUseToken,
    ) -> GpioPin<GpioInConfig<HighImpedance>> {
        unsafe { self.to_input_highz_no_sync() }
    }

    pub fn to_input_pullup(
        self,
        _port_sync_token: &mut GpioPortInUseToken,
    ) -> GpioPin<GpioInConfig<PullUp>> {
        unsafe { self.to_input_pullup_no_sync() }
    }

    pub fn to_input_pulldown(
        self,
        _port_sync_token: &mut GpioPortInUseToken,
    ) -> GpioPin<GpioInConfig<PullDown>> {
        unsafe { self.to_input_pulldown_no_sync() }
    }

    pub fn to_output_pushpull(
        self,
        _port_sync_token: &mut GpioPortInUseToken,
    ) -> GpioPin<GpioOutConfig<PushPull>> {
        unsafe { self.to_output_pushpull_no_sync() }
    }

    pub fn to_output_opencollector(
        self,
        _port_sync_token: &mut GpioPortInUseToken,
    ) -> GpioPin<GpioOutConfig<OpenCollector>> {
        unsafe { self.to_output_opencollector_no_sync() }
    }

    // Unsafe state modification functions. These are safe to use only if there are no active
    // GPIO buses that have pins in the same 8-bit port as this pin.

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
    fn read(&self) -> bool {
        *self.input != 0
    }
}

impl GpioPinOutput for GpioPin<GpioOutConfig<PushPull>> {
    // Safe write functions.

    fn set(&mut self, _port_sync_token: &mut GpioPortInUseToken) {
        unsafe { self.set_no_sync() };
    }

    fn clear(&mut self, _port_sync_token: &mut GpioPortInUseToken) {
        unsafe { self.clear_no_sync() };
    }

    fn toggle(&mut self, _port_sync_token: &mut GpioPortInUseToken) {
        unsafe { self.toggle_no_sync() };
    }

    // Unsafe write functions. These are safe to use only if there are no active
    // GPIO buses that have pins in the same 8-bit port as this pin.

    unsafe fn set_no_sync(&mut self) {
        *self.output = 1;
    }

    unsafe fn clear_no_sync(&mut self) {
        *self.output = 0;
    }

    unsafe fn toggle_no_sync(&mut self) {
        *self.output ^= 1;
    }
}

impl GpioPinOutput for GpioPin<GpioOutConfig<OpenCollector>> {
    // Safe write functions.

    fn set(&mut self, _port_sync_token: &mut GpioPortInUseToken) {
        unsafe { self.set_no_sync() };
    }

    fn clear(&mut self, _port_sync_token: &mut GpioPortInUseToken) {
        unsafe { self.clear_no_sync() };
    }

    fn toggle(&mut self, _port_sync_token: &mut GpioPortInUseToken) {
        unsafe { self.toggle_no_sync() };
    }

    // Unsafe write functions. These are safe to use only if there are no active
    // GPIO buses that have pins in the same 8-bit port as this pin.

    unsafe fn set_no_sync(&mut self) {
        *self.direction = 0;
        *self.output = 1;
    }

    unsafe fn clear_no_sync(&mut self) {
        *self.output = 0;
        *self.direction = 1;
    }

    unsafe fn toggle_no_sync(&mut self) {
        if *self.input == 0 {
            self.set_no_sync();
        } else {
            self.clear_no_sync();
        }
    }
}

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
