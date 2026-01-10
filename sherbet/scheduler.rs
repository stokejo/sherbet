use core::{
	cell::RefCell,
	future::Future,
	mem,
	pin::Pin,
	task::{Context, Poll, Waker},
};

use alloc::boxed::Box;
use critical_section::Mutex as FakeMutex;
use heapless as hp;

// =========================================================================

type PinBoxFut = Pin<Box<dyn Future<Output = ()> + Send>>;
type Taski = hp::Vec<PinBoxFut, 2>;

static TASKI: FakeMutex<RefCell<Taski>> = //
	FakeMutex::new(RefCell::new(hp::Vec::new()));

pub fn spawn(t: PinBoxFut) -> Result<(), PinBoxFut> {
	critical_section::with(|cs| {
		let mut taski = TASKI.borrow_ref_mut(cs);
		taski.push(t)
	})
}

// =========================================================================

struct Yield {
	inner: bool,
}

impl Future for Yield {
	type Output = ();
	fn poll(mut self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<()> {
		self.inner.then_some(Poll::Ready(())).unwrap_or_else(|| {
			self.inner = true;
			Poll::Pending
		})
	}
}

pub async fn yield_now() {
	Yield { inner: false }.await;
}

// =========================================================================

fn schedule<'a>(
	t: &mut PinBoxFut,
	context: &mut Context<'a>,
) -> Option<()> {
	match t.as_mut().poll(context) {
		Poll::Ready(()) => critical_section::with(|cs| {
			let mut taski = TASKI.borrow_ref_mut(cs);
			let taska = (!taski.is_empty()).then(|| taski.remove(0));
			taska.map(|taska| mem::replace(t, taska)).map(|_| ())
		})?,
		Poll::Pending => critical_section::with(|cs| {
			let mut taski = TASKI.borrow_ref_mut(cs);
			let taska = (!taski.is_empty()).then(|| taski.remove(0));
			let taska = taska.map(|taska| mem::replace(t, taska));
			taska.map(|t| unsafe { taski.push_unchecked(t) });
			Some(())
		})?,
	};
	Some(())
}

pub fn block(mut t: PinBoxFut) {
	let waker = Waker::noop();
	let mut context = Context::from_waker(waker);
	while let Some(()) = schedule(&mut t, &mut context) {}
}
