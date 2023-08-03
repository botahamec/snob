use crate::csets::CharacterSet;

/// This is used to analyze string. It can be initialized using either
/// [`Scanner::from`] or [`Scanner::new`].
///
/// # Example
///
/// ```
/// use snob::Scanner;
///
/// let mut scanner = Scanner::new("Hello, world!");
/// if let Some(position) = scanner.starts_with("Hello") {
///     scanner.goto(position);
/// }
/// ```
#[derive(Debug, Clone)]
pub struct Scanner {
	source: Box<[char]>,
	position: usize,
}

impl Scanner {
	/// Create a new Scanner with a given source.
	///
	/// # Example
	///
	/// ```
	/// use snob::Scanner;
	///
	/// let scanner = Scanner::new("Hello, world!");
	/// ```
	pub fn new(source: impl AsRef<str>) -> Self {
		Self {
			source: source.as_ref().chars().collect(),
			position: 0,
		}
	}

	/// Get the full source being used in this scanner, as a slice of characters
	///
	/// # Example
	///
	/// ```
	/// use snob::Scanner;
	///
	/// let scanner = Scanner::new("Hello, world!");
	/// let source = scanner.source().iter().collect::<String>();
	/// assert_eq!(scanner.source().iter().collect::<String>(), "Hello, world!");
	/// ```
	pub fn source(&self) -> &[char] {
		&self.source
	}

	/// Get the full length of the source being used in this scanner
	///
	/// # Example
	///
	/// ```
	/// use snob::Scanner;
	///
	/// let scanner = Scanner::new("Hello, world!");
	/// assert_eq!(scanner.len(), 13);
	/// ```
	pub fn len(&self) -> usize {
		self.source.len()
	}

	/// Returns `true` if the scanner's source is an empty string
	///
	/// # Example
	///
	/// ```
	/// use snob::Scanner;
	///
	/// let scanner = Scanner::new("Hello, world!");
	/// assert!(!scanner.is_empty());
	/// ```
	pub fn is_empty(&self) -> bool {
		self.len() == 0
	}

	/// Get the character at a given position in the string.
	///
	/// # Example
	///
	/// ```
	/// use snob::Scanner;
	///
	/// let scanner = Scanner::new("Hello, world!");
	/// assert!(!scanner.is_empty());
	/// ```
	pub fn char_at(&self, index: usize) -> Option<char> {
		self.source.get(index).cloned()
	}

	/// Get the current position in the string. When the [`Scanner`] is
	/// created, this value is zero.
	///
	/// # Example
	///
	/// ```
	/// use snob::Scanner;
	///
	/// let mut scanner = Scanner::new("Hello, world!");
	/// assert_eq!(scanner.position(), 0);
	/// scanner.advance(5);
	/// assert_eq!(scanner.position(), 5);
	/// scanner.goto(3);
	/// assert_eq!(scanner.position(), 3);
	/// ```
	pub fn position(&self) -> usize {
		self.position
	}

	/// Returns true if the scanner's position has reached the end of its
	/// source.
	///
	/// # Example
	///
	/// ```
	/// use snob::Scanner;
	///
	/// let mut scanner = Scanner::new("Hello, world!");
	/// assert!(!scanner.is_at_end());
	///
	/// if let Some(position) = scanner.starts_with("Hello, world!") {
	///     scanner.goto(position);
	/// }
	///
	/// assert!(scanner.is_at_end());
	/// ```
	pub fn is_at_end(&self) -> bool {
		self.position == self.source.len()
	}

	/// Set the scanner's `position`. If the position out of range out the
	/// source, then `None` is returned. Otherwise, the subslice from the old
	/// position to the new position is returned. If the latter is less than
	/// the former, then the string is reversed.
	///
	/// # Example
	///
	/// ```
	/// use snob::Scanner;
	///
	/// let mut scanner = Scanner::new("Hello, world!");
	/// scanner.goto(3);
	/// assert_eq!(scanner.position(), 3);
	/// ```
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

	/// Increase the position by the given `amount`. If the new position is out
	/// of the range of the source, then `None` is returned. Otherwise, the
	/// subslice from the old position to the new position is returned. If the
	/// latter is less than the former, then the string is reversed.
	///
	/// # Example
	///
	/// ```
	/// use snob::Scanner;
	///
	/// let mut scanner = Scanner::new("Hello, world!");
	/// scanner.advance(5);
	/// assert_eq!(scanner.position(), 5);
	/// ```
	pub fn advance(&mut self, amount: isize) -> Option<String> {
		let position = self.position.checked_add_signed(amount)?;
		self.goto(position)
	}

