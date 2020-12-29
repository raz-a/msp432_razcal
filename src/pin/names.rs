//! # Names
//! The `names` module includes structures and functions to label the different available pins and
//! ports on a given system.

use crate::Half;
use core::debug_assert;

// Ports.

/// Represents the size of a port.
#[derive(Copy, Clone)]
pub enum PortSize {
    Port8Bit,
    Port16Bit,
}

pub enum PortNameConversionResult {
    SinglePort(PortName),
    TwoPorts([PortName; 2]),
    HalfPort(PortName, Half),
}

/// Represents unique values for each port grouping.
#[derive(Copy, Clone)]
pub struct PortName {
    pub(super) number: usize,
    pub(super) size: PortSize,
}

impl PortName {
    //
    // 16-bit Port Names
    //

    pub const PORTA: PortName = PortName {
        number: 0,
        size: PortSize::Port16Bit,
    };

    pub const PORTB: PortName = PortName {
        number: 1,
        size: PortSize::Port16Bit,
    };

    pub const PORTC: PortName = PortName {
        number: 2,
        size: PortSize::Port16Bit,
    };

    pub const PORTD: PortName = PortName {
        number: 3,
        size: PortSize::Port16Bit,
    };

    pub const PORTE: PortName = PortName {
        number: 4,
        size: PortSize::Port16Bit,
    };

    pub const PORTJ: PortName = PortName {
        number: 5,
        size: PortSize::Port16Bit,
    };

    //
    // 8-bit Port Names
    //

    pub const PORT1: PortName = PortName {
        number: PortName::PORTA.number * 2,
        size: PortSize::Port8Bit,
    };

    pub const PORT2: PortName = PortName {
        number: PortName::PORTA.number * 2 + 1,
        size: PortSize::Port8Bit,
    };

    pub const PORT3: PortName = PortName {
        number: PortName::PORTB.number * 2,
        size: PortSize::Port8Bit,
    };

    pub const PORT4: PortName = PortName {
        number: PortName::PORTB.number * 2 + 1,
        size: PortSize::Port8Bit,
    };

    pub const PORT5: PortName = PortName {
        number: PortName::PORTC.number * 2,
        size: PortSize::Port8Bit,
    };

    pub const PORT6: PortName = PortName {
        number: PortName::PORTC.number * 2 + 1,
        size: PortSize::Port8Bit,
    };

    pub const PORT7: PortName = PortName {
        number: PortName::PORTD.number * 2,
        size: PortSize::Port8Bit,
    };

    pub const PORT8: PortName = PortName {
        number: PortName::PORTD.number * 2 + 1,
        size: PortSize::Port8Bit,
    };

    pub const PORT9: PortName = PortName {
        number: PortName::PORTE.number * 2,
        size: PortSize::Port8Bit,
    };

    pub const PORT10: PortName = PortName {
        number: PortName::PORTE.number * 2 + 1,
        size: PortSize::Port8Bit,
    };

    pub const PORTJ_8BIT: PortName = PortName {
        number: PortName::PORTJ.number * 2,
        size: PortSize::Port8Bit,
    };

    //
    // Module Functions.
    //

    /// Determines if port number corresponds to an upper half 8-bit port.
    ///
    /// # Returns
    /// Whether or not this port number represents an upper 8-bit port.
    pub(super) fn is_upper_half_port(&self) -> bool {
        match self.size {
            PortSize::Port8Bit => self.number & 1 != 0,
            _ => {
                debug_assert!(false);
                false
            }
        }
    }

    //
    // Info functions.
    //

    /// Calculates the 16-bit port number from a port name.
    ///
    /// # Returns
    /// The 16-bit port number
    pub fn get_16_bit_port_index(&self) -> usize {
        match self.size {
            PortSize::Port8Bit => self.number / 2,
            PortSize::Port16Bit => self.number,
        }
    }

    /// Calculates the 8-bit port number from a port name
    ///
    /// # Arguments
    /// `half` - Provides the half to calculate. Ignored if the port name represents an 8-bit port.
    ///
    /// # Returns
    /// The 8-bit port number
    pub fn get_8_bit_port_index(&self, half: Half) -> usize {
        match self.size {
            PortSize::Port16Bit => match half {
                Half::Lower => self.number * 2,
                Half::Upper => self.number * 2 + 1,
            },
            PortSize::Port8Bit => self.number,
        }
    }

