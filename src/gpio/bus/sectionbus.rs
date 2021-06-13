//! # SectionBus
//! The `sectionbus` module includes structures and functions to utilize a port section as a GPIO
//! bus.

use crate::{
    gpio::{
        get_gpio_port, Disabled, GpioIn, GpioInputMode, GpioMode, GpioOut, GpioOutputMode,
        HighImpedance, OpenCollector, PullDown, PullUp, PushPull,
    },
    pin::PortSectionX,
};

use super::{private, GpioBusInput, GpioBusOutput};

//
// Structures.
//

/// Represents a port section configured as a GPIO Bus.
pub struct GpioSectionBus<const SIZE: usize, Section: PortSectionX<SIZE>, Mode: GpioMode> {
    /// The specfic GPIO configuration.
    _config: Mode,

    /// The actual port section.
    section: Section,
}

/// The following implements state modification for GPIO Section Bus configurations.
impl<const SIZE: usize, Section: PortSectionX<SIZE>, Mode: GpioMode>
    GpioSectionBus<SIZE, Section, Mode>
{
    // Convert this port section into a high-impedance input bus.
    ///
    /// # Returns
    /// A GPIO Section Bus instance configured in high-impedance input mode.
    pub fn to_input_highz(self) -> GpioSectionBus<SIZE, Section, GpioIn<HighImpedance>> {
        let port_regs = get_gpio_port(self.section.get_port_name());

        port_regs
            .resistor_enable
            .modify(|value| value & !self.section.get_mask() as u16);

        port_regs
            .direction
            .modify(|value| value & !self.section.get_mask() as u16);

        GpioSectionBus {
            _config: GpioIn {
                _input_mode: HighImpedance,
            },

            section: self.section,
        }
    }

    /// Convert this port section into an input bus with pull-up resistors.
    ///
    /// # Returns
    /// A GPIO Section Bus instance configured in input mode with pull-up resistors.
    pub fn to_input_pullup(self) -> GpioSectionBus<SIZE, Section, GpioIn<PullUp>> {
        let port_regs = get_gpio_port(self.section.get_port_name());

        port_regs
            .resistor_enable
            .modify(|value| value | self.section.get_mask() as u16);

        port_regs
            .direction
            .modify(|value| value & !self.section.get_mask() as u16);

        port_regs
            .output
            .modify(|value| value | self.section.get_mask() as u16);

        GpioSectionBus {
            _config: GpioIn {
                _input_mode: PullUp,
            },

            section: self.section,
        }
    }

    /// Convert this port section into an input bus with pull-down resistors.
    ///
    /// # Returns
    /// A GPIO Section Bus instance configured in input mode with pull-down resistors.
    pub fn to_input_pulldown(self) -> GpioSectionBus<SIZE, Section, GpioIn<PullDown>> {
        let port_regs = get_gpio_port(self.section.get_port_name());

        port_regs
            .resistor_enable
            .modify(|value| value | self.section.get_mask() as u16);

        port_regs
            .direction
            .modify(|value| value & !self.section.get_mask() as u16);

        port_regs
            .output
            .modify(|value| value & !self.section.get_mask() as u16);

        GpioSectionBus {
            _config: GpioIn {
                _input_mode: PullDown,
            },

            section: self.section,
        }
    }

    /// Convert this port section into an output bus with push-pull configuration.
    ///
    /// # Returns
    /// A GPIO Section Bus instance configured in output mode with push-pull configuration.
    pub fn to_output_pushpull(self) -> GpioSectionBus<SIZE, Section, GpioOut<PushPull>> {
        let port_regs = get_gpio_port(self.section.get_port_name());

        port_regs
            .output
            .modify(|value| value & !self.section.get_mask() as u16);

        port_regs
            .direction
            .modify(|value| value | self.section.get_mask() as u16);

        GpioSectionBus {
            _config: GpioOut {
                _output_mode: PushPull,
            },

            section: self.section,
        }
    }

    /// Convert this port section into an output bus with open collector configuration.
    ///
    /// # Returns
    /// A GPIO Section Bus instance configured in output mode with open collector configuration.
    pub fn to_output_opencollector(self) -> GpioSectionBus<SIZE, Section, GpioOut<OpenCollector>> {
        let port_regs = get_gpio_port(self.section.get_port_name());

        port_regs
            .output
            .modify(|value| value & !self.section.get_mask() as u16);

        port_regs
            .direction
            .modify(|value| value | self.section.get_mask() as u16);

        port_regs
            .resistor_enable
            .modify(|value| value | self.section.get_mask() as u16);

        GpioSectionBus {
            _config: GpioOut {
                _output_mode: OpenCollector,
            },

            section: self.section,
        }
    }
}

