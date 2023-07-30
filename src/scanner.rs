use crate::csets::CharacterSet;

#[derive(Debug, Clone)]
pub struct Scanner {
	source: Box<[char]>,
	position: usize,
}

impl Scanner {
	pub fn new(source: impl AsRef<str>) -> Self {
		Self {
			source: source.as_ref().chars().collect(),
			position: 0,
		}
	}

	pub fn source(&self) -> &[char] {
		&self.source
	}

	pub fn len(&self) -> usize {
		self.source.len()
	}

	pub fn is_empty(&self) -> bool {
		self.len() == 0
	}

	pub fn char_at(&self, index: usize) -> Option<char> {
		self.source.get(index).cloned()
	}

	pub fn position(&self) -> usize {
		self.position
	}

	pub fn is_at_end(&self) -> bool {
		self.position == self.source.len()
	}

	pub fn goto(&mut self, position: usize) -> Option<String> {
		// allow reverse ranges
		let production = if self.position < position {
			self.source.get(self.position..position)?.iter().collect()
		} else {
			self.source
				.get(position..self.position)?
				.iter()
				.rev()
				.collect()
		};

		self.position = position;
		Some(production)
	}

	pub fn advance(&mut self, amount: isize) -> Option<String> {
		let position = self.position.checked_add_signed(amount)?;
		self.goto(position)
	}

	pub fn find_substring(&self, substring: impl AsRef<str>) -> Option<usize> {
		self.source
			.get(self.position..)?
			.iter()
			.collect::<String>()
			.find(substring.as_ref())
	}

	pub fn starts_with(&self, substring: impl AsRef<str>) -> Option<usize> {
		let mut i = self.position;
		for substring_char in substring.as_ref().chars() {
			if *self.source.get(i)? != substring_char {
				return None;
			}
			i += 1;
		}

		Some(i)
	}

	pub fn advance_if_starts_with(&mut self, substring: impl AsRef<str>) -> Option<String> {
		let position = self.starts_with(substring)?;
		self.goto(position)
	}

	pub fn any(&self, cset: impl CharacterSet) -> Option<usize> {
		cset.contains(*self.source.get(self.position)?)
			.then_some(self.position + 1)
	}

	pub fn many(&self, cset: impl CharacterSet) -> Option<usize> {
		if !cset.contains(*self.source.get(self.position)?) {
			return None;
		}

		let mut i = self.position;
		while i < self.source.len() && cset.contains(self.source[i]) {
			i += 1;
		}

		Some(i)
	}

	pub fn upto(&self, cset: impl CharacterSet) -> Option<usize> {
		let mut i = self.position;
		while !cset.contains(*self.source.get(i)?) {
			i += 1;
		}

		Some(i)
	}
}