    /// Determines the size of a port represented by the given port name.
    ///
    /// # Returns
    /// The port size of the port name.
    pub fn get_port_size(&self) -> PortSize {
        self.size
    }

    //
    // Conversion functions.
    //

    /// Converts the current port name to its 8-bit representation.
    ///
    /// # Returns
    /// The converted port name.
    pub fn to_8_bit(self) -> PortNameConversionResult {
        match self.size {
            PortSize::Port8Bit => PortNameConversionResult::SinglePort(self),
            PortSize::Port16Bit => {
                let lower_port = PortName {
                    number: self.get_8_bit_port_index(Half::Lower),
                    size: PortSize::Port16Bit,
                };

                let upper_port = PortName {
                    number: self.get_8_bit_port_index(Half::Lower),
                    size: PortSize::Port16Bit,
                };

                PortNameConversionResult::TwoPorts([lower_port, upper_port])
            }
        }
    }

    /// Converts the current port name to its 16-bit representation.
    ///
    /// # Returns
    /// The converted port name.
    pub fn to_16_bit(self) -> PortNameConversionResult {
        match self.size {
            PortSize::Port16Bit => PortNameConversionResult::SinglePort(self),
            PortSize::Port8Bit => {
                let half = if self.is_upper_half_port() {
                    Half::Upper
                } else {
                    Half::Lower
                };

                let port_name = PortName {
                    number: self.get_16_bit_port_index(),
                    size: PortSize::Port8Bit,
                };

                PortNameConversionResult::HalfPort(port_name, half)
            }
        }
    }
}

// Pins.

/// Represents unique values for each pin.
#[derive(Copy, Clone)]
pub struct PinName {
    pub(super) port_name: PortName,
    pub(super) pin_offset: usize,
}

impl PinName {
    //
    // 8-bit Pin Names
    //

    pub const P1_0: PinName = PinName {
        port_name: PortName::PORT1,
        pin_offset: 0,
    };
    pub const P1_1: PinName = PinName {
        port_name: PortName::PORT1,
        pin_offset: 1,
    };
    pub const P1_2: PinName = PinName {
        port_name: PortName::PORT1,
        pin_offset: 2,
    };
    pub const P1_3: PinName = PinName {
        port_name: PortName::PORT1,
        pin_offset: 3,
    };
    pub const P1_4: PinName = PinName {
        port_name: PortName::PORT1,
        pin_offset: 4,
    };
    pub const P1_5: PinName = PinName {
        port_name: PortName::PORT1,
        pin_offset: 5,
    };
    pub const P1_6: PinName = PinName {
        port_name: PortName::PORT1,
        pin_offset: 6,
    };
    pub const P1_7: PinName = PinName {
        port_name: PortName::PORT1,
        pin_offset: 7,
    };

    pub const P2_0: PinName = PinName {
        port_name: PortName::PORT2,
        pin_offset: 0,
    };
    pub const P2_1: PinName = PinName {
        port_name: PortName::PORT2,
        pin_offset: 1,
    };
    pub const P2_2: PinName = PinName {
        port_name: PortName::PORT2,
        pin_offset: 2,
    };
    pub const P2_3: PinName = PinName {
        port_name: PortName::PORT2,
        pin_offset: 3,
    };
    pub const P2_4: PinName = PinName {
        port_name: PortName::PORT2,
        pin_offset: 4,
    };
    pub const P2_5: PinName = PinName {
        port_name: PortName::PORT2,
        pin_offset: 5,
    };
    pub const P2_6: PinName = PinName {
        port_name: PortName::PORT2,
        pin_offset: 6,
    };
    pub const P2_7: PinName = PinName {
        port_name: PortName::PORT2,
        pin_offset: 7,
    };

