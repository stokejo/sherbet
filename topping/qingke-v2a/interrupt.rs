use core::{arch, ptr};
pub use rv32::interrupt::allow;

// =========================================================================

#[unsafe(no_mangle)]
unsafe extern "C" fn clock_tick_interrupt() {
	sherbet::clock::ticka();
	super::clock::reset();
}

#[unsafe(naked)]
unsafe extern "C" fn clock_tick_interrupt_wrapper() {
	arch::naked_asm!(
		"call clock_tick_interrupt",
		"mret" //
	)
}

#[unsafe(no_mangle)]
unsafe extern "C" fn required_interrupt() {
	loop {}
}

#[unsafe(naked)]
unsafe extern "C" fn required_interrupt_wrapper() {
	arch::naked_asm!(
		"call required_interrupt",
		"mret" //
	)
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
	Some(required_interrupt_wrapper),
	Some(required_interrupt_wrapper),
	None,
	None,
	None,
	None,
	None,
	None,
	None,
	None,
	Some(clock_tick_interrupt_wrapper),
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

#[repr(C)]
pub struct ProgrammableFastInterruptController {
	__0: [u32; 64],
	pub r32_pfic_ienr: [u32; 8],
	__1: [u32; 24],
	pub r32_pfic_irer: [u32; 8],
}

impl ProgrammableFastInterruptController {
	unsafe fn get() -> &'static mut Self {
		unsafe { (0xE000E000 as *mut Self).as_mut().unwrap_unchecked() }
	}
}

const _: () = {
	use core::mem::offset_of;
	assert!(
		offset_of!(
			ProgrammableFastInterruptController,
			r32_pfic_ienr //
		) == 0x100
	);
	assert!(
		offset_of!(
			ProgrammableFastInterruptController,
			r32_pfic_irer //
		) == 0x180
	);
};

// =========================================================================

pub const DISABLE: bool = false;
pub const ENABLE: bool = true;

pub unsafe fn set(interrupt: u8, allow: bool) {
	let programmable_fast_interrupt_controller = unsafe {
		ProgrammableFastInterruptController::get() //
	};
	let (offset, bit) = ((interrupt / 32) as usize, interrupt % 32);
	let register = allow
		.then(|| {
			&mut programmable_fast_interrupt_controller.r32_pfic_ienr
				[offset]
		})
		.unwrap_or_else(|| {
			&mut programmable_fast_interrupt_controller.r32_pfic_irer
				[offset]
		});
	unsafe {
		ptr::write_volatile(register, 1 << bit);
	}
}
