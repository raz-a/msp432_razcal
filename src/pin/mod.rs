//! # Pin
//! The `pin` module includes structures and functions to wrap microcontroller pins in borrowable
//! structures.

//
// Assure configuration variables are set.
//

#[cfg(not(all(razcal_gpio_port_size = "8", razcal_gpio_port_size = "16")))]
compile_error!("razcal_gpio_port_size should be defined as both 8 and 16 for MSP432");

//
// Internal Modules
//

mod pin;
mod port;

//
// Reexports
//

pub use pin::*;
pub use port::*;

//
// Dependencies
//

use core::sync::atomic::AtomicU16;

/// Represents unique values for each 8-bit port grouping.
#[repr(usize)]
#[derive(Copy, Clone)]
pub enum PortName8 {
    Port1 = 2 * (PortName16::PortA as usize),
    Port2 = 2 * (PortName16::PortA as usize) + 1,
    Port3 = 2 * (PortName16::PortB as usize),
    Port4 = 2 * (PortName16::PortB as usize) + 1,
    Port5 = 2 * (PortName16::PortC as usize),
    Port6 = 2 * (PortName16::PortC as usize) + 1,
    Port7 = 2 * (PortName16::PortD as usize),
    Port8 = 2 * (PortName16::PortD as usize) + 1,
    Port9 = 2 * (PortName16::PortE as usize),
    Port10 = 2 * (PortName16::PortE as usize) + 1,
    PortJ = 2 * (PortName16::PortJ as usize),
}

/// Represents unique values for each 16-bit port grouping.
#[repr(usize)]
#[derive(Copy, Clone)]
pub enum PortName16 {
    PortA = 0,
    PortB = 1,
    PortC = 2,
    PortD = 3,
    PortE = 4,
    PortJ = 5,
}

/// Represents unique values for each port grouping.
#[derive(Copy, Clone)]
pub enum PortName {
    Port8(PortName8),
    Port16(PortName16),
}

/// Represents the offset of a pin into its containing port.
#[repr(usize)]
#[derive(Copy, Clone)]
pub enum PinOffset {
    Offset0 = 0,
    Offset1 = 1,
    Offset2 = 2,
    Offset3 = 3,
    Offset4 = 4,
    Offset5 = 5,
    Offset6 = 6,
    Offset7 = 7,
    Offset8 = 8,
    Offset9 = 9,
    Offset10 = 10,
    Offset11 = 11,
    Offset12 = 12,
    Offset13 = 13,
    Offset14 = 14,
    Offset15 = 15,
}

/// Represents unique values for each pin.
#[derive(Copy, Clone)]
pub struct PinName {
    port_name: PortName,
    pin_offset: PinOffset,
}

/// Represents unique values for each pin in 8 bit format.
#[derive(Copy, Clone)]
pub struct PinName8 {
    port_name: PortName8,
    pin_offset: PinOffset,
}

/// Represents unique values for each pin in 16 bit format.
#[derive(Copy, Clone)]
pub struct PinName16 {
    port_name: PortName16,
    pin_offset: PinOffset,
}

impl PinName {
    //
    // 8-bit Pin Names
    //

    pub const P1_0: PinName = PinName {
        port_name: PortName::Port8(PortName8::Port1),
        pin_offset: PinOffset::Offset0,
    };
    pub const P1_1: PinName = PinName {
        port_name: PortName::Port8(PortName8::Port1),
        pin_offset: PinOffset::Offset1,
    };
    pub const P1_2: PinName = PinName {
        port_name: PortName::Port8(PortName8::Port1),
        pin_offset: PinOffset::Offset2,
    };
    pub const P1_3: PinName = PinName {
        port_name: PortName::Port8(PortName8::Port1),
        pin_offset: PinOffset::Offset3,
    };
    pub const P1_4: PinName = PinName {
        port_name: PortName::Port8(PortName8::Port1),
        pin_offset: PinOffset::Offset4,
    };
    pub const P1_5: PinName = PinName {
        port_name: PortName::Port8(PortName8::Port1),
        pin_offset: PinOffset::Offset5,
    };
    pub const P1_6: PinName = PinName {
        port_name: PortName::Port8(PortName8::Port1),
        pin_offset: PinOffset::Offset6,
    };
    pub const P1_7: PinName = PinName {
        port_name: PortName::Port8(PortName8::Port1),
        pin_offset: PinOffset::Offset7,
    };

