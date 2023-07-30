use std::collections::HashSet;

pub trait CharacterSet {
	fn contains(&self, ch: char) -> bool;
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
