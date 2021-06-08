#![no_std]
#![feature(asm)]
#![feature(const_generics)]
#![feature(const_evaluatable_checked)]
#![allow(dead_code)]

mod registers;

pub mod gpio;
pub mod interrupt;
pub mod pin;
pub mod watchdog;
