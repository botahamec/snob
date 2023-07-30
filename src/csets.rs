pub trait CharacterSet {
	fn contains(ch: char) -> bool;
}

pub struct AnyCharacter;

impl CharacterSet for AnyCharacter {
	fn contains(_: char) -> bool {
		true
	}
}

pub struct Ascii;

impl CharacterSet for Ascii {
	fn contains(ch: char) -> bool {
		ch.is_ascii()
	}
}

pub struct AsciiDigits;

impl CharacterSet for AsciiDigits {
	fn contains(ch: char) -> bool {
		ch.is_ascii_digit()
	}
}

pub struct AsciiLowercase;

impl CharacterSet for AsciiLowercase {
	fn contains(ch: char) -> bool {
		ch.is_ascii_lowercase()
	}
}

pub struct AsciiUppercase;

impl CharacterSet for AsciiUppercase {
	fn contains(ch: char) -> bool {
		ch.is_ascii_uppercase()
	}
}

pub struct AsciiLetters;

impl CharacterSet for AsciiLetters {
	fn contains(ch: char) -> bool {
		ch.is_ascii_alphabetic()
	}
}
