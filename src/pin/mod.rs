//! # Pin
//! The `pin` module includes structures and functions to wrap microcontroller pins in borrowable
//! structures.

//
// Internal Modules
//

mod pin;
mod pinset;
mod port;

//
// Reexports
//

pub use pin::*;
pub use pinset::*;
pub use port::*;

//
// Dependencies
//

use core::sync::atomic::AtomicU16;


/// Represents unique values for each port grouping.
#[derive(Copy, Clone)]
pub enum PortName {
    /// Port A (Port 1 + Port 2)
    PortA = 0,

    /// Port B (Port 3 + Port 4)
    PortB = 1,

    /// Port C (Port 5 + Port 6)
    PortC = 2,

    /// Port D (Port 7 + Port 8)
    PortD = 3,

    /// Port E (Port 9 + Port 10)
    PortE = 4,

    /// Port J
    PortJ = 5,
}

/// Represents unique values for each pin.
#[allow(non_camel_case_types)]
#[derive(Copy, Clone)]
pub enum PinName {
    PA_0 = pin_name(PortName::PortA, 0),
    PA_1 = pin_name(PortName::PortA, 1),
    PA_2 = pin_name(PortName::PortA, 2),
    PA_3 = pin_name(PortName::PortA, 3),
    PA_4 = pin_name(PortName::PortA, 4),
    PA_5 = pin_name(PortName::PortA, 5),
    PA_6 = pin_name(PortName::PortA, 6),
    PA_7 = pin_name(PortName::PortA, 7),
    PA_8 = pin_name(PortName::PortA, 8),
    PA_9 = pin_name(PortName::PortA, 9),
    PA_10 = pin_name(PortName::PortA, 10),
    PA_11 = pin_name(PortName::PortA, 11),
    PA_12 = pin_name(PortName::PortA, 12),
    PA_13 = pin_name(PortName::PortA, 13),
    PA_14 = pin_name(PortName::PortA, 14),
    PA_15 = pin_name(PortName::PortA, 15),
    PB_0 = pin_name(PortName::PortB, 0),
    PB_1 = pin_name(PortName::PortB, 1),
    PB_2 = pin_name(PortName::PortB, 2),
    PB_3 = pin_name(PortName::PortB, 3),
    PB_4 = pin_name(PortName::PortB, 4),
    PB_5 = pin_name(PortName::PortB, 5),
    PB_6 = pin_name(PortName::PortB, 6),
    PB_7 = pin_name(PortName::PortB, 7),
    PB_8 = pin_name(PortName::PortB, 8),
    PB_9 = pin_name(PortName::PortB, 9),
    PB_10 = pin_name(PortName::PortB, 10),
    PB_11 = pin_name(PortName::PortB, 11),
    PB_12 = pin_name(PortName::PortB, 12),
    PB_13 = pin_name(PortName::PortB, 13),
    PB_14 = pin_name(PortName::PortB, 14),
    PB_15 = pin_name(PortName::PortB, 15),
    PC_0 = pin_name(PortName::PortC, 0),
    PC_1 = pin_name(PortName::PortC, 1),
    PC_2 = pin_name(PortName::PortC, 2),
    PC_3 = pin_name(PortName::PortC, 3),
    PC_4 = pin_name(PortName::PortC, 4),
    PC_5 = pin_name(PortName::PortC, 5),
    PC_6 = pin_name(PortName::PortC, 6),
    PC_7 = pin_name(PortName::PortC, 7),
    PC_8 = pin_name(PortName::PortC, 8),
    PC_9 = pin_name(PortName::PortC, 9),
    PC_10 = pin_name(PortName::PortC, 10),
    PC_11 = pin_name(PortName::PortC, 11),
    PC_12 = pin_name(PortName::PortC, 12),
    PC_13 = pin_name(PortName::PortC, 13),
    PC_14 = pin_name(PortName::PortC, 14),
    PC_15 = pin_name(PortName::PortC, 15),
    PD_0 = pin_name(PortName::PortD, 0),
    PD_1 = pin_name(PortName::PortD, 1),
    PD_2 = pin_name(PortName::PortD, 2),
    PD_3 = pin_name(PortName::PortD, 3),
    PD_4 = pin_name(PortName::PortD, 4),
    PD_5 = pin_name(PortName::PortD, 5),
    PD_6 = pin_name(PortName::PortD, 6),
    PD_7 = pin_name(PortName::PortD, 7),
    PD_8 = pin_name(PortName::PortD, 8),
    PD_9 = pin_name(PortName::PortD, 9),
    PD_10 = pin_name(PortName::PortD, 10),
    PD_11 = pin_name(PortName::PortD, 11),
    PD_12 = pin_name(PortName::PortD, 12),
    PD_13 = pin_name(PortName::PortD, 13),
    PD_14 = pin_name(PortName::PortD, 14),
    PD_15 = pin_name(PortName::PortD, 15),
    PE_0 = pin_name(PortName::PortE, 0),
    PE_1 = pin_name(PortName::PortE, 1),
    PE_2 = pin_name(PortName::PortE, 2),
    PE_3 = pin_name(PortName::PortE, 3),
    PE_4 = pin_name(PortName::PortE, 4),
    PE_5 = pin_name(PortName::PortE, 5),
    PE_6 = pin_name(PortName::PortE, 6),
    PE_7 = pin_name(PortName::PortE, 7),
    PE_8 = pin_name(PortName::PortE, 8),
    PE_9 = pin_name(PortName::PortE, 9),
    PE_10 = pin_name(PortName::PortE, 10),
    PE_11 = pin_name(PortName::PortE, 11),
    PE_12 = pin_name(PortName::PortE, 12),
    PE_13 = pin_name(PortName::PortE, 13),
    PE_14 = pin_name(PortName::PortE, 14),
    PE_15 = pin_name(PortName::PortE, 15),
    PJ_0 = pin_name(PortName::PortJ, 0),
    PJ_1 = pin_name(PortName::PortJ, 1),
    PJ_2 = pin_name(PortName::PortJ, 2),
    PJ_3 = pin_name(PortName::PortJ, 3),
    PJ_4 = pin_name(PortName::PortJ, 4),
    PJ_5 = pin_name(PortName::PortJ, 5),
}