    pub const P2_0: PinName = PinName {
        port_name: PortName::Port8(PortName8::Port2),
        pin_offset: PinOffset::Offset0,
    };
    pub const P2_1: PinName = PinName {
        port_name: PortName::Port8(PortName8::Port2),
        pin_offset: PinOffset::Offset1,
    };
    pub const P2_2: PinName = PinName {
        port_name: PortName::Port8(PortName8::Port2),
        pin_offset: PinOffset::Offset2,
    };
    pub const P2_3: PinName = PinName {
        port_name: PortName::Port8(PortName8::Port2),
        pin_offset: PinOffset::Offset3,
    };
    pub const P2_4: PinName = PinName {
        port_name: PortName::Port8(PortName8::Port2),
        pin_offset: PinOffset::Offset4,
    };
    pub const P2_5: PinName = PinName {
        port_name: PortName::Port8(PortName8::Port2),
        pin_offset: PinOffset::Offset5,
    };
    pub const P2_6: PinName = PinName {
        port_name: PortName::Port8(PortName8::Port2),
        pin_offset: PinOffset::Offset6,
    };
    pub const P2_7: PinName = PinName {
        port_name: PortName::Port8(PortName8::Port2),
        pin_offset: PinOffset::Offset7,
    };

    pub const P3_0: PinName = PinName {
        port_name: PortName::Port8(PortName8::Port3),
        pin_offset: PinOffset::Offset0,
    };
    pub const P3_1: PinName = PinName {
        port_name: PortName::Port8(PortName8::Port3),
        pin_offset: PinOffset::Offset1,
    };
    pub const P3_2: PinName = PinName {
        port_name: PortName::Port8(PortName8::Port3),
        pin_offset: PinOffset::Offset2,
    };
    pub const P3_3: PinName = PinName {
        port_name: PortName::Port8(PortName8::Port3),
        pin_offset: PinOffset::Offset3,
    };
    pub const P3_4: PinName = PinName {
        port_name: PortName::Port8(PortName8::Port3),
        pin_offset: PinOffset::Offset4,
    };
    pub const P3_5: PinName = PinName {
        port_name: PortName::Port8(PortName8::Port3),
        pin_offset: PinOffset::Offset5,
    };
    pub const P3_6: PinName = PinName {
        port_name: PortName::Port8(PortName8::Port3),
        pin_offset: PinOffset::Offset6,
    };
    pub const P3_7: PinName = PinName {
        port_name: PortName::Port8(PortName8::Port3),
        pin_offset: PinOffset::Offset7,
    };

    pub const P4_0: PinName = PinName {
        port_name: PortName::Port8(PortName8::Port4),
        pin_offset: PinOffset::Offset0,
    };
    pub const P4_1: PinName = PinName {
        port_name: PortName::Port8(PortName8::Port4),
        pin_offset: PinOffset::Offset1,
    };
    pub const P4_2: PinName = PinName {
        port_name: PortName::Port8(PortName8::Port4),
        pin_offset: PinOffset::Offset2,
    };
    pub const P4_3: PinName = PinName {
        port_name: PortName::Port8(PortName8::Port4),
        pin_offset: PinOffset::Offset3,
    };
    pub const P4_4: PinName = PinName {
        port_name: PortName::Port8(PortName8::Port4),
        pin_offset: PinOffset::Offset4,
    };
    pub const P4_5: PinName = PinName {
        port_name: PortName::Port8(PortName8::Port4),
        pin_offset: PinOffset::Offset5,
    };
    pub const P4_6: PinName = PinName {
        port_name: PortName::Port8(PortName8::Port4),
        pin_offset: PinOffset::Offset6,
    };
    pub const P4_7: PinName = PinName {
        port_name: PortName::Port8(PortName8::Port4),
        pin_offset: PinOffset::Offset7,
    };

