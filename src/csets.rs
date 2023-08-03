use std::collections::HashSet;

/// An unordered set of characters
///
/// # Example
///
/// ```
/// use snob::csets::CharacterSet;
///
/// struct AsciiCharacter;
///
/// impl CharacterSet for AsciiCharacter {
///     fn contains(&self, ch: char) -> bool {
///         ch.is_ascii()
///     }
/// }
/// ```
pub trait CharacterSet {
	/// Returns `true` if the character set contains the given character.
	///
	/// # Example
	///
	/// ```
	/// use snob::csets::AsciiLetters;
	///
	/// assert!(AsciiLetters.contains('h'));
	/// assert!(!AsciiLetters.contains(' '));
	/// ```
	fn contains(&self, ch: char) -> bool;

	/// Returns a [`CharacterSet`] that contains the characters in the `self`
	/// set, as well as any characters in the given `other` character set.
	///
	/// # Example
	///
	/// ```
	/// use snob::csets::AsciiLetters;
	///
	/// let cset = AsciiLetters.union(' ');
	/// assert!(cset.contains('h'));
	/// assert!(cset.contains(' '));
	/// ```
	fn union<Other: CharacterSet>(self, other: Other) -> CharacterSetUnion<Self, Other>
	where
		Self: Sized,
	{
		CharacterSetUnion {
			first: self,
			second: other,
		}
	}

	/// Returns a [`CharacterSet`] that contains only the characters in both
	/// of `self` and `other`.
	///
	/// # Example
	///
	/// ```
	/// use snob::csets::AsciiLetters;
	///
	/// let cset = AsciiLetters.intersection("Hello, world");
	/// assert!(cset.contains('e'));
	/// assert!(!cset.contains('a'));
	/// assert!(!cset.contains(' '));
	/// ```
	fn intersection<Other: CharacterSet>(
		self,
		other: Other,
	) -> CharacterSetIntersection<Self, Other>
	where
		Self: Sized,
	{
		CharacterSetIntersection {
			first: self,
			second: other,
		}
	}

	/// Returns a [`CharacterSet`] that contains the characters in the `self`
	/// character set, unless they are also contained in `other`.
	///
	/// # Example
	///
	/// ```
	/// use snob::csets::AsciiLetters;
	///
	/// let cset = AsciiLetters.intersection("Hello, world");
	/// assert!(cset.contains('a'));
	/// assert!(!cset.contains('e'));
	/// assert!(!cset.contains(' '));
	/// ```
	fn difference<Other: CharacterSet>(self, other: Other) -> CharacterSetDifference<Self, Other>
	where
		Self: Sized,
	{
		CharacterSetDifference {
			first: self,
			second: other,
		}
	}

	/// Returns a [`CharacterSet`] that contains all of the characters that are
	/// NOT contained in the `self` character set.
	///
	/// # Example
	///
	/// ```
	/// use snob::csets::AsciiLetters;
	///
	/// let cset = AsciiLetters.complement();
	/// assert!(!cset.contains('a'));
	/// assert!(cset.contains(' '));
	/// ```
	fn complement(self) -> CharacterSetComplement<Self>
	where
		Self: Sized,
	{
		CharacterSetComplement { inner: self }
	}
}

/// Contains all Unicode characters
#[derive(Debug, Clone, Copy)]
pub struct AnyCharacter;

impl CharacterSet for AnyCharacter {
	fn contains(&self, _: char) -> bool {
		true
	}
}

/// Contains all ASCII characters
#[derive(Debug, Clone, Copy)]
pub struct Ascii;

impl CharacterSet for Ascii {
	fn contains(&self, ch: char) -> bool {
		ch.is_ascii()
	}
}

/// Contains the ASCII digits, 0-9
#[derive(Debug, Clone, Copy)]
pub struct AsciiDigits;

impl CharacterSet for AsciiDigits {
	fn contains(&self, ch: char) -> bool {
		ch.is_ascii_digit()
	}
}

/// Contains all lowercase ASCII letters, a-z
#[derive(Debug, Clone, Copy)]
pub struct AsciiLowercase;

impl CharacterSet for AsciiLowercase {
	fn contains(&self, ch: char) -> bool {
		ch.is_ascii_lowercase()
	}
}

/// Contains all uppercase ASCII letters, A-Z
#[derive(Debug, Clone, Copy)]
pub struct AsciiUppercase;

impl CharacterSet for AsciiUppercase {
	fn contains(&self, ch: char) -> bool {
		ch.is_ascii_uppercase()
	}
}

/// Containes all ASCII letters: a-z, A-Z
#[derive(Debug, Clone, Copy)]
pub struct AsciiLetters;

impl CharacterSet for AsciiLetters {
	fn contains(&self, ch: char) -> bool {
		ch.is_ascii_alphabetic()
	}
}

impl CharacterSet for char {
	fn contains(&self, ch: char) -> bool {
		*self == ch
	}
}

impl CharacterSet for &[char] {
	fn contains(&self, ch: char) -> bool {
		(self as &[char]).contains(&ch)
	}
}

impl CharacterSet for &str {
	fn contains(&self, ch: char) -> bool {
		self.chars().any(|c| c == ch)
	}
}

impl CharacterSet for HashSet<char> {
	fn contains(&self, ch: char) -> bool {
		self.contains(&ch)
	}
}

/// A union of two [`CharacterSet`]s.
///
/// This is created by calling [`CharacterSet::union`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CharacterSetUnion<A: CharacterSet, B: CharacterSet> {
	first: A,
	second: B,
}

impl<A: CharacterSet, B: CharacterSet> CharacterSet for CharacterSetUnion<A, B> {
	fn contains(&self, ch: char) -> bool {
		self.first.contains(ch) || self.second.contains(ch)
	}
}

/// An intersection of two [`CharacterSet`]s.
///
/// This is created by calling [`CharacterSet::intersection`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CharacterSetIntersection<A: CharacterSet, B: CharacterSet> {
	first: A,
	second: B,
}

impl<A: CharacterSet, B: CharacterSet> CharacterSet for CharacterSetIntersection<A, B> {
	fn contains(&self, ch: char) -> bool {
		self.first.contains(ch) && self.second.contains(ch)
	}
}

/// The difference of two [`CharacterSet`]s.
///
/// This is created by calling [`CharacterSet::difference`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CharacterSetDifference<A: CharacterSet, B: CharacterSet> {
	first: A,
	second: B,
}

impl<A: CharacterSet, B: CharacterSet> CharacterSet for CharacterSetDifference<A, B> {
	fn contains(&self, ch: char) -> bool {
		self.first.contains(ch) && !self.second.contains(ch)
	}
}

/// The complement of a [`CharacterSet`].
///
/// This is created by calling [`CharacterSet::complement`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CharacterSetComplement<Inner: CharacterSet> {
	inner: Inner,
}

impl<Inner: CharacterSet> CharacterSet for CharacterSetComplement<Inner> {
	fn contains(&self, ch: char) -> bool {
		!self.inner.contains(ch)
	}
}
