#[cfg(feature = "interrupt")]
#[unsafe(no_mangle)]
unsafe extern "C" fn required_interrupt() {
	loop {}
}

#[cfg(feature = "interrupt")]
core::arch::global_asm!(
	".section .text.interrupt",
	".weak __EXCEPTION_AND_INTERRUPT_VECTOR_TABLE",
	".option push",
	".option norelax",
	".option norvc",
	"__EXCEPTION_AND_INTERRUPT_VECTOR_TABLE:",
	"j required_interrupt",
	".option pop",
);

pub unsafe fn setup() {
	unsafe {
		core::arch::asm!(
			//
			// We set mtvec[0]=1 so that processor use vector table.
			//
			"la t0, _TRAP_VECTOR_BASE_ADDR",
			"addi t0, t0, 1",
			"csrw mtvec, t0",
			out("t0") _,
		);
	}
}

pub unsafe fn allow() {
	unsafe { core::arch::asm!("csrs mstatus, 0b1000") };
}
