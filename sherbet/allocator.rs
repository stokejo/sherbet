use core::{
	alloc::{GlobalAlloc, Layout},
	ptr::{self, NonNull},
};

use portable_atomic::{AtomicPtr, Ordering};

// =========================================================================

extern "C" {
	static mut _heap_va: u8;
	static mut _heap_hi: u8;
}
static CURSOR: AtomicPtr<u8> = const { AtomicPtr::new(&raw mut _heap_va) };

// =========================================================================

fn alloc(mut size: usize, align: usize) -> Option<ptr::NonNull<u8>> {
	(size == 0).then(|| size = 1);
	let mut cursor = CURSOR.load(Ordering::Acquire);
	let hi = (&raw mut _heap_hi) as usize;
	loop {
		// Check how much free space is required for align.
		let align_offset = cursor.align_offset(align);

		// This also checks for possible usize::MAX align_offset.
		let aligned_size = size.checked_add(align_offset)?;

		// Check that there enough space for this alloc.
		let offset = hi.checked_sub(cursor as usize)?;
		(offset >= aligned_size).then_some(())?;

		// Align pointer to get address that we should return.
		let addr = unsafe { cursor.byte_add(align_offset) };

		// Try to place value into cursor and check.
		let exchange = CURSOR.compare_exchange(
			cursor,
			addr.wrapping_byte_add(size),
			Ordering::AcqRel,
			Ordering::Acquire,
		);
		if let Err(new_cursor) = exchange {
			cursor = new_cursor;
			continue;
		}

		// Now with updated state just safely return pointer.
		let addr = unsafe { NonNull::new_unchecked(addr) };
		break Some(addr);
	}
}

// =========================================================================

struct Alloc;

unsafe impl GlobalAlloc for Alloc {
	unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
		let ptr = alloc(layout.size(), layout.align());
		ptr.map_or(ptr::null_mut(), NonNull::as_ptr)
	}
	unsafe fn dealloc(&self, _: *mut u8, _: Layout) {}
}

#[global_allocator]
static ALLOC: Alloc = Alloc;
