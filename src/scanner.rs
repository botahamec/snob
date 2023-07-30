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

	pub fn position(&self) -> usize {
		self.position
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

	pub fn advance(&mut self, amount: usize) -> Option<String> {
		self.goto(self.position + amount)
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
}
