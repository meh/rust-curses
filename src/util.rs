macro_rules! some {
	($expr:expr) => (
		if let Ok(value) = $expr {
			value
		}
		else {
			return None;
		}
	)
}

macro_rules! cchar {
	($expr:expr) => (
		if $expr.is_empty() {
			::std::ptr::null()
		}
		else {
			$expr.as_ptr()
		}
	)
}
