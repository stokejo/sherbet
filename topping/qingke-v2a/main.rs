#![no_std]
use critical_section::set_impl;
use rv32::sync::SingleHartCriticalSection;

pub mod interrupt;
set_impl!(SingleHartCriticalSection);