    pub const P3_0: PinName = PinName {
        port_name: PortName::PORT3,
        pin_offset: 0,
    };
    pub const P3_1: PinName = PinName {
        port_name: PortName::PORT3,
        pin_offset: 1,
    };
    pub const P3_2: PinName = PinName {
        port_name: PortName::PORT3,
        pin_offset: 2,
    };
    pub const P3_3: PinName = PinName {
        port_name: PortName::PORT3,
        pin_offset: 3,
    };
    pub const P3_4: PinName = PinName {
        port_name: PortName::PORT3,
        pin_offset: 4,
    };
    pub const P3_5: PinName = PinName {
        port_name: PortName::PORT3,
        pin_offset: 5,
    };
    pub const P3_6: PinName = PinName {
        port_name: PortName::PORT3,
        pin_offset: 6,
    };
    pub const P3_7: PinName = PinName {
        port_name: PortName::PORT3,
        pin_offset: 7,
    };

    pub const P4_0: PinName = PinName {
        port_name: PortName::PORT4,
        pin_offset: 0,
    };
    pub const P4_1: PinName = PinName {
        port_name: PortName::PORT4,
        pin_offset: 1,
    };
    pub const P4_2: PinName = PinName {
        port_name: PortName::PORT4,
        pin_offset: 2,
    };
    pub const P4_3: PinName = PinName {
        port_name: PortName::PORT4,
        pin_offset: 3,
    };
    pub const P4_4: PinName = PinName {
        port_name: PortName::PORT4,
        pin_offset: 4,
    };
    pub const P4_5: PinName = PinName {
        port_name: PortName::PORT4,
        pin_offset: 5,
    };
    pub const P4_6: PinName = PinName {
        port_name: PortName::PORT4,
        pin_offset: 6,
    };
    pub const P4_7: PinName = PinName {
        port_name: PortName::PORT4,
        pin_offset: 7,
    };

    pub const P5_0: PinName = PinName {
        port_name: PortName::PORT5,
        pin_offset: 0,
    };
    pub const P5_1: PinName = PinName {
        port_name: PortName::PORT5,
        pin_offset: 1,
    };
    pub const P5_2: PinName = PinName {
        port_name: PortName::PORT5,
        pin_offset: 2,
    };
    pub const P5_3: PinName = PinName {
        port_name: PortName::PORT5,
        pin_offset: 3,
    };
    pub const P5_4: PinName = PinName {
        port_name: PortName::PORT5,
        pin_offset: 4,
    };
    pub const P5_5: PinName = PinName {
        port_name: PortName::PORT5,
        pin_offset: 5,
    };
    pub const P5_6: PinName = PinName {
        port_name: PortName::PORT5,
        pin_offset: 6,
    };
    pub const P5_7: PinName = PinName {
        port_name: PortName::PORT5,
        pin_offset: 7,
    };

    pub const P6_0: PinName = PinName {
        port_name: PortName::PORT6,
        pin_offset: 0,
    };
    pub const P6_1: PinName = PinName {
        port_name: PortName::PORT6,
        pin_offset: 1,
    };
    pub const P6_2: PinName = PinName {
        port_name: PortName::PORT6,
        pin_offset: 2,
    };
    pub const P6_3: PinName = PinName {
        port_name: PortName::PORT6,
        pin_offset: 3,
    };
    pub const P6_4: PinName = PinName {
        port_name: PortName::PORT6,
        pin_offset: 4,
    };
    pub const P6_5: PinName = PinName {
        port_name: PortName::PORT6,
        pin_offset: 5,
    };
    pub const P6_6: PinName = PinName {
        port_name: PortName::PORT6,
        pin_offset: 6,
    };
    pub const P6_7: PinName = PinName {
        port_name: PortName::PORT6,
        pin_offset: 7,
    };

