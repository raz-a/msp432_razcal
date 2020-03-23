
use super::GpioPort;
use crate::peripheral_to_alias;
use crate::pin::Pin;

mod output;
pub use output::*;

pub trait GpioSingle {
    fn new(pin: Pin) -> Self;
    fn get_current_state(&self) -> bool;
}

fn set_pin_function_to_gpio(port: &mut GpioPort, pin_offset: u8) {
    // Set function select bits to 00 (GPIO).
    let sel0_addr =
        peripheral_to_alias(((&mut port.select_0) as *mut u16) as u32, pin_offset);

    let sel0_reg = unsafe {
        &mut *(sel0_addr as *mut u16)
    };

    let sel1_addr =
        peripheral_to_alias(((&mut port.select_1) as *mut u16) as u32, pin_offset);

    let sel1_reg = unsafe {
        &mut *(sel1_addr as *mut u16)
    };

    let select_status = (*sel1_reg << 1) | *sel0_reg;
    match select_status {
        1 => {
            // Clear Select 0.
            *sel0_reg = 0;
        },

        2 => {
            // Clear Select 1.
            *sel1_reg = 0;
        },

        3 => {
            // Use the Select Compliment register to ensure atomic clearing of both Select 0 and
            // Select 1.

            let selc_addr = peripheral_to_alias(
                                ((&mut port.compliment_selection) as *mut u16) as u32,
                                pin_offset);
        },

        _ => {
            debug_assert_eq!(select_status, 0);
        }
    }
}