    pub const P5_0: PinName = PinName {
        port_name: PortName::Port8(PortName8::Port5),
        pin_offset: PinOffset::Offset0,
    };
    pub const P5_1: PinName = PinName {
        port_name: PortName::Port8(PortName8::Port5),
        pin_offset: PinOffset::Offset1,
    };
    pub const P5_2: PinName = PinName {
        port_name: PortName::Port8(PortName8::Port5),
        pin_offset: PinOffset::Offset2,
    };
    pub const P5_3: PinName = PinName {
        port_name: PortName::Port8(PortName8::Port5),
        pin_offset: PinOffset::Offset3,
    };
    pub const P5_4: PinName = PinName {
        port_name: PortName::Port8(PortName8::Port5),
        pin_offset: PinOffset::Offset4,
    };
    pub const P5_5: PinName = PinName {
        port_name: PortName::Port8(PortName8::Port5),
        pin_offset: PinOffset::Offset5,
    };
    pub const P5_6: PinName = PinName {
        port_name: PortName::Port8(PortName8::Port5),
        pin_offset: PinOffset::Offset6,
    };
    pub const P5_7: PinName = PinName {
        port_name: PortName::Port8(PortName8::Port5),
        pin_offset: PinOffset::Offset7,
    };

    pub const P6_0: PinName = PinName {
        port_name: PortName::Port8(PortName8::Port6),
        pin_offset: PinOffset::Offset0,
    };
    pub const P6_1: PinName = PinName {
        port_name: PortName::Port8(PortName8::Port6),
        pin_offset: PinOffset::Offset1,
    };
    pub const P6_2: PinName = PinName {
        port_name: PortName::Port8(PortName8::Port6),
        pin_offset: PinOffset::Offset2,
    };
    pub const P6_3: PinName = PinName {
        port_name: PortName::Port8(PortName8::Port6),
        pin_offset: PinOffset::Offset3,
    };
    pub const P6_4: PinName = PinName {
        port_name: PortName::Port8(PortName8::Port6),
        pin_offset: PinOffset::Offset4,
    };
    pub const P6_5: PinName = PinName {
        port_name: PortName::Port8(PortName8::Port6),
        pin_offset: PinOffset::Offset5,
    };
    pub const P6_6: PinName = PinName {
        port_name: PortName::Port8(PortName8::Port6),
        pin_offset: PinOffset::Offset6,
    };
    pub const P6_7: PinName = PinName {
        port_name: PortName::Port8(PortName8::Port6),
        pin_offset: PinOffset::Offset7,
    };

    pub const P7_0: PinName = PinName {
        port_name: PortName::Port8(PortName8::Port7),
        pin_offset: PinOffset::Offset0,
    };
    pub const P7_1: PinName = PinName {
        port_name: PortName::Port8(PortName8::Port7),
        pin_offset: PinOffset::Offset1,
    };
    pub const P7_2: PinName = PinName {
        port_name: PortName::Port8(PortName8::Port7),
        pin_offset: PinOffset::Offset2,
    };
    pub const P7_3: PinName = PinName {
        port_name: PortName::Port8(PortName8::Port7),
        pin_offset: PinOffset::Offset3,
    };
    pub const P7_4: PinName = PinName {
        port_name: PortName::Port8(PortName8::Port7),
        pin_offset: PinOffset::Offset4,
    };
    pub const P7_5: PinName = PinName {
        port_name: PortName::Port8(PortName8::Port7),
        pin_offset: PinOffset::Offset5,
    };
    pub const P7_6: PinName = PinName {
        port_name: PortName::Port8(PortName8::Port7),
        pin_offset: PinOffset::Offset6,
    };
    pub const P7_7: PinName = PinName {
        port_name: PortName::Port8(PortName8::Port7),
        pin_offset: PinOffset::Offset7,
    };

    pub const P8_0: PinName = PinName {
        port_name: PortName::Port8(PortName8::Port8),
        pin_offset: PinOffset::Offset0,
    };
    pub const P8_1: PinName = PinName {
        port_name: PortName::Port8(PortName8::Port8),
        pin_offset: PinOffset::Offset1,
    };
    pub const P8_2: PinName = PinName {
        port_name: PortName::Port8(PortName8::Port8),
        pin_offset: PinOffset::Offset2,
    };
    pub const P8_3: PinName = PinName {
        port_name: PortName::Port8(PortName8::Port8),
        pin_offset: PinOffset::Offset3,
    };
    pub const P8_4: PinName = PinName {
        port_name: PortName::Port8(PortName8::Port8),
        pin_offset: PinOffset::Offset4,
    };
    pub const P8_5: PinName = PinName {
        port_name: PortName::Port8(PortName8::Port8),
        pin_offset: PinOffset::Offset5,
    };
    pub const P8_6: PinName = PinName {
        port_name: PortName::Port8(PortName8::Port8),
        pin_offset: PinOffset::Offset6,
    };
    pub const P8_7: PinName = PinName {
        port_name: PortName::Port8(PortName8::Port8),
        pin_offset: PinOffset::Offset7,
    };

