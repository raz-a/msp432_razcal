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

pub mod gpio;
pub mod pin;
pub mod watchdog;
