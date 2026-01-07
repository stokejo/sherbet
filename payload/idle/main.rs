#![no_std]

use core::{
	hint,
	pin::Pin,
	task::{Context, Poll},
};

pub(crate) struct Yield {
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

pub async fn main() {
	loop {
		Yield { inner: false }.await;
		hint::spin_loop();
	}
}
