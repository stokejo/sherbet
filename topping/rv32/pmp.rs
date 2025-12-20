extern "C" {
	static _FLASH_BASE_ADDR: u8;
	static _FLASH_LEN: u8;
}

pub unsafe fn setup() {
	let flash_base_addr = &raw const _FLASH_BASE_ADDR as u32;
	let flash_len = &raw const _FLASH_LEN as u32;

	let val = (flash_base_addr >> 2) | ((flash_len >> 3) - 1);
	core::arch::asm!(
		"csrw 0x3B0, {val}",
		val = in(reg) val,
		options(nostack, nomem)
	);

	let val: u32 = 0b1 << 7 | 0b11 << 3 | 0b101;
	core::arch::asm!(
		"csrw 0x3A0, {val}",
		"fence.i",
		val = in(reg) val,
		options(nostack, nomem)
	);
}