    pub const P7_0: PinName = PinName {
        port_name: PortName::PORT7,
        pin_offset: 0,
    };
    pub const P7_1: PinName = PinName {
        port_name: PortName::PORT7,
        pin_offset: 1,
    };
    pub const P7_2: PinName = PinName {
        port_name: PortName::PORT7,
        pin_offset: 2,
    };
    pub const P7_3: PinName = PinName {
        port_name: PortName::PORT7,
        pin_offset: 3,
    };
    pub const P7_4: PinName = PinName {
        port_name: PortName::PORT7,
        pin_offset: 4,
    };
    pub const P7_5: PinName = PinName {
        port_name: PortName::PORT7,
        pin_offset: 5,
    };
    pub const P7_6: PinName = PinName {
        port_name: PortName::PORT7,
        pin_offset: 6,
    };
    pub const P7_7: PinName = PinName {
        port_name: PortName::PORT7,
        pin_offset: 7,
    };

    pub const P8_0: PinName = PinName {
        port_name: PortName::PORT8,
        pin_offset: 0,
    };
    pub const P8_1: PinName = PinName {
        port_name: PortName::PORT8,
        pin_offset: 1,
    };
    pub const P8_2: PinName = PinName {
        port_name: PortName::PORT8,
        pin_offset: 2,
    };
    pub const P8_3: PinName = PinName {
        port_name: PortName::PORT8,
        pin_offset: 3,
    };
    pub const P8_4: PinName = PinName {
        port_name: PortName::PORT8,
        pin_offset: 4,
    };
    pub const P8_5: PinName = PinName {
        port_name: PortName::PORT8,
        pin_offset: 5,
    };
    pub const P8_6: PinName = PinName {
        port_name: PortName::PORT8,
        pin_offset: 6,
    };
    pub const P8_7: PinName = PinName {
        port_name: PortName::PORT8,
        pin_offset: 7,
    };

    pub const P9_0: PinName = PinName {
        port_name: PortName::PORT9,
        pin_offset: 0,
    };
    pub const P9_1: PinName = PinName {
        port_name: PortName::PORT9,
        pin_offset: 1,
    };
    pub const P9_2: PinName = PinName {
        port_name: PortName::PORT9,
        pin_offset: 2,
    };
    pub const P9_3: PinName = PinName {
        port_name: PortName::PORT9,
        pin_offset: 3,
    };
    pub const P9_4: PinName = PinName {
        port_name: PortName::PORT9,
        pin_offset: 4,
    };
    pub const P9_5: PinName = PinName {
        port_name: PortName::PORT9,
        pin_offset: 5,
    };
    pub const P9_6: PinName = PinName {
        port_name: PortName::PORT9,
        pin_offset: 6,
    };
    pub const P9_7: PinName = PinName {
        port_name: PortName::PORT9,
        pin_offset: 7,
    };

    pub const P10_0: PinName = PinName {
        port_name: PortName::PORT10,
        pin_offset: 0,
    };
    pub const P10_1: PinName = PinName {
        port_name: PortName::PORT10,
        pin_offset: 1,
    };
    pub const P10_2: PinName = PinName {
        port_name: PortName::PORT10,
        pin_offset: 2,
    };
    pub const P10_3: PinName = PinName {
        port_name: PortName::PORT10,
        pin_offset: 3,
    };
    pub const P10_4: PinName = PinName {
        port_name: PortName::PORT10,
        pin_offset: 4,
    };
    pub const P10_5: PinName = PinName {
        port_name: PortName::PORT10,
        pin_offset: 5,
    };
    pub const P10_6: PinName = PinName {
        port_name: PortName::PORT10,
        pin_offset: 6,
    };
    pub const P10_7: PinName = PinName {
        port_name: PortName::PORT10,
        pin_offset: 7,
    };

    pub const PJ_0_8: PinName = PinName {
        port_name: PortName::PORTJ_8BIT,
        pin_offset: 0,
    };
    pub const PJ_1_8: PinName = PinName {
        port_name: PortName::PORTJ_8BIT,
        pin_offset: 1,
    };
    pub const PJ_2_8: PinName = PinName {
        port_name: PortName::PORTJ_8BIT,
        pin_offset: 2,
    };
    pub const PJ_3_8: PinName = PinName {
        port_name: PortName::PORTJ_8BIT,
        pin_offset: 3,
    };
    pub const PJ_4_8: PinName = PinName {
        port_name: PortName::PORTJ_8BIT,
        pin_offset: 4,
    };
    pub const PJ_5_8: PinName = PinName {
        port_name: PortName::PORTJ_8BIT,
        pin_offset: 5,
    };

