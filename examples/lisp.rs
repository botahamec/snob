pub use snob::{csets, Scanner};

pub const EXAMPLE_LIST_PROGRAM: &str = r"
(defclass rewindable ()
((rewind-store :reader rewind-store
			   :initform (make-array 12 :fill-pointer 0 :adjustable t))
 ;; Index is the number of rewinds we've done.
 (rewind-index :accessor rewind-index
			   :initform 0)))


(defun rewind-count (rewindable)
(fill-pointer (rewind-store rewindable)))


(defun last-state (rewindable)
(let ((size (rewind-count rewindable)))
  (if (zerop size)
	  (values nil nil)
	  (values (aref (rewind-store rewindable) (1- size)) t))))


(defun save-rewindable-state (rewindable object)
(let ((index (rewind-index rewindable))
	  (store (rewind-store rewindable)))
  (unless (zerop index)
	;; Reverse the tail of pool, since we've
	;; gotten to the middle by rewinding.
	(setf (subseq store index) (nreverse (subseq store index))))
  (vector-push-extend object store)))


(defmethod rewind-state ((rewindable rewindable))
(invariant (not (zerop (rewind-count rewindable))))
(setf (rewind-index rewindable)
	  (mod (1+ (rewind-index rewindable)) (rewind-count rewindable)))
(aref (rewind-store rewindable)
	  (- (rewind-count rewindable) (rewind-index rewindable) 1)))
";

#[derive(Debug)]
enum Token {
	Dot,
	Quote,
	Function,
	Integer(i64),
	Symbol(String),
	Comment(String),
	LeftParenthesis,
	RightParenthesis,
}

struct Tokenizer {
	scanner: Scanner,
}

impl Tokenizer {
	fn new(source: &str) -> Self {
		Self {
			scanner: Scanner::new(source),
		}
	}
}

impl Iterator for Tokenizer {
	type Item = Token;

	fn next(&mut self) -> Option<Self::Item> {
		// skip over any whitespace
		if let Some(position) = self.scanner.many(" \t\r\n") {
			self.scanner.goto(position);
		}

		// terminate if done
		if self.scanner.is_at_end() {
			return None;
		}

		if let Some(position) = self.scanner.any('.') {
			self.scanner.goto(position);
			Some(Token::Dot)
		} else if let Some(position) = self.scanner.any('\'') {
			self.scanner.goto(position);
			Some(Token::Quote)
		} else if let Some(position) = self.scanner.any('(') {
			self.scanner.goto(position);
			Some(Token::LeftParenthesis)
		} else if let Some(position) = self.scanner.any(')') {
			self.scanner.goto(position);
			Some(Token::RightParenthesis)
		} else if let Some(position) = self.scanner.starts_with("#'") {
			self.scanner.goto(position);
			Some(Token::Function)
		} else if let Some(position) = self.scanner.many(csets::AsciiDigits) {
			let number = self.scanner.goto(position).unwrap();
			let number = number.parse::<i64>().unwrap();
			Some(Token::Integer(number))
		} else if let Some(position) = self.scanner.any(';') {
			self.scanner.goto(position);
			let position = self.scanner.upto("\r\n").expect("Unterminated comment");
			let comment = self.scanner.goto(position).unwrap();
			Some(Token::Comment(comment))
		} else {
			let position = self
				.scanner
				.upto(" \t\r\n().\"'#")
				.expect("unterminated symbol");
			let symbol = self.scanner.goto(position).unwrap();
			Some(Token::Symbol(symbol))
		}
	}
}

fn main() {
	for token in Tokenizer::new(EXAMPLE_LIST_PROGRAM) {
		println!("{token:?}");
	}
}