/// Pin Aliases
impl PinName {
    pub const P1_0: PinName = PinName::PA_0;
    pub const P1_1: PinName = PinName::PA_1;
    pub const P1_2: PinName = PinName::PA_2;
    pub const P1_3: PinName = PinName::PA_3;
    pub const P1_4: PinName = PinName::PA_4;
    pub const P1_5: PinName = PinName::PA_5;
    pub const P1_6: PinName = PinName::PA_6;
    pub const P1_7: PinName = PinName::PA_7;
    pub const P2_0: PinName = PinName::PA_8;
    pub const P2_1: PinName = PinName::PA_9;
    pub const P2_2: PinName = PinName::PA_10;
    pub const P2_3: PinName = PinName::PA_11;
    pub const P2_4: PinName = PinName::PA_12;
    pub const P2_5: PinName = PinName::PA_13;
    pub const P2_6: PinName = PinName::PA_14;
    pub const P2_7: PinName = PinName::PA_15;
    pub const P3_0: PinName = PinName::PB_0;
    pub const P3_1: PinName = PinName::PB_1;
    pub const P3_2: PinName = PinName::PB_2;
    pub const P3_3: PinName = PinName::PB_3;
    pub const P3_4: PinName = PinName::PB_4;
    pub const P3_5: PinName = PinName::PB_5;
    pub const P3_6: PinName = PinName::PB_6;
    pub const P3_7: PinName = PinName::PB_7;
    pub const P4_0: PinName = PinName::PB_8;
    pub const P4_1: PinName = PinName::PB_9;
    pub const P4_2: PinName = PinName::PB_10;
    pub const P4_3: PinName = PinName::PB_11;
    pub const P4_4: PinName = PinName::PB_12;
    pub const P4_5: PinName = PinName::PB_13;
    pub const P4_6: PinName = PinName::PB_14;
    pub const P4_7: PinName = PinName::PB_15;
    pub const P5_0: PinName = PinName::PC_0;
    pub const P5_1: PinName = PinName::PC_1;
    pub const P5_2: PinName = PinName::PC_2;
    pub const P5_3: PinName = PinName::PC_3;
    pub const P5_4: PinName = PinName::PC_4;
    pub const P5_5: PinName = PinName::PC_5;
    pub const P5_6: PinName = PinName::PC_6;
    pub const P5_7: PinName = PinName::PC_7;
    pub const P6_0: PinName = PinName::PC_8;
    pub const P6_1: PinName = PinName::PC_9;
    pub const P6_2: PinName = PinName::PC_10;
    pub const P6_3: PinName = PinName::PC_11;
    pub const P6_4: PinName = PinName::PC_12;
    pub const P6_5: PinName = PinName::PC_13;
    pub const P6_6: PinName = PinName::PC_14;
    pub const P6_7: PinName = PinName::PC_15;
    pub const P7_0: PinName = PinName::PD_0;
    pub const P7_1: PinName = PinName::PD_1;
    pub const P7_2: PinName = PinName::PD_2;
    pub const P7_3: PinName = PinName::PD_3;
    pub const P7_4: PinName = PinName::PD_4;
    pub const P7_5: PinName = PinName::PD_5;
    pub const P7_6: PinName = PinName::PD_6;
    pub const P7_7: PinName = PinName::PD_7;
    pub const P8_0: PinName = PinName::PD_8;
    pub const P8_1: PinName = PinName::PD_9;
    pub const P8_2: PinName = PinName::PD_10;
    pub const P8_3: PinName = PinName::PD_11;
    pub const P8_4: PinName = PinName::PD_12;
    pub const P8_5: PinName = PinName::PD_13;
    pub const P8_6: PinName = PinName::PD_14;
    pub const P8_7: PinName = PinName::PD_15;
    pub const P9_0: PinName = PinName::PE_0;
    pub const P9_1: PinName = PinName::PE_1;
    pub const P9_2: PinName = PinName::PE_2;
    pub const P9_3: PinName = PinName::PE_3;
    pub const P9_4: PinName = PinName::PE_4;
    pub const P9_5: PinName = PinName::PE_5;
    pub const P9_6: PinName = PinName::PE_6;
    pub const P9_7: PinName = PinName::PE_7;
    pub const P10_0: PinName = PinName::PE_8;
    pub const P10_1: PinName = PinName::PE_9;
    pub const P10_2: PinName = PinName::PE_10;
    pub const P10_3: PinName = PinName::PE_11;
    pub const P10_4: PinName = PinName::PE_12;
    pub const P10_5: PinName = PinName::PE_13;
    pub const P10_6: PinName = PinName::PE_14;
    pub const P10_7: PinName = PinName::PE_15;
}

