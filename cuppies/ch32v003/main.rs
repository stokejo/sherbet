#![no_main]
#![no_std]
extern crate alloc;
extern crate rv32;
extern crate sherbet;

use alloc::boxed::Box;
use qingke_v2a::interrupt;
use sherbet::scheduler;

#[no_mangle]
extern "C" fn sherbet_user_provided_entry() -> ! {
	unsafe {
		interrupt::setup();
		sherbet::reloc();
		interrupt::allow();
	}
	let fut = Box::pin(idle::main());
	scheduler::block(fut);
	loop {}
}
