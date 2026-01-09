#![no_std]
use {core::hint, sherbet::scheduler};

pub async fn main() {
	loop {
		scheduler::yield_now().await;
		hint::spin_loop();
	}
}
