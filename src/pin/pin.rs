use super::{extract_pin_number, extract_port_number, PinName, PORT_PINS_AVAILABLE};
use core::sync::atomic::Ordering;

pub struct Pin {
    pin: PinName,
}

impl Pin {
    pub fn new(pin: PinName) -> Option<Self> {
        let port = extract_port_number(pin) as usize;
        let pin_mask = 1 << extract_pin_number(pin);
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

    pub fn get_pin(&self) -> PinName {
        self.pin
    }

    pub fn get_port(&self) -> u8 {
        extract_port_number(self.pin)
    }

    pub fn get_pin_offset_in_port(&self) -> u8 {
        extract_pin_number(self.pin)
    }
}

impl Drop for Pin {
    fn drop(&mut self) {
        let port = extract_port_number(self.pin) as usize;
        let pin_mask = 1 << extract_pin_number(self.pin);
        unsafe {
            PORT_PINS_AVAILABLE
                .get_unchecked_mut(port)
                .fetch_or(pin_mask, Ordering::Relaxed);
        }
    }
}
