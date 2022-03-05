//! # Registers
//! The `registers` module includes structure to properly interact with registers as well as
//! constants that hold the location of memory mapped registers.

use core::{
    marker::PhantomData,
    ops::{BitAnd, BitOr, Not},
};
use vcell::VolatileCell;

pub const PERIPHERAL_BASE: u32 = 0x4000_0000;
pub const PERIPHERAL_END: u32 = 0x400F_FFFF;

pub const PERIPHERAL_BITBAND_BASE: u32 = 0x4200_0000;
pub const PERIPHERAL_BITBAND_END: u32 = 0x43FF_FFFF;

fn peripheral_to_bitband_alias(address: u32, bit: u8) -> u32 {
    debug_assert!(address >= PERIPHERAL_BASE);
    debug_assert!(address <= PERIPHERAL_END);

    let byte_offset = address - PERIPHERAL_BASE;
    PERIPHERAL_BITBAND_BASE + (byte_offset * 32) + ((bit as u32) * 4)
}

//
// Trait to define allowed register primitives.
//

pub trait RegBase:
    BitAnd<Output = Self>
    + BitOr<Output = Self>
    + Copy
    + Default
    + From<bool>
    + Not<Output = Self>
    + PartialEq
    + Sized
{
}

impl RegBase for u8 {}
impl RegBase for u16 {}
impl RegBase for u32 {}

//
// Register access mode.
//

/// Represents the access mode for a register.
pub trait AccessMode: private::Sealed {}
pub trait ReadAccessMode: AccessMode {}
pub trait WriteAccessMode: AccessMode {}

/// Represents a register that cannot be accessed.
pub struct NoAccess;
impl AccessMode for NoAccess {}

/// Represents a read-only register.
pub struct ReadOnlyAccess;
impl AccessMode for ReadOnlyAccess {}
impl ReadAccessMode for ReadOnlyAccess {}

/// Represents a write-only register.
pub struct WriteOnlyAccess;
impl AccessMode for WriteOnlyAccess {}
impl WriteAccessMode for WriteOnlyAccess {}

/// Represents a read/write register.
pub struct ReadWriteAccess;
impl AccessMode for ReadWriteAccess {}
impl ReadAccessMode for ReadWriteAccess {}
impl WriteAccessMode for ReadWriteAccess {}

//
// Base register type.
//

pub struct Reg<Mode: AccessMode, T: RegBase, const BIT_BAND: bool> {
    value: VolatileCell<T>,
    _mode: PhantomData<Mode>,
}

//
// Register type aliases.
//

pub type Reserved<T> = Reg<NoAccess, T, false>;
pub type ReadOnly<T> = Reg<ReadOnlyAccess, T, false>;
pub type WriteOnly<T> = Reg<WriteOnlyAccess, T, false>;
pub type ReadWrite<T> = Reg<ReadWriteAccess, T, false>;
pub type BitBandReadOnly<T> = Reg<ReadOnlyAccess, T, true>;
pub type BitBandWriteOnly<T> = Reg<WriteOnlyAccess, T, true>;
pub type BitBandReadWrite<T> = Reg<ReadWriteAccess, T, true>;

//
// Register implementations.
//

impl<Mode: AccessMode, T: RegBase> Reg<Mode, T, false> {
    /// Gets the bit-banded alias for the register.
    ///
    /// # Arguments
    /// `bit` - the bit within the register to get the bit-banded alias for.
    ///
    /// # Returns
    /// The bit-banded alias register.
    pub fn get_bitband(&self, bit: u8) -> &Reg<Mode, T, true> {
        let bitband_address = peripheral_to_bitband_alias(self.value.as_ptr() as u32, bit);
        unsafe { &*(bitband_address as *const Reg<Mode, T, true>) }
    }
}

impl<Mode: ReadAccessMode, T: RegBase> Reg<Mode, T, false> {
    /// Reads the register.
    ///
    /// # Returns
    /// The value of the register.
    pub fn read(&self) -> T {
        self.value.get()
    }
}

impl<Mode: WriteAccessMode, T: RegBase> Reg<Mode, T, false> {
    /// Writes to the register.
    ///
    /// # Arguments
    /// `value` - The value to write to the register.
    pub fn write(&self, value: T) {
        self.value.set(value);
    }
}

impl<T: RegBase> ReadWrite<T> {
    /// Performs a read-modify-write of the register.
    ///
    /// # Arguments
    /// `modify_func` - A function to modify the register value.
    pub fn modify<F: FnOnce(T) -> T>(&self, modify_func: F) {
        let modified_value = modify_func(self.read());
        self.write(modified_value);
    }

    /// Clears bits based on the provided mask.
    ///
    /// # Arguments
    /// `mask` - Provides the bits to clear.
    pub fn clear_bits(&self, mask: T) {
        self.modify(|value| value & !mask);
    }

    /// Sets bits based on the provided mask.
    ///
    /// # Arguments
    /// `mask` - Provides the bits to set.
    pub fn set_bits(&self, mask: T) {
        self.modify(|value| value | mask);
    }
}

//
// Bit-Banded Register aliases.
//

impl<Mode: ReadAccessMode, T: RegBase> Reg<Mode, T, true> {
    /// Reads the register.
    ///
    /// # Returns
    /// The value of the register.
    pub fn read(&self) -> bool {
        self.value.get() != T::default()
    }
}

impl<Mode: WriteAccessMode, T: RegBase> Reg<Mode, T, true> {
    /// Writes to the register.
    ///
    /// # Arguments
    /// `value` - The value to write to the register.
    pub fn write(&self, value: bool) {
        self.value.set(T::from(value));
    }
}

impl<T: RegBase> BitBandReadWrite<T> {
    /// Performs a read-modify-write of the register.
    ///
    /// # Arguments
    /// `modify_func` - A function to modify the register value.
    pub fn modify<F: FnOnce(bool) -> bool>(&self, modify_func: F) {
        let modified_value = modify_func(self.read());
        self.write(modified_value);
    }

    /// Performs a read-modify-write of the register.
    ///
    /// # Arguments
    /// `modify_func` - A function to modify the register value.
    ///
    ///  # UNSAFE
    ///  This function takes the raw register value and does not check if it is only modifying
    ///  the bitband lowest bit.
    pub unsafe fn modify_raw<F: FnOnce(T) -> T>(&self, modify_func: F) {
        let modified_value = modify_func(self.value.get());
        self.value.set(modified_value);
    }
}

//
// For sealed traits.
//

mod private {
    pub trait Sealed {}
}

impl private::Sealed for NoAccess {}
impl private::Sealed for ReadOnlyAccess {}
impl private::Sealed for WriteOnlyAccess {}
impl private::Sealed for ReadWriteAccess {}