    pub const P9_0: PinName = PinName {
        port_name: PortName::Port8(PortName8::Port9),
        pin_offset: PinOffset::Offset0,
    };
    pub const P9_1: PinName = PinName {
        port_name: PortName::Port8(PortName8::Port9),
        pin_offset: PinOffset::Offset1,
    };
    pub const P9_2: PinName = PinName {
        port_name: PortName::Port8(PortName8::Port9),
        pin_offset: PinOffset::Offset2,
    };
    pub const P9_3: PinName = PinName {
        port_name: PortName::Port8(PortName8::Port9),
        pin_offset: PinOffset::Offset3,
    };
    pub const P9_4: PinName = PinName {
        port_name: PortName::Port8(PortName8::Port9),
        pin_offset: PinOffset::Offset4,
    };
    pub const P9_5: PinName = PinName {
        port_name: PortName::Port8(PortName8::Port9),
        pin_offset: PinOffset::Offset5,
    };
    pub const P9_6: PinName = PinName {
        port_name: PortName::Port8(PortName8::Port9),
        pin_offset: PinOffset::Offset6,
    };
    pub const P9_7: PinName = PinName {
        port_name: PortName::Port8(PortName8::Port9),
        pin_offset: PinOffset::Offset7,
    };

    pub const P10_0: PinName = PinName {
        port_name: PortName::Port8(PortName8::Port10),
        pin_offset: PinOffset::Offset0,
    };
    pub const P10_1: PinName = PinName {
        port_name: PortName::Port8(PortName8::Port10),
        pin_offset: PinOffset::Offset1,
    };
    pub const P10_2: PinName = PinName {
        port_name: PortName::Port8(PortName8::Port10),
        pin_offset: PinOffset::Offset2,
    };
    pub const P10_3: PinName = PinName {
        port_name: PortName::Port8(PortName8::Port10),
        pin_offset: PinOffset::Offset3,
    };
    pub const P10_4: PinName = PinName {
        port_name: PortName::Port8(PortName8::Port10),
        pin_offset: PinOffset::Offset4,
    };
    pub const P10_5: PinName = PinName {
        port_name: PortName::Port8(PortName8::Port10),
        pin_offset: PinOffset::Offset5,
    };
    pub const P10_6: PinName = PinName {
        port_name: PortName::Port8(PortName8::Port10),
        pin_offset: PinOffset::Offset6,
    };
    pub const P10_7: PinName = PinName {
        port_name: PortName::Port8(PortName8::Port10),
        pin_offset: PinOffset::Offset7,
    };

    pub const PJ_0_8: PinName = PinName {
        port_name: PortName::Port8(PortName8::PortJ),
        pin_offset: PinOffset::Offset0,
    };
    pub const PJ_1_8: PinName = PinName {
        port_name: PortName::Port8(PortName8::PortJ),
        pin_offset: PinOffset::Offset1,
    };
    pub const PJ_2_8: PinName = PinName {
        port_name: PortName::Port8(PortName8::PortJ),
        pin_offset: PinOffset::Offset2,
    };
    pub const PJ_3_8: PinName = PinName {
        port_name: PortName::Port8(PortName8::PortJ),
        pin_offset: PinOffset::Offset3,
    };
    pub const PJ_4_8: PinName = PinName {
        port_name: PortName::Port8(PortName8::PortJ),
        pin_offset: PinOffset::Offset4,
    };
    pub const PJ_5_8: PinName = PinName {
        port_name: PortName::Port8(PortName8::PortJ),
        pin_offset: PinOffset::Offset5,
    };

    //
    // 16-bit Pin Names
    //

