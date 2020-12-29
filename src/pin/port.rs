//! # Port
//! The `port` module includes structures and functions to abstract ports as software resources.

//
// Dependencies
//

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
    /// The unique identifier for the port/
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
        let port_index = port.get_16_bit_port_index();
        let required_pins = match port.size {
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

    /// Gets the number of pins in this port.
    ///
    /// # Returns
    /// Number of pins in the port.
    pub fn get_number_of_pins(&self) -> usize {
        match self.port_name.size {
            PortSize::Port8Bit => HALF_PINS_IN_PORT,
            PortSize::Port16Bit => NUM_PINS_IN_PORT,
        }
    }
}

impl Drop for Port {
    /// Drops the Port structure and marks all the pins as available.
    fn drop(&mut self) {
        let port_index = self.port_name.get_16_bit_port_index();
        let required_pins = match self.port_name.size {
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
