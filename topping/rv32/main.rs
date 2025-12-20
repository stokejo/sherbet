#![no_std]
use core::arch::global_asm;

pub mod interrupt;
pub mod pmp;
pub mod sync;

global_asm!(
	// Hardware entry point. Must be single
	// jump instruction so jump to setup.
	".section .text.start",
	".global start",
	"start:",
	"j setup",
	"",
	// Setup handler. Set stack and global pointers
	// and jump to user provided entry.
	".section .text.setup",
	".global setup",
	"setup:",
	"la sp, _stack_hi",
	"",
	".option push",
	".option norelax",
	"la gp, __global_pointer$",
	".option pop",
	"",
	"j sherbet_user_provided_entry",
);