    pub const PA_0: PinName = PinName {
        port_name: PortName::Port16(PortName16::PortA),
        pin_offset: PinOffset::Offset0,
    };
    pub const PA_1: PinName = PinName {
        port_name: PortName::Port16(PortName16::PortA),
        pin_offset: PinOffset::Offset1,
    };
    pub const PA_2: PinName = PinName {
        port_name: PortName::Port16(PortName16::PortA),
        pin_offset: PinOffset::Offset2,
    };
    pub const PA_3: PinName = PinName {
        port_name: PortName::Port16(PortName16::PortA),
        pin_offset: PinOffset::Offset3,
    };
    pub const PA_4: PinName = PinName {
        port_name: PortName::Port16(PortName16::PortA),
        pin_offset: PinOffset::Offset4,
    };
    pub const PA_5: PinName = PinName {
        port_name: PortName::Port16(PortName16::PortA),
        pin_offset: PinOffset::Offset5,
    };
    pub const PA_6: PinName = PinName {
        port_name: PortName::Port16(PortName16::PortA),
        pin_offset: PinOffset::Offset6,
    };
    pub const PA_7: PinName = PinName {
        port_name: PortName::Port16(PortName16::PortA),
        pin_offset: PinOffset::Offset7,
    };
    pub const PA_8: PinName = PinName {
        port_name: PortName::Port16(PortName16::PortA),
        pin_offset: PinOffset::Offset8,
    };
    pub const PA_9: PinName = PinName {
        port_name: PortName::Port16(PortName16::PortA),
        pin_offset: PinOffset::Offset9,
    };
    pub const PA_10: PinName = PinName {
        port_name: PortName::Port16(PortName16::PortA),
        pin_offset: PinOffset::Offset10,
    };
    pub const PA_11: PinName = PinName {
        port_name: PortName::Port16(PortName16::PortA),
        pin_offset: PinOffset::Offset11,
    };
    pub const PA_12: PinName = PinName {
        port_name: PortName::Port16(PortName16::PortA),
        pin_offset: PinOffset::Offset12,
    };
    pub const PA_13: PinName = PinName {
        port_name: PortName::Port16(PortName16::PortA),
        pin_offset: PinOffset::Offset13,
    };
    pub const PA_14: PinName = PinName {
        port_name: PortName::Port16(PortName16::PortA),
        pin_offset: PinOffset::Offset14,
    };
    pub const PA_15: PinName = PinName {
        port_name: PortName::Port16(PortName16::PortA),
        pin_offset: PinOffset::Offset15,
    };

    pub const PB_0: PinName = PinName {
        port_name: PortName::Port16(PortName16::PortB),
        pin_offset: PinOffset::Offset0,
    };
    pub const PB_1: PinName = PinName {
        port_name: PortName::Port16(PortName16::PortB),
        pin_offset: PinOffset::Offset1,
    };
    pub const PB_2: PinName = PinName {
        port_name: PortName::Port16(PortName16::PortB),
        pin_offset: PinOffset::Offset2,
    };
    pub const PB_3: PinName = PinName {
        port_name: PortName::Port16(PortName16::PortB),
        pin_offset: PinOffset::Offset3,
    };
    pub const PB_4: PinName = PinName {
        port_name: PortName::Port16(PortName16::PortB),
        pin_offset: PinOffset::Offset4,
    };
    pub const PB_5: PinName = PinName {
        port_name: PortName::Port16(PortName16::PortB),
        pin_offset: PinOffset::Offset5,
    };
    pub const PB_6: PinName = PinName {
        port_name: PortName::Port16(PortName16::PortB),
        pin_offset: PinOffset::Offset6,
    };
    pub const PB_7: PinName = PinName {
        port_name: PortName::Port16(PortName16::PortB),
        pin_offset: PinOffset::Offset7,
    };
    pub const PB_8: PinName = PinName {
        port_name: PortName::Port16(PortName16::PortB),
        pin_offset: PinOffset::Offset8,
    };
    pub const PB_9: PinName = PinName {
        port_name: PortName::Port16(PortName16::PortB),
        pin_offset: PinOffset::Offset9,
    };
    pub const PB_10: PinName = PinName {
        port_name: PortName::Port16(PortName16::PortB),
        pin_offset: PinOffset::Offset10,
    };
    pub const PB_11: PinName = PinName {
        port_name: PortName::Port16(PortName16::PortB),
        pin_offset: PinOffset::Offset11,
    };
    pub const PB_12: PinName = PinName {
        port_name: PortName::Port16(PortName16::PortB),
        pin_offset: PinOffset::Offset12,
    };
    pub const PB_13: PinName = PinName {
        port_name: PortName::Port16(PortName16::PortB),
        pin_offset: PinOffset::Offset13,
    };
    pub const PB_14: PinName = PinName {
        port_name: PortName::Port16(PortName16::PortB),
        pin_offset: PinOffset::Offset14,
    };
    pub const PB_15: PinName = PinName {
        port_name: PortName::Port16(PortName16::PortB),
        pin_offset: PinOffset::Offset15,
    };

