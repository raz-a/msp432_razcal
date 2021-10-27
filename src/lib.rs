#![no_std]
#![feature(asm)]
#![feature(generic_const_exprs)]
#![allow(dead_code)]

mod registers;

pub mod gpio;
pub mod interrupt;
pub mod pin;
pub mod watchdog;