#[cfg(not(any(
    msp432_package = "vqfn",
    msp432_package = "nfbga",
    msp432_package = "lqfp"
)))]
compile_error!("Msp432 package must be defined.");

//
// Globals
//

/// Represents the pins available for the given controller.
#[cfg(msp432_package = "vqfn")]
static mut PORT_PINS_AVAILABLE: [AtomicU16; 6] = [
    AtomicU16::new(0x0FFF),
    AtomicU16::new(0xFCFF),
    AtomicU16::new(0xC0FF),
    AtomicU16::new(0x03FF),
    AtomicU16::new(0x0000),
    AtomicU16::new(0x003F),
];

/// Represents the pins available for the given controller.
#[cfg(msp432_package = "nfbga")]
static mut PORT_PINS_AVAILABLE: [AtomicU16; 6] = [
    AtomicU16::new(0xFFFF),
    AtomicU16::new(0xFFFF),
    AtomicU16::new(0xFFFF),
    AtomicU16::new(0x03FF),
    AtomicU16::new(0x0000),
    AtomicU16::new(0x003F),
];

/// Represents the pins available for the given controller.
#[cfg(msp432_package = "lqfp")]
static mut PORT_PINS_AVAILABLE: [AtomicU16; 6] = [
    AtomicU16::new(0xFFFF),
    AtomicU16::new(0xFFFF),
    AtomicU16::new(0xFFFF),
    AtomicU16::new(0xFFFF),
    AtomicU16::new(0xFFFF),
    AtomicU16::new(0x003F),
];

//
// Module Private Functions.
//

/// Calculate the PinName from a PortName and pin offset.
///
/// # Arguments
/// `port` - Provides the port name.
/// `pin` - Provides the pin offset.
///
/// # Returns
/// The concatenated PinName.
const fn pin_name(port: PortName, pin: u8) -> isize {
    (port as isize) << 8 | (pin as isize)
}

/// Extracts the port number from a PinName.
///
/// # Arguments
/// `pin_name` - Provides the PinName.
///
/// # Returns
/// The port number.
const fn extract_port_number(pin_name: PinName) -> u8 {
    ((pin_name as u16) >> 8) as u8
}

/// Extracts the pin offset from a PinName.
///
/// # Arguments
/// `pin_name` - Provides the PinName.
///
/// # Returns
/// The pin offset.
const fn extract_pin_number(pin_name: PinName) -> u8 {
    ((pin_name as u16) & 0xFF) as u8
}
