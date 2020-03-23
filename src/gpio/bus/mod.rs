use crate::pin::Pin;

pub trait GpioBus<const WIDTH: usize> {
    fn new(pins: [Pin; WIDTH]) -> Self;
    fn get_current_state(&self) -> u32;
}

pub trait GpioSparseBus<const WIDTH: usize> {
    fn new(pins: [Pin; WIDTH]) -> Self;
    fn get_current_state(&self) -> u32;
}
