//! # Port
//! The `port` module includes structures and functions to abstract ports as software resources.

//
// Dependencies
//

use crate::Half;

use super::{PortName, PortSize, PORT_PINS_AVAILABLE};
use core::sync::atomic::Ordering;

//
// Constants
//

/// The number of pins in a port.
const NUM_PINS_IN_PORT: usize = 16;
const HALF_PINS_IN_PORT: usize = NUM_PINS_IN_PORT / 2;

/// A mask that represents all pins in the port.
const ALL_PINS_IN_PORT: u16 = (((1 as usize) << NUM_PINS_IN_PORT) - 1) as u16;
const LOWER_PINS_IN_PORT: u16 = (((1 as usize) << HALF_PINS_IN_PORT) - 1) as u16;
const UPPER_PINS_IN_PORT: u16 = ALL_PINS_IN_PORT - LOWER_PINS_IN_PORT;

//
// Structures
//

/// Represents a port on the MCU.
pub struct Port {
    /// The unique identifier for the port.
    port_name: PortName,
}

impl Port {
    /// Creates a new Port structure.
    ///
    /// # Arguments
    /// `port` - Provides the unique identifier for the port to be created.
    ///
    /// # Returns
    /// Some(Port) - If all pins in the port are available.
    /// None - If any pins within the port are already in use.
    pub fn new(port: PortName) -> Option<Self> {
        let port_index = port.get_16_bit_index();
        let required_pins = match port.get_size() {
            PortSize::Port8Bit => {
                if port.is_upper_half_port() {
                    UPPER_PINS_IN_PORT
                } else {
                    LOWER_PINS_IN_PORT
                }
            }
            PortSize::Port16Bit => ALL_PINS_IN_PORT,
        };
        let value = unsafe {
            PORT_PINS_AVAILABLE
                .get_unchecked_mut(port_index)
                .fetch_and(!required_pins, Ordering::Relaxed)
        };

        if (value & required_pins) == required_pins {
            return Some(Port { port_name: port });
        }

        // Set back all of the pins that were available. Do not touch the other bits that were not
        // available as they may be touched by the owning thread, causing a race condition.
        unsafe {
            PORT_PINS_AVAILABLE
                .get_unchecked_mut(port_index)
                .fetch_or(value & required_pins, Ordering::Relaxed)
        };

        None
    }

    /// Gets the unique identifier for this port.
    ///
    /// # Returns
    /// PortName
    pub fn get_name(&self) -> PortName {
        self.port_name
    }

    /// Gets the port index from a port.
    ///
    /// # Returns
    /// The port index.

    pub fn get_index(&self) -> usize {
        self.get_name().get_index()
    }

    /// Calculates the 16-bit port number from a port.
    ///
    /// # Returns
    /// The 16-bit port number.
    pub fn get_16_bit_index(&self) -> usize {
        self.get_name().get_16_bit_index()
    }

    /// Calculates the 8-bit port number from a port
    ///
    /// # Arguments
    /// `half` - Provides the half to calculate. Ignored if the port is an 8-bit port.
    ///
    /// # Returns
    /// The 8-bit port number.
    pub fn get_8_bit_index(&self, half: Half) -> usize {
        self.get_name().get_8_bit_index(half)
    }

    /// Determines the size of a port.
    ///
    /// # Returns
    /// The port size.
    pub fn get_size(&self) -> PortSize {
        self.get_name().get_size()
    }
}

impl Drop for Port {
    /// Drops the Port structure and marks all the pins as available.
    fn drop(&mut self) {
        let port_index = self.get_16_bit_index();
        let required_pins = match self.get_size() {
            PortSize::Port8Bit => {
                if self.port_name.is_upper_half_port() {
                    UPPER_PINS_IN_PORT
                } else {
                    LOWER_PINS_IN_PORT
                }
            }
            PortSize::Port16Bit => ALL_PINS_IN_PORT,
        };

        unsafe {
            PORT_PINS_AVAILABLE
                .get_unchecked_mut(port_index)
                .fetch_or(required_pins, Ordering::Relaxed);
        }
    }
}
