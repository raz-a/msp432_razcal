//! # Registers
//! The `registers` module includes structure to properly interact with registers as well as
//! constants that hold the location of memory mapped registers.

use vcell::VolatileCell;

fn peripheral_to_bitband_alias(address: u32, bit: u8) -> u32 {
    debug_assert!(address >= PERIPHERAL_BASE);
    debug_assert!(address <= PERIPHERAL_END);

    let byte_offset = address - PERIPHERAL_BASE;
    PERIPHERAL_BITBAND_BASE + (byte_offset * 32) + ((bit as u32) * 4)
}

/// Represents a reserved register.
pub struct Reserved<T: Copy> {
    _field: T,
}

/// Represents a read-only register.
pub struct ReadOnly<T: Copy> {
    value: VolatileCell<T>,
}

impl<T: Copy> ReadOnly<T> {
    /// Reads the register.
    ///
    /// # Returns
    /// The value of the register.
    pub fn read(&self) -> T {
        self.value.get()
    }
}

/// Represents a write-only register.
pub struct WriteOnly<T: Copy> {
    value: VolatileCell<T>,
}

impl<T: Copy> WriteOnly<T> {
    /// Writes to the register.
    ///
    /// # Arguments
    /// `value` - The value to write to the register.
    pub fn write(&self, value: T) {
        self.value.set(value);
    }
}

/// Represents a read/write register.
pub struct ReadWrite<T: Copy> {
    value: VolatileCell<T>,
}

impl<T: Copy> ReadWrite<T> {
    /// Reads the register.
    ///
    /// # Returns
    /// The value of the register.
    pub fn read(&self) -> T {
        self.value.get()
    }

    /// Writes to the register.
    ///
    /// # Arguments
    /// `value` - The value to write to the register.
    pub fn write(&self, value: T) {
        self.value.set(value);
    }

    /// Performs a read-modify-write of the register.
    ///
    /// # Arguments
    /// `modify_func` - A function to modify the register value.
    pub fn modify<F: FnOnce(T) -> T>(&self, modify_func: F) {
        let modified_value = modify_func(self.read());
        self.write(modified_value);
    }
}

pub const PERIPHERAL_BASE: u32 = 0x4000_0000;
pub const PERIPHERAL_END: u32 = 0x400F_FFFF;

pub const PERIPHERAL_BITBAND_BASE: u32 = 0x4200_0000;
pub const PERIPHERAL_BITBAND_END: u32 = 0x43FF_FFFF;
