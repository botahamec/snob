use std::collections::HashSet;

pub trait CharacterSet {
	fn contains(&self, ch: char) -> bool;

	fn union<Other: CharacterSet>(self, other: Other) -> CharacterSetUnion<Self, Other>
	where
		Self: Sized,
	{
		CharacterSetUnion {
			first: self,
			second: other,
		}
	}

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

	fn difference<Other: CharacterSet>(self, other: Other) -> CharacterSetDifference<Self, Other>
	where
		Self: Sized,
	{
		CharacterSetDifference {
			first: self,
			second: other,
		}
	}

	fn complement(self) -> CharacterSetComplement<Self>
	where
		Self: Sized,
	{
		CharacterSetComplement { inner: self }
	}
}

#[derive(Debug, Clone, Copy)]
pub struct AnyCharacter;

impl CharacterSet for AnyCharacter {
	fn contains(&self, _: char) -> bool {
		true
	}
}

#[derive(Debug, Clone, Copy)]
pub struct Ascii;

impl CharacterSet for Ascii {
	fn contains(&self, ch: char) -> bool {
		ch.is_ascii()
	}
}

#[derive(Debug, Clone, Copy)]
pub struct AsciiDigits;

impl CharacterSet for AsciiDigits {
	fn contains(&self, ch: char) -> bool {
		ch.is_ascii_digit()
	}
}

#[derive(Debug, Clone, Copy)]
pub struct AsciiLowercase;

impl CharacterSet for AsciiLowercase {
	fn contains(&self, ch: char) -> bool {
		ch.is_ascii_lowercase()
	}
}

#[derive(Debug, Clone, Copy)]
pub struct AsciiUppercase;

impl CharacterSet for AsciiUppercase {
	fn contains(&self, ch: char) -> bool {
		ch.is_ascii_uppercase()
	}
}

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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]

pub struct CharacterSetComplement<Inner: CharacterSet> {
	inner: Inner,
}

impl<Inner: CharacterSet> CharacterSet for CharacterSetComplement<Inner> {
	fn contains(&self, ch: char) -> bool {
		!self.inner.contains(ch)
	}
}
