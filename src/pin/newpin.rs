//! # Pin
//! The `pin` module includes structures and functions to abstract pins as software resources.

use core::marker::PhantomData;
use paste::paste;

pub struct PinX<const PORT_NAME: char, const OFFSET: u8> {
    _marker: PhantomData<()>,
}

impl<const PORT_NAME: char, const OFFSET: u8> PinX<PORT_NAME, OFFSET> {
    fn new() -> Self {
        PinX {
            _marker: Default::default(),
        }
    }

    pub fn get_port_name(&self) -> char {
        PORT_NAME
    }

    pub fn get_offset(&self) -> u8 {
        OFFSET
    }
}

macro_rules! define_pinset {
    ($(($port:tt, $port_char:literal, $($pin:literal),+)),+) => {
        paste! {
            pub struct SystemPinSet {
                $(
                    $(
                        pub [<p $port $pin>]: PinX<$port_char , $pin>,
                    )*
                )*
            }

            impl SystemPinSet {
                pub fn new() -> Self {
                    Self {
                        $(
                            $(
                                [<p $port $pin>]: PinX::new(),
                            )*
                        )*
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