    pub const PC_0: PinName = PinName {
        port_name: PortName::Port16(PortName16::PortC),
        pin_offset: PinOffset::Offset0,
    };
    pub const PC_1: PinName = PinName {
        port_name: PortName::Port16(PortName16::PortC),
        pin_offset: PinOffset::Offset1,
    };
    pub const PC_2: PinName = PinName {
        port_name: PortName::Port16(PortName16::PortC),
        pin_offset: PinOffset::Offset2,
    };
    pub const PC_3: PinName = PinName {
        port_name: PortName::Port16(PortName16::PortC),
        pin_offset: PinOffset::Offset3,
    };
    pub const PC_4: PinName = PinName {
        port_name: PortName::Port16(PortName16::PortC),
        pin_offset: PinOffset::Offset4,
    };
    pub const PC_5: PinName = PinName {
        port_name: PortName::Port16(PortName16::PortC),
        pin_offset: PinOffset::Offset5,
    };
    pub const PC_6: PinName = PinName {
        port_name: PortName::Port16(PortName16::PortC),
        pin_offset: PinOffset::Offset6,
    };
    pub const PC_7: PinName = PinName {
        port_name: PortName::Port16(PortName16::PortC),
        pin_offset: PinOffset::Offset7,
    };
    pub const PC_8: PinName = PinName {
        port_name: PortName::Port16(PortName16::PortC),
        pin_offset: PinOffset::Offset8,
    };
    pub const PC_9: PinName = PinName {
        port_name: PortName::Port16(PortName16::PortC),
        pin_offset: PinOffset::Offset9,
    };
    pub const PC_10: PinName = PinName {
        port_name: PortName::Port16(PortName16::PortC),
        pin_offset: PinOffset::Offset10,
    };
    pub const PC_11: PinName = PinName {
        port_name: PortName::Port16(PortName16::PortC),
        pin_offset: PinOffset::Offset11,
    };
    pub const PC_12: PinName = PinName {
        port_name: PortName::Port16(PortName16::PortC),
        pin_offset: PinOffset::Offset12,
    };
    pub const PC_13: PinName = PinName {
        port_name: PortName::Port16(PortName16::PortC),
        pin_offset: PinOffset::Offset13,
    };
    pub const PC_14: PinName = PinName {
        port_name: PortName::Port16(PortName16::PortC),
        pin_offset: PinOffset::Offset14,
    };
    pub const PC_15: PinName = PinName {
        port_name: PortName::Port16(PortName16::PortC),
        pin_offset: PinOffset::Offset15,
    };

