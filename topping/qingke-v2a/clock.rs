use crate::interrupt;
use {core::ptr, sherbet::clock};

// =========================================================================

#[repr(C)]
struct SysTick {
	r32_stk_ctlr: u32,
	r32_stk_stsr: u32,
	r32_stk_cntr: u32,
	__0: u32,
	r32_stk_cmpr: u32,
	__1: u32,
}

impl SysTick {
	pub unsafe fn get() -> &'static mut Self {
		unsafe { (0xE000F000 as *mut Self).as_mut().unwrap_unchecked() }
	}
}

// =========================================================================

const STK_AUTO_RELOAD_ENABLE: u32 = 1 << 3;
const STK_INTERRUPT_ENABLE: u32 = 1 << 1;
const STK_ENABLE: u32 = 1 << 0;
const STK_INTERRUPT_NUMBER: u32 = 12;

pub fn setup() {
	let sys_tick = unsafe { SysTick::get() };
	unsafe {
		ptr::write_volatile(&mut sys_tick.r32_stk_ctlr, 0x0);
		ptr::write_volatile(
			&mut sys_tick.r32_stk_cmpr,
			8_000_000 / 8, // it works, but why???
		);
		ptr::write_volatile(&mut sys_tick.r32_stk_cntr, 0x0);
	};
	clock::set(0x0);
	unsafe {
		ptr::write_volatile(
			&mut sys_tick.r32_stk_ctlr,
			STK_AUTO_RELOAD_ENABLE | STK_INTERRUPT_ENABLE | STK_ENABLE,
		);
		interrupt::set(STK_INTERRUPT_NUMBER, interrupt::ALLOW);
	};
}

pub(crate) fn reset() {
	let sys_tick = unsafe { SysTick::get() };
	unsafe {
		ptr::write_volatile(&mut sys_tick.r32_stk_stsr, 0);
	};
}
