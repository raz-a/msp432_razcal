
#![cfg_attr(not(test), no_std)]

pub mod gpio;
pub mod pin;
pub mod watchdog;

pub(crate) type Reg8 = u8;

#[repr(C)]
pub(crate) union Reg16 {
    byte: (u8, u8),
    halfword: u16
}

#[repr(C)]
pub(crate) union Reg32 {
    byte: (u8, u8, u8, u8),
    halfword: (u16, u16),
    word: u32
}