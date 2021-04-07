#![no_std]
#![feature(min_const_generics)]

const PERIPHERAL_BASE: u32 = 0x4000_0000;
const PERIPHERAL_END: u32 = 0x400F_FFFF;
const PERIPHERAL_BITBAND_BASE: u32 = 0x4200_0000;

#[allow(dead_code)]
const PERIPHERAL_BITBAND_END: u32 = 0x43FF_FFFF;

fn peripheral_to_alias(address: u32, bit: u8) -> u32 {
    debug_assert!(address >= PERIPHERAL_BASE);
    debug_assert!(address <= PERIPHERAL_END);

    let byte_offset = address - PERIPHERAL_BASE;
    PERIPHERAL_BITBAND_BASE + (byte_offset * 32) + ((bit as u32) * 4)
}

pub enum Half {
    Lower = 0,
    Upper = 1,
}

pub enum Quarter {
    FirstQuartile = 0,
    SecondQuartile = 1,
    ThirdQuartile = 2,
    FourthQuartile = 3,
}

pub mod registers {
    use super::*;

    pub type Reg8 = u8;

    pub union Reg16 {
        bytes: [u8; 2],
        halfword: u16,
    }

    impl Reg16 {
        pub fn get_byte(&self, half: Half) -> u8 {
            let index = half as usize;
            unsafe { self.bytes[index] }
        }

        pub fn set_byte(&mut self, half: Half, value: u8) {
            let index = half as usize;
            unsafe { self.bytes[index] = value };
        }

        pub fn get_byte_ptr(&self, half: Half) -> *const u8 {
            let index = half as usize;
            unsafe { &self.bytes[index] as *const u8 }
        }

        pub fn get_byte_ptr_mut(&mut self, half: Half) -> *mut u8 {
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

//TODO: Move to separate project or "util" component
/// Consists of general utility functions and structures.
pub mod util {
    use core::num::NonZeroU32;

    /// Retries a provided operation if the operation result is a certain value.
    ///
    /// # Arguments
    /// `operation` - The operation to be executed.
    /// `retry_result` - The result value that will prompt a retry.
    /// `num_tries` - Optionally provides the maximum number of tries to attempt.
    ///
    /// # Returns
    /// The result of the last execution of the operation, or an error if the maximum number of tries
    /// was reached.
    pub fn retry_if<O, R>(
        mut operation: O,
        retry_result: R,
        num_tries: Option<NonZeroU32>,
    ) -> Result<R, ()>
    where
        O: FnMut() -> R,
        R: PartialEq,
    {
        match num_tries {
            Some(attempts) => {
                for _i in 0..attempts.get() {
                    let result = operation();
                    if result != retry_result {
                        return Ok(result);
                    }
                }

                Err(())
            }
            None => loop {
                let result = operation();
                if result != retry_result {
                    return Ok(result);
                }
            },
        }
    }
}

pub mod gpio;
pub mod pin;
pub mod watchdog;
