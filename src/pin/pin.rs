//! # Pin
//! The `pin` module includes structures and functions to abstract pins as software resources.

//
// Dependencies
//

use super::{PinName, PinOffset, PortName, PORT_PINS_AVAILABLE};
use core::sync::atomic::Ordering;

//
// Structures
//

/// Represents a pin to the MCU.
pub struct Pin {
    /// The unique identifier for the pin.
    pin: PinName,
}

impl Pin {
    /// Creates a new Pin structure.
    ///
    /// # Arguments
    /// `pin` - Provides the unique identifier for the pin to be created.
    ///
    /// # Returns
    /// Some(Pin) - If the pin is available.\
    /// None - If a pin with the same PinName already exists.
    pub fn new(pin: PinName) -> Option<Self> {
        let port = pin.port_name as usize;
        let pin_mask = 1 << (pin.pin_offset as usize);
        let value = unsafe {
            PORT_PINS_AVAILABLE
                .get_unchecked_mut(port)
                .fetch_nand(pin_mask, Ordering::Relaxed)
        };

        if value & pin_mask == 0 {
            return None;
        }

        Some(Pin { pin: pin })
    }

    /// Gets the uniquie identifier for this pin.
    ///
    /// # Returns
    /// PinName
    pub fn get_pin(&self) -> PinName {
        self.pin
    }

    /// Gets the port that this pin belongs to.
    ///
    /// # Returns
    /// PortName
    pub fn get_port(&self) -> PortName {
        self.pin.port_name
    }

    /// Gets this pin's offset in the port it belongs to.
    ///
    /// # Returns
    /// PinOffset
    pub fn get_pin_offset_in_port(&self) -> PinOffset {
        self.pin.pin_offset
    }
}

impl Drop for Pin {
    /// Drops the Pin structure and marks the pin as available.
    fn drop(&mut self) {
        let port = self.pin.port_name as usize;
        let pin_mask = 1 << (self.pin.pin_offset as usize);
        unsafe {
            PORT_PINS_AVAILABLE
                .get_unchecked_mut(port)
                .fetch_or(pin_mask, Ordering::Relaxed);
        }
    }
}
