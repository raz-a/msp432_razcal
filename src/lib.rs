#![cfg_attr(not(test), no_std)]
#![feature(const_generics)]
#![feature(maybe_uninit_uninit_array)]
#![feature(maybe_uninit_ref)]

const PERIPHERAL_BASE: u32 = 0x4000_0000;
const PERIPHERAL_END: u32 = 0x400F_FFFF;
const PERIPHERAL_BITBAND_BASE: u32 = 0x4200_0000;
const PERIPHERAL_BITBAND_END: u32 = 0x43FF_FFFF;

fn peripheral_to_alias(address: u32, bit: u8) -> u32 {
    debug_assert!(address >= PERIPHERAL_BASE);
    debug_assert!(address <= PERIPHERAL_END);

    let byte_offset = address - PERIPHERAL_BASE;
    PERIPHERAL_BITBAND_BASE + (byte_offset * 32) + ((bit as u32) * 4)
}

fn assume_init_const_generic_array<T, const SIZE: usize>(
    mut uninit_array: [core::mem::MaybeUninit<T>; SIZE],
) -> [T; SIZE] {
    let init_array = &mut uninit_array as *mut _ as *mut [T; SIZE];
    core::mem::forget(uninit_array);
    unsafe { init_array.read() }
}

pub mod registers {
    pub enum Halves {
        Lower = 0,
        Upper = 1,
    }

    pub enum Quarters {
        FirstQuartile = 0,
        SecondQuartile = 1,
        ThirdQuartile = 2,
        FourthQuartile = 3,
    }

    pub type Reg8 = u8;

    pub union Reg16 {
        bytes: [u8; 2],
        halfword: u16,
    }

    impl Reg16 {
        pub fn get_byte(&self, half: Halves) -> u8 {
            let index = half as usize;
            unsafe { self.bytes[index] }
        }

        pub fn set_byte(&mut self, half: Halves, value: u8) {
            let index = half as usize;
            unsafe { self.bytes[index] = value };
        }

        pub fn get_byte_ptr(&self, half: Halves) -> *const u8 {
            let index = half as usize;
            unsafe { &self.bytes[index] as *const u8 }
        }

        pub fn get_byte_ptr_mut(&mut self, half: Halves) -> *mut u8 {
            let index = half as usize;
            unsafe { &mut self.bytes[index] as *mut u8 }
        }

        pub fn get_halfword(&self) -> u16 {
            unsafe { self.halfword }
        }

        pub fn set_halfword(&mut self, value: u16) {
            self.halfword = value;
        }

        pub fn get_halfword_ptr(&self) -> *const u16 {
            unsafe { &self.halfword as *const u16 }
        }

        pub fn get_halfword_ptr_mut(&mut self) -> *mut u16 {
            unsafe { &mut self.halfword as *mut u16 }
        }
    }

    pub union Reg32 {
        _bytes: [u8; 4],
        _halfwords: [u16; 2],
        _word: u32,
    }

    //TODO: Implement 32 bit version
}

pub mod gpio;
pub mod pin;
pub mod watchdog;
