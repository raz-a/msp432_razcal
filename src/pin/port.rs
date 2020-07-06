//! # Port
//! The `port` module includes structures and functions to abstract ports as software resources.

//
// Dependencies
//

use super::{PortName, PORT_PINS_AVAILABLE};
use core::sync::atomic::Ordering;

//
// Constants
//

const NUM_PINS_IN_PORT: usize = 16;
const ALL_PINS_IN_PORT: u16 = (((1 as usize) << NUM_PINS_IN_PORT) - 1) as u16;

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
    /// Some(Port) - If all pins in the port are available.\
    /// None - If any pins within the port are already in use.
    pub fn new(port: PortName) -> Option<Self> {
        let port_index = port as usize;
        let value = unsafe {
            PORT_PINS_AVAILABLE
                .get_unchecked_mut(port_index)
                .fetch_nand(ALL_PINS_IN_PORT, Ordering::Relaxed)
        };

        if value == ALL_PINS_IN_PORT {
            return Some(Port { port_name: port });
        }

        // Set back all of the pins that were available. Do not touch the other bits that were not
        // available as they may be touched by the owning thread, causing a race condition.
        unsafe {
            PORT_PINS_AVAILABLE
                .get_unchecked_mut(port_index)
                .fetch_or(value, Ordering::Relaxed)
        };

        None
    }

    /// Gets the uniquie identifier for this port.
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
        NUM_PINS_IN_PORT
    }
}

impl Drop for Port {
    /// Drops the Port structure and marks all the pins as available.
    fn drop(&mut self) {
        let port_index = self.port_name as usize;
        unsafe {
            PORT_PINS_AVAILABLE
                .get_unchecked_mut(port_index)
                .fetch_or(ALL_PINS_IN_PORT, Ordering::Relaxed);
        }
    }
}