impl<const SIZE: usize, Section: PortSectionX<SIZE>, InputMode: GpioInputMode> GpioBusInput<SIZE>
    for GpioSectionBus<SIZE, Section, GpioIn<InputMode>>
{
    /// Reads the value of the GPIO Bus.
    ///
    /// # Returns
    /// Value of the GPIO Bus.
    fn read(&self) -> usize {
        let port_regs = get_gpio_port(self.section.get_port_name());
        ((port_regs.input.read() & self.section.get_mask() as u16) >> self.section.get_offset())
            as usize
    }
}

impl<const SIZE: usize, Section: PortSectionX<SIZE>, OutputMode: GpioOutputMode> GpioBusInput<SIZE>
    for GpioSectionBus<SIZE, Section, GpioOut<OutputMode>>
{
    /// Reads the value of the GPIO Bus.
    ///
    /// # Returns
    /// Value of the GPIO Bus.
    fn read(&self) -> usize {
        let port_regs = get_gpio_port(self.section.get_port_name());
        ((port_regs.input.read() & self.section.get_mask() as u16) >> self.section.get_offset())
            as usize
    }
}

impl<const SIZE: usize, Section: PortSectionX<SIZE>> GpioBusOutput<SIZE>
    for GpioSectionBus<SIZE, Section, GpioOut<PushPull>>
{
    /// Sets the value of the GPIO Bus.
    ///
    /// # Arguments
    /// `value` - The value to write to the bus.
    fn write(&mut self, value: usize) {
        let masked_value = ((value << self.section.get_offset()) & self.section.get_mask()) as u16;

        let port_regs = get_gpio_port(self.section.get_port_name());
        port_regs
            .output
            .modify(|content| (content & !self.section.get_mask() as u16) | masked_value);
    }

    /// Sets bits on the GPIO Bus.
    ///
    /// # Arguments
    /// `set_mask` - The bits to set on the bus.
    fn set_bits(&mut self, set_mask: usize) {
        let masked_value =
            ((set_mask << self.section.get_offset()) & self.section.get_mask()) as u16;

        let port_regs = get_gpio_port(self.section.get_port_name());
        port_regs.output.modify(|value| value | masked_value);
    }

    /// Clears bits on the GPIO Bus.
    ///
    /// # Arguments
    /// `clear_mask` - The bits to clear on the bus.
    fn clear_bits(&mut self, clear_mask: usize) {
        let masked_value =
            ((clear_mask << self.section.get_offset()) & self.section.get_mask()) as u16;

        let port_regs = get_gpio_port(self.section.get_port_name());
        port_regs.output.modify(|value| value & !masked_value);
    }

    /// Toggles bits on the GPIO Bus.
    ///
    /// # Arguments
    /// `toggle_mask` - The bits to toggle on the bus.
    fn toggle_bits(&mut self, toggle_mask: usize) {
        let masked_value =
            ((toggle_mask << self.section.get_offset()) & self.section.get_mask()) as u16;

        let port_regs = get_gpio_port(self.section.get_port_name());
        port_regs.output.modify(|value| value ^ masked_value);
    }
}

//
// Note: GpioSectionBus<Port, GpioOut<OpenCollector>> is not implemented as the output value cannot
// be changed atomically.
//

impl<const SIZE: usize, Section: PortSectionX<SIZE>> GpioSectionBus<SIZE, Section, Disabled> {
    /// Allocates a new GPIO configured Port.
    ///
    /// # Arguments
    /// `port` - Provides the port to be configred for GPIO.
    ///
    /// # Returns
    /// A GPIO Port in the `Disabled` configuration.
    pub fn new(section: Section) -> Self {
        Self {
            _config: Disabled,
            section: section,
        }
    }
}

impl<const SIZE: usize, Section: PortSectionX<SIZE>, Mode: GpioMode> private::Sealed
    for GpioSectionBus<SIZE, Section, Mode>
{
}
