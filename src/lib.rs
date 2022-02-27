#![no_std]
#![feature(generic_const_exprs)]
#![feature(generic_arg_infer)]
#![allow(incomplete_features)]
#![allow(dead_code)]

mod registers;

pub mod gpio;
pub mod interrupt;
pub mod pin;
pub mod spi;
pub mod watchdog;

pub enum Edge {
    RisingEdge,
    FallingEdge,
}