    pub const PD_0: PinName = PinName {
        port_name: PortName::Port16(PortName16::PortD),
        pin_offset: PinOffset::Offset0,
    };
    pub const PD_1: PinName = PinName {
        port_name: PortName::Port16(PortName16::PortD),
        pin_offset: PinOffset::Offset1,
    };
    pub const PD_2: PinName = PinName {
        port_name: PortName::Port16(PortName16::PortD),
        pin_offset: PinOffset::Offset2,
    };
    pub const PD_3: PinName = PinName {
        port_name: PortName::Port16(PortName16::PortD),
        pin_offset: PinOffset::Offset3,
    };
    pub const PD_4: PinName = PinName {
        port_name: PortName::Port16(PortName16::PortD),
        pin_offset: PinOffset::Offset4,
    };
    pub const PD_5: PinName = PinName {
        port_name: PortName::Port16(PortName16::PortD),
        pin_offset: PinOffset::Offset5,
    };
    pub const PD_6: PinName = PinName {
        port_name: PortName::Port16(PortName16::PortD),
        pin_offset: PinOffset::Offset6,
    };
    pub const PD_7: PinName = PinName {
        port_name: PortName::Port16(PortName16::PortD),
        pin_offset: PinOffset::Offset7,
    };
    pub const PD_8: PinName = PinName {
        port_name: PortName::Port16(PortName16::PortD),
        pin_offset: PinOffset::Offset8,
    };
    pub const PD_9: PinName = PinName {
        port_name: PortName::Port16(PortName16::PortD),
        pin_offset: PinOffset::Offset9,
    };
    pub const PD_10: PinName = PinName {
        port_name: PortName::Port16(PortName16::PortD),
        pin_offset: PinOffset::Offset10,
    };
    pub const PD_11: PinName = PinName {
        port_name: PortName::Port16(PortName16::PortD),
        pin_offset: PinOffset::Offset11,
    };
    pub const PD_12: PinName = PinName {
        port_name: PortName::Port16(PortName16::PortD),
        pin_offset: PinOffset::Offset12,
    };
    pub const PD_13: PinName = PinName {
        port_name: PortName::Port16(PortName16::PortD),
        pin_offset: PinOffset::Offset13,
    };
    pub const PD_14: PinName = PinName {
        port_name: PortName::Port16(PortName16::PortD),
        pin_offset: PinOffset::Offset14,
    };
    pub const PD_15: PinName = PinName {
        port_name: PortName::Port16(PortName16::PortD),
        pin_offset: PinOffset::Offset15,
    };

    pub const PE_0: PinName = PinName {
        port_name: PortName::Port16(PortName16::PortE),
        pin_offset: PinOffset::Offset0,
    };
    pub const PE_1: PinName = PinName {
        port_name: PortName::Port16(PortName16::PortE),
        pin_offset: PinOffset::Offset1,
    };
    pub const PE_2: PinName = PinName {
        port_name: PortName::Port16(PortName16::PortE),
        pin_offset: PinOffset::Offset2,
    };
    pub const PE_3: PinName = PinName {
        port_name: PortName::Port16(PortName16::PortE),
        pin_offset: PinOffset::Offset3,
    };
    pub const PE_4: PinName = PinName {
        port_name: PortName::Port16(PortName16::PortE),
        pin_offset: PinOffset::Offset4,
    };
    pub const PE_5: PinName = PinName {
        port_name: PortName::Port16(PortName16::PortE),
        pin_offset: PinOffset::Offset5,
    };
    pub const PE_6: PinName = PinName {
        port_name: PortName::Port16(PortName16::PortE),
        pin_offset: PinOffset::Offset6,
    };
    pub const PE_7: PinName = PinName {
        port_name: PortName::Port16(PortName16::PortE),
        pin_offset: PinOffset::Offset7,
    };
    pub const PE_8: PinName = PinName {
        port_name: PortName::Port16(PortName16::PortE),
        pin_offset: PinOffset::Offset8,
    };
    pub const PE_9: PinName = PinName {
        port_name: PortName::Port16(PortName16::PortE),
        pin_offset: PinOffset::Offset9,
    };
    pub const PE_10: PinName = PinName {
        port_name: PortName::Port16(PortName16::PortE),
        pin_offset: PinOffset::Offset10,
    };
    pub const PE_11: PinName = PinName {
        port_name: PortName::Port16(PortName16::PortE),
        pin_offset: PinOffset::Offset11,
    };
    pub const PE_12: PinName = PinName {
        port_name: PortName::Port16(PortName16::PortE),
        pin_offset: PinOffset::Offset12,
    };
    pub const PE_13: PinName = PinName {
        port_name: PortName::Port16(PortName16::PortE),
        pin_offset: PinOffset::Offset13,
    };
    pub const PE_14: PinName = PinName {
        port_name: PortName::Port16(PortName16::PortE),
        pin_offset: PinOffset::Offset14,
    };
    pub const PE_15: PinName = PinName {
        port_name: PortName::Port16(PortName16::PortE),
        pin_offset: PinOffset::Offset15,
    };

