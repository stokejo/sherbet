#![no_std]
#![allow(clippy::option_map_unit_fn)]
extern crate alloc;
use core::panic::PanicInfo;

pub mod allocator;
pub mod clock;
pub mod interface;
pub mod scheduler;

pub unsafe fn reloc() {
	extern "C" {
		static _data_la: u8;
		static mut _data_va: u8;
		static _data_hi: u8;
		static mut _wipe_va: u8;
		static _wipe_hi: u8;
	}
	let hi = &raw const _data_hi;
	let lo = &raw mut _data_va;

	let count = hi.byte_offset_from_unsigned(lo);
	core::ptr::copy_nonoverlapping(
		&raw const _data_la,
		&raw mut _data_va,
		count,
	);
	let hi = &raw const _wipe_hi;
	let lo = &raw mut _wipe_va;

	let count = hi.byte_offset_from_unsigned(lo);
	core::ptr::write_bytes(lo, 0, count);
}

#[panic_handler]
fn panic(_panic: &PanicInfo<'_>) -> ! {
	loop {}
}
