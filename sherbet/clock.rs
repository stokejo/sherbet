use portable_atomic::{AtomicUsize, Ordering};

static TICKI: AtomicUsize = AtomicUsize::new(0);

pub fn set(value: usize) {
	TICKI.store(value, Ordering::SeqCst);
}

pub fn ticka() {
	TICKI.add(1, Ordering::SeqCst);
}
