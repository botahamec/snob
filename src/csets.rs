use std::collections::HashSet;

pub trait CharacterSet: Sized {
	fn contains(&self, ch: char) -> bool;

	fn union<Other: CharacterSet>(self, other: Other) -> CharacterSetUnion<Self, Other> {
		CharacterSetUnion {
			first: self,
			second: other,
		}
	}

	fn intersection<Other: CharacterSet>(
		self,
		other: Other,
	) -> CharacterSetIntersection<Self, Other> {
		CharacterSetIntersection {
			first: self,
			second: other,
		}
	}

	fn difference<Other: CharacterSet>(self, other: Other) -> CharacterSetDifference<Self, Other> {
		CharacterSetDifference {
			first: self,
			second: other,
		}
	}
}

pub struct AnyCharacter;

impl CharacterSet for AnyCharacter {
	fn contains(&self, _: char) -> bool {
		true
	}
}

pub struct Ascii;

impl CharacterSet for Ascii {
	fn contains(&self, ch: char) -> bool {
		ch.is_ascii()
	}
}

pub struct AsciiDigits;

impl CharacterSet for AsciiDigits {
	fn contains(&self, ch: char) -> bool {
		ch.is_ascii_digit()
	}
}

pub struct AsciiLowercase;

impl CharacterSet for AsciiLowercase {
	fn contains(&self, ch: char) -> bool {
		ch.is_ascii_lowercase()
	}
}

pub struct AsciiUppercase;

impl CharacterSet for AsciiUppercase {
	fn contains(&self, ch: char) -> bool {
		ch.is_ascii_uppercase()
	}
}

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

pub struct CharacterSetUnion<A: CharacterSet, B: CharacterSet> {
	first: A,
	second: B,
}

impl<A: CharacterSet, B: CharacterSet> CharacterSet for CharacterSetUnion<A, B> {
	fn contains(&self, ch: char) -> bool {
		self.first.contains(ch) || self.second.contains(ch)
	}
}

pub struct CharacterSetIntersection<A: CharacterSet, B: CharacterSet> {
	first: A,
	second: B,
}

impl<A: CharacterSet, B: CharacterSet> CharacterSet for CharacterSetIntersection<A, B> {
	fn contains(&self, ch: char) -> bool {
		self.first.contains(ch) && self.second.contains(ch)
	}
}

pub struct CharacterSetDifference<A: CharacterSet, B: CharacterSet> {
	first: A,
	second: B,
}

impl<A: CharacterSet, B: CharacterSet> CharacterSet for CharacterSetDifference<A, B> {
	fn contains(&self, ch: char) -> bool {
		self.first.contains(ch) && !self.second.contains(ch)
	}
}

pub struct CharacterSetComplement<Inner: CharacterSet> {
	inner: Inner,
}

impl<Inner: CharacterSet> CharacterSet for CharacterSetComplement<Inner> {
	fn contains(&self, ch: char) -> bool {
		!self.inner.contains(ch)
	}
}
