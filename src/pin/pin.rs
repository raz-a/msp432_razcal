//! # Pin
//! The `pin` module includes structures and functions to abstract pins as software resources.

//
// Dependencies
//

use crate::Half;

use super::{PinName, PortName, PortSize, PORT_PINS_AVAILABLE};
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
        // Get the 16-bit port identifier to make PINS_AVAILABLE calculation easier.
        let port = pin.get_owning_port_16_bit_index();
        let pin_mask = 1 << pin.get_owning_port_16_bit_index();
        let value = unsafe {
            PORT_PINS_AVAILABLE
                .get_unchecked_mut(port)
                .fetch_and(!pin_mask, Ordering::Relaxed)
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
    pub fn get_name(&self) -> PinName {
        self.pin
    }

    /// Gets the owning port name.
    ///
    /// # Returns
    /// The owning port name.
    pub fn get_owning_port_name(&self) -> PortName {
        self.get_name().get_owning_port_name()
    }

    /// Gets the port index from the owning port.
    ///
    /// # Returns
    /// The port index.

    pub fn get_owning_port_index(&self) -> usize {
        self.get_name().get_owning_port_index()
    }

    /// Calculates the 16-bit port number from the owning port.
    ///
    /// # Returns
    /// The 16-bit port number.
    pub fn get_owning_port_16_bit_index(&self) -> usize {
        self.get_name().get_owning_port_16_bit_index()
    }

    /// Calculates the 8-bit port number from the owning port
    ///
    /// # Arguments
    /// `half` - Provides the half to calculate. Ignored if the port is an 8-bit port.
    ///
    /// # Returns
    /// The 8-bit port number.
    pub fn get_owning_port_8_bit_index(&self, half: Half) -> usize {
        self.get_name().get_owning_port_8_bit_index(half)
    }

    /// Determines the size of a port represented by the owning port.
    ///
    /// # Returns
    /// The port size of the owning port.
    pub fn get_owning_port_size(&self) -> PortSize {
        self.get_name().get_owning_port_size()
    }

    /// Gets the pin's offset within it's owning port.
    ///
    /// # Returns
    /// The pin offset for the pin.
    pub fn get_offset(&self) -> usize {
        self.get_name().get_offset()
    }

    /// Gets the pin's offset within it's owning 8-bit port.
    ///
    /// # Returns
    /// The pin offset for the pin.
    pub fn get_8_bit_port_offset(&self) -> usize {
        self.get_name().get_8_bit_port_offset()
    }

    /// Gets the pin's offset within it's owning 16-bit port.
    ///
    /// # Returns
    /// The pin offset for the pin.
    pub fn get_16_bit_port_offset(&self) -> usize {
        self.get_name().get_16_bit_port_offset()
    }
}

impl Drop for Pin {
    /// Drops the Pin structure and marks the pin as available.
    fn drop(&mut self) {
        // Get the 16-bit port identifier to make PINS_AVAILABLE calculation easier.
        let port = self.get_owning_port_16_bit_index();
        let pin_mask = 1 << self.get_16_bit_port_offset();
        unsafe {
            PORT_PINS_AVAILABLE
                .get_unchecked_mut(port)
                .fetch_or(pin_mask, Ordering::Relaxed);
        }
    }
}
