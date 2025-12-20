#![no_main]
#![no_std]
extern crate alloc;
extern crate rv32;
extern crate sherbet;

use qingke_v2a::interrupt;

#[no_mangle]
extern "C" fn sherbet_user_provided_entry() -> ! {
	unsafe {
		interrupt::setup();
		sherbet::reloc();
	}
	loop {}
}
