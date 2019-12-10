
use core::sync::atomic::{AtomicU16, Ordering};

#[allow(non_camel_case_types)]
#[derive(Copy, Clone)]
pub enum PinName {

    /// Port A (Port 1 + Port 2)

    PA_0 =  pin_name(0, 0),
    PA_1 =  pin_name(0, 1),
    PA_2 =  pin_name(0, 2),
    PA_3 =  pin_name(0, 3),
    PA_4 =  pin_name(0, 4),
    PA_5 =  pin_name(0, 5),
    PA_6 =  pin_name(0, 6),
    PA_7 =  pin_name(0, 7),
    PA_8 =  pin_name(0, 8),
    PA_9 =  pin_name(0, 9),
    PA_10 = pin_name(0, 10),
    PA_11 = pin_name(0, 11),
    PA_12 = pin_name(0, 12),
    PA_13 = pin_name(0, 13),
    PA_14 = pin_name(0, 14),
    PA_15 = pin_name(0, 15),

    /// Port B (Port 3 + Port 4)

    PB_0 =  pin_name(1, 0),
    PB_1 =  pin_name(1, 1),
    PB_2 =  pin_name(1, 2),
    PB_3 =  pin_name(1, 3),
    PB_4 =  pin_name(1, 4),
    PB_5 =  pin_name(1, 5),
    PB_6 =  pin_name(1, 6),
    PB_7 =  pin_name(1, 7),
    PB_8 =  pin_name(1, 8),
    PB_9 =  pin_name(1, 9),
    PB_10 = pin_name(1, 10),
    PB_11 = pin_name(1, 11),
    PB_12 = pin_name(1, 12),
    PB_13 = pin_name(1, 13),
    PB_14 = pin_name(1, 14),
    PB_15 = pin_name(1, 15),

    /// Port C (Port 5 + Port 6)

    PC_0 =  pin_name(2, 0),
    PC_1 =  pin_name(2, 1),
    PC_2 =  pin_name(2, 2),
    PC_3 =  pin_name(2, 3),
    PC_4 =  pin_name(2, 4),
    PC_5 =  pin_name(2, 5),
    PC_6 =  pin_name(2, 6),
    PC_7 =  pin_name(2, 7),
    PC_8 =  pin_name(2, 8),
    PC_9 =  pin_name(2, 9),
    PC_10 = pin_name(2, 10),
    PC_11 = pin_name(2, 11),
    PC_12 = pin_name(2, 12),
    PC_13 = pin_name(2, 13),
    PC_14 = pin_name(2, 14),
    PC_15 = pin_name(2, 15),

    /// Port D (Port 7 + Port 8)

    PD_0 =  pin_name(3, 0),
    PD_1 =  pin_name(3, 1),
    PD_2 =  pin_name(3, 2),
    PD_3 =  pin_name(3, 3),
    PD_4 =  pin_name(3, 4),
    PD_5 =  pin_name(3, 5),
    PD_6 =  pin_name(3, 6),
    PD_7 =  pin_name(3, 7),
    PD_8 =  pin_name(3, 8),
    PD_9 =  pin_name(3, 9),
    PD_10 = pin_name(3, 10),
    PD_11 = pin_name(3, 11),
    PD_12 = pin_name(3, 12),
    PD_13 = pin_name(3, 13),
    PD_14 = pin_name(3, 14),
    PD_15 = pin_name(3, 15),

    /// Port E (Port 9 + Port 10)

    PE_0 =  pin_name(4, 0),
    PE_1 =  pin_name(4, 1),
    PE_2 =  pin_name(4, 2),
    PE_3 =  pin_name(4, 3),
    PE_4 =  pin_name(4, 4),
    PE_5 =  pin_name(4, 5),
    PE_6 =  pin_name(4, 6),
    PE_7 =  pin_name(4, 7),
    PE_8 =  pin_name(4, 8),
    PE_9 =  pin_name(4, 9),
    PE_10 = pin_name(4, 10),
    PE_11 = pin_name(4, 11),
    PE_12 = pin_name(4, 12),
    PE_13 = pin_name(4, 13),
    PE_14 = pin_name(4, 14),
    PE_15 = pin_name(4, 15),

    /// Port J

    PJ_0 = pin_name(5, 0),
    PJ_1 = pin_name(5, 1),
    PJ_2 = pin_name(5, 2),
    PJ_3 = pin_name(5, 3),
    PJ_4 = pin_name(5, 4),
    PJ_5 = pin_name(5, 5),
}

impl PinName {

    /// Pin Aliases

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

pub struct Pin {
    pin: PinName
}

//
// TODO: build.rs to define package type from msp432 type
//

#[cfg(not(any(msp432_package = "vqfn", msp432_package = "nfbga", msp432_package = "lqfp")))]
compile_error!("Msp432 package must be defined.");

#[cfg(msp432_package = "vqfn")]
static mut PORT_PINS_AVAILABLE: [AtomicU16; 6] = [
    AtomicU16::new(0x0FFF),
    AtomicU16::new(0xFCFF),
    AtomicU16::new(0xC0FF),
    AtomicU16::new(0x03FF),
    AtomicU16::new(0x0000),
    AtomicU16::new(0x003F),
];

#[cfg(msp432_package = "nfbga")]
static mut PORT_PINS_AVAILABLE: [AtomicU16; 6] = [
    AtomicU16::new(0xFFFF),
    AtomicU16::new(0xFFFF),
    AtomicU16::new(0xFFFF),
    AtomicU16::new(0x03FF),
    AtomicU16::new(0x0000),
    AtomicU16::new(0x003F),
];

#[cfg(msp432_package = "lqfp")]
static mut PORT_PINS_AVAILABLE: [AtomicU16; 6] = [
    AtomicU16::new(0xFFFF),
    AtomicU16::new(0xFFFF),
    AtomicU16::new(0xFFFF),
    AtomicU16::new(0xFFFF),
    AtomicU16::new(0xFFFF),
    AtomicU16::new(0x003F),
];

const fn pin_name(port: u8, pin: u8) -> isize {
    (port as isize) << 8 | (pin as isize)
}

const fn extract_port_number(pin_name: PinName) -> u8 {
    ((pin_name as u16) >> 8) as u8
}

const fn extract_pin_number(pin_name: PinName) -> u8 {
    ((pin_name as u16) & 0xFF) as u8
}

impl Pin {
    pub fn new(pin: PinName) -> Option<Self> {
        let port = extract_port_number(pin) as usize;
        let pin_mask = 1 << extract_pin_number(pin);
        let value = unsafe {
            PORT_PINS_AVAILABLE[port].fetch_nand(pin_mask, Ordering::Relaxed)
        };

        if value & pin_mask == 0 {
            return None;
        }

        Some(Pin {pin: pin})
    }

    pub fn get_pin(&self) -> PinName {
        self.pin
    }

    pub fn get_port(&self) -> u8 {
        extract_port_number(self.pin)
    }

    pub fn get_pin_offset_in_port(&self) -> u8 {
        extract_pin_number(self.pin)
    }
}

impl Drop for Pin {
    fn drop(&mut self) {
        let port = extract_port_number(self.pin) as usize;
        let pin_mask = 1 << extract_pin_number(self.pin);
        unsafe {
            PORT_PINS_AVAILABLE[port].fetch_or(pin_mask, Ordering::Relaxed);
        }
    }
}