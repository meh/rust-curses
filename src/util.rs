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
