use alloc::boxed::Box;
use core::{cell::RefCell, future::Future, pin::Pin};

use critical_section::Mutex;
use heapless as hp;

// =========================================================================

type PinBoxFut = Pin<Box<dyn Future<Output = ()> + Send>>;
type Taski = hp::Vec<PinBoxFut, 2>;

static TASKI: Mutex<RefCell<Taski>> = //
	Mutex::new(RefCell::new(hp::Vec::new()));

// =========================================================================

pub fn spawn(t: PinBoxFut) -> Result<(), PinBoxFut> {
	critical_section::with(|cs| {
		let mut taski = TASKI.borrow_ref_mut(cs);
		taski.push(t)
	})
}

pub fn block(t: PinBoxFut) {}
