use Character;

#[derive(Copy, Clone, Eq, PartialEq, Default, Debug)]
pub struct Border {
	pub left:   Option<Character>,
	pub right:  Option<Character>,
	pub top:    Option<Character>,
	pub bottom: Option<Character>,

	pub top_left:     Option<Character>,
	pub top_right:    Option<Character>,
	pub bottom_left:  Option<Character>,
	pub bottom_right: Option<Character>,
}

impl Border {
	#[inline]
	pub fn blank() -> Self {
		Border {
			left:   Some(' '.into()),
			right:  Some(' '.into()),
			top:    Some(' '.into()),
			bottom: Some(' '.into()),

			top_left:     Some(' '.into()),
			top_right:    Some(' '.into()),
			bottom_left:  Some(' '.into()),
			bottom_right: Some(' '.into()),
		}
	}

	#[inline]
	pub fn left<T: Into<Character>>(mut self, value: T) -> Self {
		self.left = Some(value.into());
		self
	}

	#[inline]
	pub fn right<T: Into<Character>>(mut self, value: T) -> Self {
		self.right = Some(value.into());
		self
	}

	#[inline]
	pub fn top<T: Into<Character>>(mut self, value: T) -> Self {
		self.top = Some(value.into());
		self
	}

	#[inline]
	pub fn bottom<T: Into<Character>>(mut self, value: T) -> Self {
		self.bottom = Some(value.into());
		self
	}

	#[inline]
	pub fn top_left<T: Into<Character>>(mut self, value: T) -> Self {
		self.top_left = Some(value.into());
		self
	}

	#[inline]
	pub fn top_right<T: Into<Character>>(mut self, value: T) -> Self {
		self.top_right = Some(value.into());
		self
	}

	#[inline]
	pub fn bottom_left<T: Into<Character>>(mut self, value: T) -> Self {
		self.bottom_left = Some(value.into());
		self
	}

	#[inline]
	pub fn bottom_right<T: Into<Character>>(mut self, value: T) -> Self {
		self.bottom_right = Some(value.into());
		self
	}
}
