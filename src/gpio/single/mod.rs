use crate::pin::Pin;

mod output;
pub use output::*;

pub trait GpioSingle {
    fn new(pin: Pin) -> Self;
    fn get_current_state(&self) -> bool;
}
