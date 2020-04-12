use super::pin::Pin;
use super::PinName;
use crate::assume_init_const_generic_array;
use core::mem::MaybeUninit;

pub struct PinSet<const COUNT: usize> {
    pins: [Pin; COUNT],
}

pub type PinSet2 = PinSet<2>;
pub type PinSet4 = PinSet<4>;
pub type PinSet8 = PinSet<8>;
pub type PinSet16 = PinSet<16>;
pub type PinSet32 = PinSet<32>;

impl<const COUNT: usize> PinSet<COUNT> {
    // TODO: Figure out how to make this as efficient as possible.
    pub fn new(pins: [PinName; COUNT]) -> Option<Self> {
        let mut pins_allocated = 0;
        let mut pin_set_uninit: [MaybeUninit<Pin>; COUNT] = MaybeUninit::uninit_array();
        for i in 0..COUNT {
            let allocated_pin = Pin::new(unsafe { *pins.get_unchecked(i) });
            match allocated_pin {
                Some(p) => {
                    let pin = unsafe { pin_set_uninit.get_unchecked_mut(i) };
                    *pin = MaybeUninit::new(p);
                    pins_allocated += 1;
                }
                None => {
                    break;
                }
            }
        }

        if pins_allocated == COUNT {
            return Some(PinSet {
                pins: assume_init_const_generic_array(pin_set_uninit),
            });
        }

        for pin in &mut pin_set_uninit[0..pins_allocated] {
            unsafe {
                core::ptr::drop_in_place(pin.as_mut_ptr());
            }
        }

        None
    }

    pub fn get_pins(&self) -> [PinName; COUNT] {
        let mut pins: [MaybeUninit<PinName>; COUNT] = MaybeUninit::uninit_array();
        let map = self.pins.iter().map(|pin| pin.get_pin());
        let mut i = 0;
        for pin in map {
            pins[i] = MaybeUninit::new(pin);
            i += 1;
        }

        assume_init_const_generic_array(pins)
    }

    pub fn get_ports(&self) -> [u8; COUNT] {
        let mut ports: [MaybeUninit<u8>; COUNT] = MaybeUninit::uninit_array();
        let map = self.pins.iter().map(|pin| pin.get_port());
        let mut i = 0;
        for port in map {
            ports[i] = MaybeUninit::new(port);
            i += 1;
        }

        assume_init_const_generic_array(ports)
    }

    pub fn get_pin_offsets_in_port(&self) -> [u8; COUNT] {
        let mut offsets: [MaybeUninit<u8>; COUNT] = MaybeUninit::uninit_array();
        let map = self.pins.iter().map(|pin| pin.get_pin_offset_in_port());
        let mut i = 0;
        for offset in map {
            offsets[i] = MaybeUninit::new(offset);
            i += 1;
        }

        assume_init_const_generic_array(offsets)
    }
}
