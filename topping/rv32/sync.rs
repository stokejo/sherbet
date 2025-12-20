use core::arch::asm;
use critical_section::{Impl, RawRestoreState};

use crate::interrupt;

// =========================================================================

pub struct SingleHartCriticalSection;

unsafe impl Impl for SingleHartCriticalSection {
	unsafe fn acquire() -> RawRestoreState {
		let mut mstatus: usize = 0;
		unsafe { asm!("csrrci {}, mstatus, 0b1000", out(reg) mstatus) };
		((mstatus & 0b1000) != 0).into()
	}

	unsafe fn release(restore_state: RawRestoreState) {
		restore_state.then(|| unsafe { interrupt::enable() });
	}
}
