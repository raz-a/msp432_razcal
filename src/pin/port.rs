//! # Port
//! The `port` module includes structures and functions to abstract ports as software resources.

//TODO: 8-bit ports?

//
// Dependencies
//

use super::Pin;
use seq_macro::seq;

//
// Traits
//

/// Describes a port that can be identified by its port name.
pub trait IdentifiablePort: private::Sealed {
    /// Gets the name of this port.
    ///
    /// # Returns
    /// Port name.
    fn get_port_name(&self) -> char;

    /// Gets the size of this port.
    ///
    /// # Returns
    /// Port size.
    fn get_port_size(&self) -> u8;
}

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
    }

    impl<const PORT_NAME: char> IdentifiablePort for Port<PORT_NAME> {
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
        fn get_port_size(&self) -> u8 {
            16
        }
    }
});

//
// For sealed traits.
//

mod private {
    pub trait Sealed {}
}

impl<const PORT_NAME: char> private::Sealed for Port<PORT_NAME> {}
