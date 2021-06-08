//! # Port
//! The `port` module includes structures and functions to abstract ports as software resources.

//TODO: 8-bit ports?

//
// Dependencies
//

use super::Pin;
use paste::paste;
use seq_macro::seq;

//
// Traits
//

/// Describes a port that can be identified by its port name.
pub trait PortId: private::Sealed {
    /// Gets the name of this port.
    ///
    /// # Returns
    /// Port name.
    fn get_port_name(&self) -> char;

    /// Gets the size of this port.
    ///
    /// # Returns
    /// Port size.
    fn get_port_size(&self) -> usize;
}

/// A trait that is a shorthabd for the `Port<...>` structure.
pub trait PortX: private::Sealed + PortId {}

//
// Structures
//

seq!(N in 0..16 {

    /// Represents a port on the MCU.
    pub struct Port<const PORT_NAME: char> {
        #(_pin#N: Pin<PORT_NAME, N>,)*
    }

    impl<const PORT_NAME: char> Port<PORT_NAME> {
        /// Creates a new Port structure.
        ///
        /// # Arguments
        /// `pin[N]` - Pin `N` for the port to be created.
        ///
        pub fn new(
            #(pin#N: Pin<PORT_NAME, N>,)*
        ) -> Self {
            Self {
                #(_pin#N: pin#N,)*
            }
        }

        pub fn to_pins(self) -> (#(Pin<PORT_NAME, N>,)*) {
            (#(self._pin#N,)*)
        }
    }

    impl<const PORT_NAME: char> PortId for Port<PORT_NAME> {
        /// Gets the name of this port.
        ///
        /// # Returns
        /// Port name.
        fn get_port_name(&self) -> char {
            PORT_NAME
        }

        /// Gets the size of this port.
        ///
        /// # Returns
        /// Port size.
        fn get_port_size(&self) -> usize {
            16
        }
    }

    impl<const PORT_NAME: char> PortX for Port<PORT_NAME> {}
});

macro_rules! define_port_section {
    ($count:literal) => {
        paste! {
            seq!(N in 0..$count {
                /// Represents a coniguous group of N pins within the same port.
                pub struct [<PortSection $count>]<const PORT_NAME: char, const OFFSET: usize>
                where
                    #([(); OFFSET + N]: ,)*
                {
                    #(_pin#N: Pin<PORT_NAME, { OFFSET + N }>,)*
                }
            });
        }
    };
}

seq!(N in 1..16 {
    define_port_section!(N);
});

//
// For sealed traits.
//

mod private {
    pub trait Sealed {}
}

impl<const PORT_NAME: char> private::Sealed for Port<PORT_NAME> {}