    //
    // 16-bit Pin Names
    //

    pub const PA_0: PinName = PinName {
        port_name: PortName::PORTA,
        pin_offset: 0,
    };
    pub const PA_1: PinName = PinName {
        port_name: PortName::PORTA,
        pin_offset: 1,
    };
    pub const PA_2: PinName = PinName {
        port_name: PortName::PORTA,
        pin_offset: 2,
    };
    pub const PA_3: PinName = PinName {
        port_name: PortName::PORTA,
        pin_offset: 3,
    };
    pub const PA_4: PinName = PinName {
        port_name: PortName::PORTA,
        pin_offset: 4,
    };
    pub const PA_5: PinName = PinName {
        port_name: PortName::PORTA,
        pin_offset: 5,
    };
    pub const PA_6: PinName = PinName {
        port_name: PortName::PORTA,
        pin_offset: 6,
    };
    pub const PA_7: PinName = PinName {
        port_name: PortName::PORTA,
        pin_offset: 7,
    };
    pub const PA_8: PinName = PinName {
        port_name: PortName::PORTA,
        pin_offset: 8,
    };
    pub const PA_9: PinName = PinName {
        port_name: PortName::PORTA,
        pin_offset: 9,
    };
    pub const PA_10: PinName = PinName {
        port_name: PortName::PORTA,
        pin_offset: 10,
    };
    pub const PA_11: PinName = PinName {
        port_name: PortName::PORTA,
        pin_offset: 11,
    };
    pub const PA_12: PinName = PinName {
        port_name: PortName::PORTA,
        pin_offset: 12,
    };
    pub const PA_13: PinName = PinName {
        port_name: PortName::PORTA,
        pin_offset: 13,
    };
    pub const PA_14: PinName = PinName {
        port_name: PortName::PORTA,
        pin_offset: 14,
    };
    pub const PA_15: PinName = PinName {
        port_name: PortName::PORTA,
        pin_offset: 15,
    };

    pub const PB_0: PinName = PinName {
        port_name: PortName::PORTB,
        pin_offset: 0,
    };
    pub const PB_1: PinName = PinName {
        port_name: PortName::PORTB,
        pin_offset: 1,
    };
    pub const PB_2: PinName = PinName {
        port_name: PortName::PORTB,
        pin_offset: 2,
    };
    pub const PB_3: PinName = PinName {
        port_name: PortName::PORTB,
        pin_offset: 3,
    };
    pub const PB_4: PinName = PinName {
        port_name: PortName::PORTB,
        pin_offset: 4,
    };
    pub const PB_5: PinName = PinName {
        port_name: PortName::PORTB,
        pin_offset: 5,
    };
    pub const PB_6: PinName = PinName {
        port_name: PortName::PORTB,
        pin_offset: 6,
    };
    pub const PB_7: PinName = PinName {
        port_name: PortName::PORTB,
        pin_offset: 7,
    };
    pub const PB_8: PinName = PinName {
        port_name: PortName::PORTB,
        pin_offset: 8,
    };
    pub const PB_9: PinName = PinName {
        port_name: PortName::PORTB,
        pin_offset: 9,
    };
    pub const PB_10: PinName = PinName {
        port_name: PortName::PORTB,
        pin_offset: 10,
    };
    pub const PB_11: PinName = PinName {
        port_name: PortName::PORTB,
        pin_offset: 11,
    };
    pub const PB_12: PinName = PinName {
        port_name: PortName::PORTB,
        pin_offset: 12,
    };
    pub const PB_13: PinName = PinName {
        port_name: PortName::PORTB,
        pin_offset: 13,
    };
    pub const PB_14: PinName = PinName {
        port_name: PortName::PORTB,
        pin_offset: 14,
    };
    pub const PB_15: PinName = PinName {
        port_name: PortName::PORTB,
        pin_offset: 15,
    };

