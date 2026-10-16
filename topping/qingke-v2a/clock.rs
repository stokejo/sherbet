use crate::interrupt;
use {core::ptr, sherbet::clock};

// =========================================================================

const RCC_CFGR0_HPRE_MASK: u32 = 0xF << 4;

#[repr(C)]
struct ResetAndClockControl {
	r32_rcc_ctlr: u32,
	r32_rcc_cfgr0: u32,
	r32_rcc_intr: u32,
	r32_rcc_apb2prstr: u32,
	r32_rcc_apb1prstr: u32,
	r32_rcc_ahbpcenr: u32,
	r32_rcc_apb2pcenr: u32,
	r32_rcc_apb1pcenr: u32,
	__0: u32,
	r32_rcc_rstsckr: u32,
}

impl ResetAndClockControl {
	unsafe fn get() -> &'static mut Self {
		unsafe { (0x40021000 as *mut Self).as_mut().unwrap_unchecked() }
	}
}

const _: () = {
	use core::mem::offset_of;
	assert!(offset_of!(ResetAndClockControl, r32_rcc_apb1pcenr) == 0x1C);
	assert!(offset_of!(ResetAndClockControl, r32_rcc_rstsckr) == 0x24);
};

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
	unsafe fn get() -> &'static mut Self {
		unsafe { (0xE000F000 as *mut Self).as_mut().unwrap_unchecked() }
	}
}

// =========================================================================

const STK_AUTO_RELOAD_ENABLE: u32 = 1 << 3;
const STK_INTERRUPT_ENABLE: u32 = 1 << 1;
const STK_ENABLE: u32 = 1 << 0;
const STK_INTERRUPT_NUMBER: u32 = 12;

pub fn setup() {
	let reset_and_clock_control = unsafe { ResetAndClockControl::get() };
	unsafe {
		let r32_rcc_cfgr0 = ptr::read_volatile(
			&reset_and_clock_control.r32_rcc_cfgr0, //
		);
		ptr::write_volatile(
			&mut reset_and_clock_control.r32_rcc_cfgr0,
			r32_rcc_cfgr0 & !RCC_CFGR0_HPRE_MASK,
		);
	};
	let sys_tick = unsafe { SysTick::get() };
	unsafe {
		ptr::write_volatile(&mut sys_tick.r32_stk_ctlr, 0x0);
		ptr::write_volatile(
			&mut sys_tick.r32_stk_cmpr,
			24_000_000 / 8, //
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
