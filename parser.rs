//! generic parser structure and functionality to be
//! extended by other (e.g., css, html) parsers

pub struct Parser {
  pub pos: uint,
  pub input: String,
}

impl Parser {
  // read & return next char
  pub fn next_char(&self) -> char {
    self.input.as_slice().char_at(self.pos)
  }

  // do the next chars start with string `s`?
  pub fn starts_with(&self, s: &str) -> bool {
    self.input.as_slice().slice_from(self.pos).starts_with(s)
  }

  // have we consumed all the input?
  pub fn eof(&self) -> bool {
    self.pos >= self.input.len()
  }

  // return current char and advance to the next
  pub fn consume_char(&mut self) -> char {
    let range = self.input.as_slice().char_range_at(self.pos);
    self.pos = range.next;

    range.ch
  }

  // consume chars until `test` -> false
  pub fn consume_while(&mut self, test: |char| -> bool) -> String {
    let mut result = String::new();
    while !self.eof() && test(self.next_char()) {
      result.push(self.consume_char());
    }

    result
  }

  // consume and throw out whitespace
  pub fn consume_whitespace(&mut self) {
    self.consume_while(|c| c.is_whitespace());
  }
}