    pub const PC_0: PinName = PinName {
        port_name: PortName::PORTC,
        pin_offset: 0,
    };
    pub const PC_1: PinName = PinName {
        port_name: PortName::PORTC,
        pin_offset: 1,
    };
    pub const PC_2: PinName = PinName {
        port_name: PortName::PORTC,
        pin_offset: 2,
    };
    pub const PC_3: PinName = PinName {
        port_name: PortName::PORTC,
        pin_offset: 3,
    };
    pub const PC_4: PinName = PinName {
        port_name: PortName::PORTC,
        pin_offset: 4,
    };
    pub const PC_5: PinName = PinName {
        port_name: PortName::PORTC,
        pin_offset: 5,
    };
    pub const PC_6: PinName = PinName {
        port_name: PortName::PORTC,
        pin_offset: 6,
    };
    pub const PC_7: PinName = PinName {
        port_name: PortName::PORTC,
        pin_offset: 7,
    };
    pub const PC_8: PinName = PinName {
        port_name: PortName::PORTC,
        pin_offset: 8,
    };
    pub const PC_9: PinName = PinName {
        port_name: PortName::PORTC,
        pin_offset: 9,
    };
    pub const PC_10: PinName = PinName {
        port_name: PortName::PORTC,
        pin_offset: 10,
    };
    pub const PC_11: PinName = PinName {
        port_name: PortName::PORTC,
        pin_offset: 11,
    };
    pub const PC_12: PinName = PinName {
        port_name: PortName::PORTC,
        pin_offset: 12,
    };
    pub const PC_13: PinName = PinName {
        port_name: PortName::PORTC,
        pin_offset: 13,
    };
    pub const PC_14: PinName = PinName {
        port_name: PortName::PORTC,
        pin_offset: 14,
    };
    pub const PC_15: PinName = PinName {
        port_name: PortName::PORTC,
        pin_offset: 15,
    };

    pub const PD_0: PinName = PinName {
        port_name: PortName::PORTD,
        pin_offset: 0,
    };
    pub const PD_1: PinName = PinName {
        port_name: PortName::PORTD,
        pin_offset: 1,
    };
    pub const PD_2: PinName = PinName {
        port_name: PortName::PORTD,
        pin_offset: 2,
    };
    pub const PD_3: PinName = PinName {
        port_name: PortName::PORTD,
        pin_offset: 3,
    };
    pub const PD_4: PinName = PinName {
        port_name: PortName::PORTD,
        pin_offset: 4,
    };
    pub const PD_5: PinName = PinName {
        port_name: PortName::PORTD,
        pin_offset: 5,
    };
    pub const PD_6: PinName = PinName {
        port_name: PortName::PORTD,
        pin_offset: 6,
    };
    pub const PD_7: PinName = PinName {
        port_name: PortName::PORTD,
        pin_offset: 7,
    };
    pub const PD_8: PinName = PinName {
        port_name: PortName::PORTD,
        pin_offset: 8,
    };
    pub const PD_9: PinName = PinName {
        port_name: PortName::PORTD,
        pin_offset: 9,
    };
    pub const PD_10: PinName = PinName {
        port_name: PortName::PORTD,
        pin_offset: 10,
    };
    pub const PD_11: PinName = PinName {
        port_name: PortName::PORTD,
        pin_offset: 11,
    };
    pub const PD_12: PinName = PinName {
        port_name: PortName::PORTD,
        pin_offset: 12,
    };
    pub const PD_13: PinName = PinName {
        port_name: PortName::PORTD,
        pin_offset: 13,
    };
    pub const PD_14: PinName = PinName {
        port_name: PortName::PORTD,
        pin_offset: 14,
    };
    pub const PD_15: PinName = PinName {
        port_name: PortName::PORTD,
        pin_offset: 15,
    };

