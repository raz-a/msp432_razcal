#![no_std]
#![feature(asm)]
#![feature(generic_const_exprs)]
#![feature(generic_arg_infer)]
#![allow(dead_code)]

mod registers;

pub mod gpio;
pub mod interrupt;
pub mod pin;
pub mod watchdog;
