//! # Pin
//! The `pin` module includes structures and functions to abstract pins as software resources.

//
// TODO: Pin implies default (GPIO mode)
//

//
// TODO: Crate public "AlternatePin" type. <- Implements PinId + PinIdWithMode
//

//
// TODO: Macro that implements "ToAlternate" functions for the correct pins.
//

//
// Dependencies.
//

use core::marker::PhantomData;
use paste::paste;

use super::PortComponent;

/// Describes a pin that can be identified by its port and pin offset.
pub trait PinId: private::Sealed + PortComponent {
    /// Gets the name of the port this pin belongs to.
    ///
    /// # Returns
    /// Port name.
    fn get_port_name(&self) -> char;

    /// Gets the offset of this pin within its owning port.
    ///
    /// # Returns
    /// Offset.
    fn get_offset(&self) -> u8;
}

//
// Main Pin structure.
//

/// A trait that is a shorthand for the `Pin<...>` structure.
pub trait PinX: private::Sealed + PinId {}

// - Private Note -
// The PinX trait also differentiates the main Pin structure from the the alternate pin structures.

/// Represents a pin on the MCU.
pub struct Pin<const PORT_NAME: char, const OFFSET: u8> {
    _marker: PhantomData<()>,
}

impl<const PORT_NAME: char, const OFFSET: u8> Pin<PORT_NAME, OFFSET> {
    /// Creates a new Pin structure.
    ///
    /// # Returns
    /// The instantiated Pin.
    const fn new() -> Self {
        Pin {
            _marker: PhantomData {},
        }
    }
}

impl<const PORT_NAME: char, const OFFSET: u8> PortComponent for Pin<PORT_NAME, OFFSET> {
    fn get_port_mask(&self) -> u16 {
        1 << self.get_offset()
    }

    fn get_port_clear_mask(&self) -> u16 {
        !self.get_port_mask()
    }
}

impl<const PORT_NAME: char, const OFFSET: u8> PinId for Pin<PORT_NAME, OFFSET> {
    /// Gets the name of the port this pin belongs to.
    ///
    /// # Returns
    /// PortName
    fn get_port_name(&self) -> char {
        PORT_NAME
    }

    /// Gets the offset of this pin within its owning port.
    ///
    /// # Returns
    /// Offset
    fn get_offset(&self) -> u8 {
        OFFSET
    }
}

impl<const PORT_NAME: char, const OFFSET: u8> PinIdWithMode for Pin<PORT_NAME, OFFSET> {
    /// Gets the pin mode of the current pin.
    ///
    /// # Returns
    /// PinMode.
    fn get_mode(&self) -> PinMode {
        PinMode::DefaultGpio
    }
}

impl<const PORT_NAME: char, const OFFSET: u8> PinX for Pin<PORT_NAME, OFFSET> {}

macro_rules! define_pinset {
    ($(($port:tt, $port_char:literal, $($pin:literal),+)),+) => {
        paste! {

            /// Singleton holding all the available pins on the MCU.
            static mut MCU_PINSET: Option<McuPinSet> = Some(McuPinSet::init_mcu_pins());

            /// Represents all the available pins on the current MCU.
            pub struct McuPinSet {
                _marker: PhantomData<()>,

                $(
                    $(
                        pub [<p $port $pin>]: Pin<$port_char , $pin>,
                    )*
                )*
            }

            impl McuPinSet {
                /// Creates an McuPins structure.
                /// Should only be used to create the sinlgeton.
                ///
                /// # Returns
                /// McuPinSet
                const fn init_mcu_pins() -> Self {
                    Self {
                        _marker: PhantomData {},

                        $(
                            $(
                                [<p $port $pin>]: Pin::new(),
                            )*
                        )*
                    }
                }

                /// Gets the MCUPinSet structure.
                ///
                /// # Returns
                /// `Some(McuPinSet)` if this is the first attempt to aquire the pins.
                ///
                /// `None` otherwise.
                pub fn get_mcu_pins() -> Option<Self> {
                    unsafe {
                        MCU_PINSET.take()
                    }
                }
            }
        }
    };
}

#[cfg(razcal_msp432_package = "vqfn")]
define_pinset!(
    (a, 'A', 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11),
    (b, 'B', 0, 1, 2, 3, 4, 5, 6, 7, 10, 11, 12, 13, 14, 15),
    (c, 'C', 0, 1, 2, 3, 4, 5, 6, 7, 14, 15),
    (d, 'D', 0, 1, 2, 3, 4, 5, 6, 7, 8, 9),
    (j, 'J', 0, 1, 2, 3, 4, 5)
);

#[cfg(razcal_msp432_package = "nfbga")]
define_pinset!(
    (a, 'A', 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15),
    (b, 'B', 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15),
    (c, 'C', 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15),
    (d, 'D', 0, 1, 2, 3, 4, 5, 6, 7, 8, 9),
    (j, 'J', 0, 1, 2, 3, 4, 5)
);

#[cfg(razcal_msp432_package = "lqfp")]
define_pinset!(
    (a, 'A', 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15),
    (b, 'B', 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15),
    (c, 'C', 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15),
    (d, 'D', 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15),
    (e, 'E', 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15),
    (j, 'J', 0, 1, 2, 3, 4, 5)
);

//
// Alternate Pin Mode support.
//

/// Defines the possible modes for a pin.
pub(crate) enum PinMode {
    DefaultGpio = 0,
    Alternate1 = 1,
    Alternate2 = 2,
    Alternate3 = 3,
}

/// Extension to the PinId trait to include the pin mode.
pub(crate) trait PinIdWithMode: PinId + private::Sealed {
    /// Gets the pin mode of the current pin.
    ///
    /// # Returns
    /// PinMode.
    fn get_mode(&self) -> PinMode;
}

//
// For sealed traits.
//

mod private {
    pub trait Sealed {}
}

impl<const PORT_NAME: char, const OFFSET: u8> private::Sealed for Pin<PORT_NAME, OFFSET> {}
impl<const PORT_NAME: char, const OFFSET: u8> super::private::Sealed for Pin<PORT_NAME, OFFSET> {}