    pub const PJ_0: PinName = PinName {
        port_name: PortName::Port16(PortName16::PortJ),
        pin_offset: PinOffset::Offset0,
    };
    pub const PJ_1: PinName = PinName {
        port_name: PortName::Port16(PortName16::PortJ),
        pin_offset: PinOffset::Offset1,
    };
    pub const PJ_2: PinName = PinName {
        port_name: PortName::Port16(PortName16::PortJ),
        pin_offset: PinOffset::Offset2,
    };
    pub const PJ_3: PinName = PinName {
        port_name: PortName::Port16(PortName16::PortJ),
        pin_offset: PinOffset::Offset3,
    };
    pub const PJ_4: PinName = PinName {
        port_name: PortName::Port16(PortName16::PortJ),
        pin_offset: PinOffset::Offset4,
    };
    pub const PJ_5: PinName = PinName {
        port_name: PortName::Port16(PortName16::PortJ),
        pin_offset: PinOffset::Offset5,
    };

    //
    // Public functions.
    //

    //
    // TODO: Implement these functions for PinName8 and PinName16
    //

    /// Converts the current pin name to represent the pin in the context of an 8-bit port.
    ///
    /// # Returns
    /// Pin Name in the context of an 8-bit port.
    pub fn to_8_bit(&self) -> PinName8 {
        match self.port_name {
            PortName::Port16(port_name) => {
                let (port, offset) = if self.pin_offset as usize >= PinOffset::Offset8 as usize {
                    (
                        port_name as usize / 2 + 1,
                        self.pin_offset as usize - PinOffset::Offset8 as usize,
                    )
                } else {
                    (port_name as usize / 2, self.pin_offset as usize)
                };

                // UNSAFE: Transmute is legal as we are following the transofmation between a valid
                // 16-bit port name and 8-bit port name.

                PinName8 {
                    port_name: unsafe { core::mem::transmute(port) },
                    pin_offset: unsafe { core::mem::transmute(offset) },
                }
            }
            PortName::Port8(port_name) => PinName8 {
                port_name: port_name,
                pin_offset: self.pin_offset,
            },
        }
    }

    /// Converts the current pin name to represent the pin in the context of an 16-bit port.
    ///
    /// # Returns
    /// Pin Name in the context of an 16-bit port.
    pub fn to_16_bit(self) -> PinName16 {
        match self.port_name {
            PortName::Port8(port_name) => {
                let port = port_name as usize;
                let offset = if port & 1 == 0 {
                    self.pin_offset as usize
                } else {
                    self.pin_offset as usize + PinOffset::Offset8 as usize
                };

                let port = port * 2;

                // UNSAFE: Transmute is legal as we are following the transofmation between a valid
                // 16-bit port name and 8-bit port name.

                PinName16 {
                    port_name: unsafe { core::mem::transmute(port) },
                    pin_offset: unsafe { core::mem::transmute(offset) },
                }
            }

            PortName::Port16(port_name) => PinName16 {
                port_name: port_name,
                pin_offset: self.pin_offset,
            },
        }
    }
}

#[cfg(not(any(
    razcal_msp432_package = "vqfn",
    razcal_msp432_package = "nfbga",
    razcal_msp432_package = "lqfp"
)))]
compile_error!("Msp432 package must be defined.");

//
// Globals
//

/// Represents the pins available for the given controller.
#[cfg(razcal_msp432_package = "vqfn")]
static mut PORT_PINS_AVAILABLE: [AtomicU16; 6] = [
    AtomicU16::new(0x0FFF),
    AtomicU16::new(0xFCFF),
    AtomicU16::new(0xC0FF),
    AtomicU16::new(0x03FF),
    AtomicU16::new(0x0000),
    AtomicU16::new(0x003F),
];

/// Represents the pins available for the given controller.
#[cfg(razcal_msp432_package = "nfbga")]
static mut PORT_PINS_AVAILABLE: [AtomicU16; 6] = [
    AtomicU16::new(0xFFFF),
    AtomicU16::new(0xFFFF),
    AtomicU16::new(0xFFFF),
    AtomicU16::new(0x03FF),
    AtomicU16::new(0x0000),
    AtomicU16::new(0x003F),
];

/// Represents the pins available for the given controller.
#[cfg(razcal_msp432_package = "lqfp")]
static mut PORT_PINS_AVAILABLE: [AtomicU16; 6] = [
    AtomicU16::new(0xFFFF),
    AtomicU16::new(0xFFFF),
    AtomicU16::new(0xFFFF),
    AtomicU16::new(0xFFFF),
    AtomicU16::new(0xFFFF),
    AtomicU16::new(0x003F),
];
