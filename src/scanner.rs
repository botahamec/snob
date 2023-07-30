use std::io::Read;

pub struct Scanner<Source: Read> {
	source: Source,
}

impl<Source: Read> Scanner<Source> {
	pub fn new(source: Source) -> Self {
		Self { source }
	}
}
