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
}
