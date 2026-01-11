use core::{arch, ptr};
pub use rv32::interrupt::allow;

// =========================================================================

unsafe extern "C" fn clock_tick_interrupt() {
	sherbet::clock::ticka();
	(super::clock::reset(), arch::asm!("addi sp, sp, 4", "mret"));
}

unsafe extern "C" fn required_interrupt() {
	loop {}
}

// =========================================================================

//
// Note 1:
// This should be placed at 0x00000004, because we place
// "j start" instead of interrupt handler at 0x00000000.
//
// Note 2:
// We set mtvec[1]=1 so that we can store absolute address
// of the interrupt handling function at the vector table.
//
#[used]
#[link_section = ".text.interrupt"]
#[no_mangle]
static __EXCEPTION_AND_INTERRUPT_VECTOR_TABLE: [Option<
	unsafe extern "C" fn(),
>; 15] = [
	None, // int 1
	Some(required_interrupt),
	Some(required_interrupt),
	None,
	None,
	None,
	None,
	None,
	None,
	None,
	None,
	Some(clock_tick_interrupt),
	None,
	None,
	None,
];

// =========================================================================

pub unsafe fn setup() {
	arch::asm!(
		//
		// We set mtvec[1]=1 so that we can store absolute address
		// of the interrupt handling function at the vector table,
		// also set mtvec[0]=1 so that processor use vector table.
		//
		"la t0, _TRAP_VECTOR_BASE_ADDR",
		"addi t0, t0, 0x3",
		"csrw mtvec, t0",
		//
		// Enable fast interrupts.
		//
		"li t0, 0x3",
		"csrw 0x804, t0",
		out("t0") _,
	)
}

// =========================================================================

const PFIC_IENR1: *mut u32 = 0xE000E100 as *mut u32;
pub const ALLOW: bool = true;

pub unsafe fn set(interrupt: u32, allow: bool) {
	let offset = (interrupt / 32) as isize;
	let bit = interrupt % 32;
	unsafe {
		ptr::write_volatile(
			PFIC_IENR1.offset(offset),
			(allow as u32) << bit,
		);
	}
}
