//! # Port Section
//! The `port_section` module includes structures and functions to abstract contiguous sections of
//! ports as software resources.

use paste::paste;
use seq_macro::seq;

use super::Pin;

//
// Traits
//

/// Describes a contiguous section of a port.
pub trait PortSectionX: private::Sealed {
    /// Gets the name of the port this section belongs to.
    ///
    /// # Returns
    /// Port name.
    fn get_port_name(&self) -> char;

    /// Gets the size of the section, in number of bits.
    ///
    /// # Returns
    /// Section size.
    fn get_size(&self) -> usize;

    /// Gets the offset within the port that the section starts in.
    ///
    /// # Returns
    /// Section offset.
    fn get_offset(&self) -> usize;

    /// Gets the bit nask that represents the section within the port.
    ///
    /// # Returns
    /// Section mask.
    fn get_mask(&self) -> usize {
        (self.get_size() - 1) << self.get_offset()
    }
}

//
// Structures.
//

macro_rules! define_port_section {
    ($count:literal) => {
        paste! {
            seq!(N in 0..$count {
                #[doc = "Represents a coniguous group of " $count " pins within the same port."]
                pub struct [<PortSection $count>]<const PORT_NAME: char, const OFFSET: usize> where
                    #([(); OFFSET + N]: ,)*
                {
                    #(_pin#N: Pin<PORT_NAME, { OFFSET + N }>,)*
                }

                impl<const PORT_NAME: char, const OFFSET: usize> [<PortSection $count>]<PORT_NAME, OFFSET> where
                    #([(); OFFSET + N]: ,)*
                {
                    /// Creates a new `port_section` structure.
                    ///
                    /// # Arguments
                    /// `pin[N]` - Pin `N` for the section to be created.
                    ///
                    /// # Returns
                    /// Port Section.
                    pub fn new(#(pin#N: Pin<PORT_NAME, { OFFSET + N }>,)*) -> Self {
                        Self {
                            #(_pin#N: pin#N,)*
                        }
                    }

                    /// Reverts the port section back to its containing pins.
                    ///
                    /// # Returns
                    /// The pins contained by the port section.
                    pub fn to_pins(self) -> (#(Pin<PORT_NAME, {OFFSET + N}>,)*) {
                        (#(self._pin#N,)*)
                    }
                }

                impl<const PORT_NAME: char, const OFFSET: usize> PortSectionX for [<PortSection $count>]<PORT_NAME, OFFSET> where
                    #([(); OFFSET + N]: ,)*
                {

                    /// Gets the name of the port this section belongs to.
                    ///
                    /// # Returns
                    /// Port name.
                    fn get_port_name(&self) -> char {
                        PORT_NAME
                    }

                    /// Gets the size of the section, in number of bits.
                    ///
                    /// # Returns
                    /// Section size.
                    fn get_size(&self) -> usize {
                        $count
                    }

                    /// Gets the offset within the port that the section starts in.
                    ///
                    /// # Returns
                    /// Section offset.
                    fn get_offset(&self) -> usize {
                        OFFSET
                    }
                }

                impl<const PORT_NAME: char, const OFFSET: usize> private::Sealed for [<PortSection $count>]<PORT_NAME, OFFSET> where
                    #([(); OFFSET + N]: ,)*
                {
                }
            });
        }
    };
}

seq!(N in 2..16 {
    define_port_section!(N);
});

//
// For sealed traits.
//

mod private {
    pub trait Sealed {}
}