	/// Looks for the given `substring` in the remainder of the scanner. If the
	/// substring is found, the position of the first character in the
	/// substring is returned. Otherwise, `None` is returned.
	///
	/// # Example
	///
	/// ```
	/// use snob::Scanner;
	///
	/// # fn foo() -> Option<()> {
	/// let scanner = Scanner::new("Hello, world!");
	/// let position = scanner.find_substring("lo")?;
	/// assert_eq!(position, 3);
	/// # Some(())
	/// # }
	pub fn find_substring(&self, substring: impl AsRef<str>) -> Option<usize> {
		self.source
			.get(self.position..)?
			.iter()
			.collect::<String>()
			.find(substring.as_ref())
	}

	/// If `source[position..]` starts with the given string, then this returns
	/// the ending position of the substring. Otherwise, `None` is returned.
	///
	/// # Example
	///
	/// ```
	/// use snob::Scanner;
	///
	/// # fn foo() -> Option<()> {
	/// let scanner = Scanner::new("Hello, world!");
	/// let position = scanner.starts_with("Hello")?;
	/// assert_eq!(position, 5);
	/// # Some(())
	/// # }
	/// ```
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

	/// If `source[position..]` starts with the given string, then this returns
	/// a copy of the substring. Otherwise, `None` is returned. This is the
	/// equivalent of: `self.goto(self.starts_with(substring)?)`.
	///
	/// # Example
	///
	/// ```
	/// use snob::Scanner;
	///
	/// # fn foo() -> Option<()> {
	/// let mut scanner = Scanner::new("Hello, world!");
	/// let substring = scanner.advance_if_starts_with("Hello")?;
	/// assert_eq!(substring, "Hello");
	/// assert_eq!(scanner.position(), 5);
	/// # Some(())
	/// # }
	/// ```
	pub fn advance_if_starts_with(&mut self, substring: impl AsRef<str>) -> Option<String> {
		let position = self.starts_with(substring)?;
		self.goto(position)
	}

	/// If the next character in the scanner is contained in the given `cset`,
	/// then the position after the next character is returned. Otherwise,
	/// `None` is returned.
	///
	/// # Example
	///
	/// ```
	/// use snob::Scanner;
	///
	/// # fn foo() -> Option<()> {
	/// let scanner = Scanner::new("Hello, world!");
	/// let position = scanner.any('H')?;
	/// assert_eq!(position, 1);
	/// # Some(())
	/// # }
	/// ```
	pub fn any(&self, cset: impl CharacterSet) -> Option<usize> {
		cset.contains(*self.source.get(self.position)?)
			.then_some(self.position + 1)
	}

	/// If the next character in the scanner is contained in the given `cset`,
	/// then the position after the longest initial sequence of characters in
	/// `cset` is returned. Otherwise, `None` is returned.
	///
	/// # Example
	///
	/// ```
	/// use snob::Scanner;
	/// use snob::csets::AsciiLetters;
	///
	/// # fn foo() -> Option<()> {
	/// let scanner = Scanner::new("Hello, world!");
	/// let position = scanner.many(AsciiLetters)?;
	/// assert_eq!(position, 5);
	/// # Some(())
	/// # }
	/// ```
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

	/// If the remainder of the scanner contains a character from the given
	/// `cset`, then the position of the aforementioned character is returned.
	/// Otherwise, `None` is returned.
	///
	/// # Example
	///
	/// ```
	/// use snob::Scanner;
	///
	/// # fn foo() -> Option<()> {
	/// let scanner = Scanner::new("Hello, world!");
	/// let position = scanner.upto(' ')?;
	/// assert_eq!(position, 6);
	/// # Some(())
	/// # }
	/// ```
	pub fn upto(&self, cset: impl CharacterSet) -> Option<usize> {
		let mut i = self.position;
		while !cset.contains(*self.source.get(i)?) {
			i += 1;
		}

		Some(i)
	}
}

impl From<&str> for Scanner {
	fn from(value: &str) -> Self {
		Self::new(value)
	}
}

impl From<Box<[char]>> for Scanner {
	fn from(value: Box<[char]>) -> Self {
		Self {
			source: value,
			position: 0,
		}
	}
}

impl AsRef<[char]> for Scanner {
	fn as_ref(&self) -> &[char] {
		&self.source
	}
}