    pub const PE_0: PinName = PinName {
        port_name: PortName::PORTE,
        pin_offset: 0,
    };
    pub const PE_1: PinName = PinName {
        port_name: PortName::PORTE,
        pin_offset: 1,
    };
    pub const PE_2: PinName = PinName {
        port_name: PortName::PORTE,
        pin_offset: 2,
    };
    pub const PE_3: PinName = PinName {
        port_name: PortName::PORTE,
        pin_offset: 3,
    };
    pub const PE_4: PinName = PinName {
        port_name: PortName::PORTE,
        pin_offset: 4,
    };
    pub const PE_5: PinName = PinName {
        port_name: PortName::PORTE,
        pin_offset: 5,
    };
    pub const PE_6: PinName = PinName {
        port_name: PortName::PORTE,
        pin_offset: 6,
    };
    pub const PE_7: PinName = PinName {
        port_name: PortName::PORTE,
        pin_offset: 7,
    };
    pub const PE_8: PinName = PinName {
        port_name: PortName::PORTE,
        pin_offset: 8,
    };
    pub const PE_9: PinName = PinName {
        port_name: PortName::PORTE,
        pin_offset: 9,
    };
    pub const PE_10: PinName = PinName {
        port_name: PortName::PORTE,
        pin_offset: 10,
    };
    pub const PE_11: PinName = PinName {
        port_name: PortName::PORTE,
        pin_offset: 11,
    };
    pub const PE_12: PinName = PinName {
        port_name: PortName::PORTE,
        pin_offset: 12,
    };
    pub const PE_13: PinName = PinName {
        port_name: PortName::PORTE,
        pin_offset: 13,
    };
    pub const PE_14: PinName = PinName {
        port_name: PortName::PORTE,
        pin_offset: 14,
    };
    pub const PE_15: PinName = PinName {
        port_name: PortName::PORTE,
        pin_offset: 15,
    };

    pub const PJ_0: PinName = PinName {
        port_name: PortName::PORTJ,
        pin_offset: 0,
    };
    pub const PJ_1: PinName = PinName {
        port_name: PortName::PORTJ,
        pin_offset: 1,
    };
    pub const PJ_2: PinName = PinName {
        port_name: PortName::PORTJ,
        pin_offset: 2,
    };
    pub const PJ_3: PinName = PinName {
        port_name: PortName::PORTJ,
        pin_offset: 3,
    };
    pub const PJ_4: PinName = PinName {
        port_name: PortName::PORTJ,
        pin_offset: 4,
    };
    pub const PJ_5: PinName = PinName {
        port_name: PortName::PORTJ,
        pin_offset: 5,
    };

    //
    // Private functions.
    //

    //
    // Conversion functions.
    //

    /// Converts the current pin name to represent the pin in the context of an 8-bit port.
    ///
    /// # Returns
    /// Pin Name in the context of an 8-bit port.
    pub fn to_8_bit(self) -> PinName {
        match self.port_name.size {
            PortSize::Port8Bit => self,
            PortSize::Port16Bit => {
                let (offset, port_number) = if self.pin_offset < 8 {
                    (
                        self.pin_offset,
                        self.port_name.get_8_bit_port_index(Half::Lower),
                    )
                } else {
                    (
                        self.pin_offset - 8,
                        self.port_name.get_8_bit_port_index(Half::Upper),
                    )
                };

                PinName {
                    port_name: PortName {
                        number: port_number,
                        size: PortSize::Port8Bit,
                    },
                    pin_offset: offset,
                }
            }
        }
    }

    /// Converts the current pin name to represent the pin in the context of an 16-bit port.
    ///
    /// # Returns
    /// Pin Name in the context of an 16-bit port.
    pub fn to_16_bit(self) -> PinName {
        match self.port_name.size {
            PortSize::Port16Bit => self,
            PortSize::Port8Bit => {
                let offset = if self.port_name.is_upper_half_port() {
                    self.pin_offset + 8
                } else {
                    self.pin_offset
                };

                PinName {
                    port_name: PortName {
                        number: self.port_name.get_16_bit_port_index(),
                        size: PortSize::Port16Bit,
                    },
                    pin_offset: offset,
                }
            }
        }
    }

    /// Gets the owning port name.
    ///
    /// # Arguments
    /// `port_size` - Provides the size of the owning port to get.
    ///
    /// # Returns
    /// The owning port name.
    pub fn get_owning_port_name(&self, port_size: PortSize) -> PortName {
        match port_size {
            PortSize::Port8Bit => self.to_8_bit().port_name,
            PortSize::Port16Bit => self.to_16_bit().port_name,
        }
    }
}
