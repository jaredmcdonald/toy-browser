use std::collections::HashMap;
use dom;

pub struct Parser {
  pos: uint,
  input: String,
}

impl Parser {
  // read next char
  fn next_char(&self) -> char {
    self.input.as_slice().char_at(self.pos)
  }

  // read next `n` chars and return as a String
  fn next_chars(&self, n: uint) -> String {
    let mut result = String::new();
    let mut counter = 0u;

    loop {
      if counter == n { break }
      result.push(self.input.as_slice().char_at(self.pos + counter));
      counter += 1;
    }

    result
  }

  // do the next chars start with string `s`?
  fn starts_with(&self, s: &str) -> bool {
    self.input.as_slice().slice_from(self.pos).starts_with(s)
  }

  // have we consumed all the input?
  fn eof(&self) -> bool {
    self.pos >= self.input.len()
  }

  // return current char and advance to the next
  fn consume_char(&mut self) -> char {
    let range = self.input.as_slice().char_range_at(self.pos);
    self.pos = range.next;

    range.ch
  }

  // consume chars until `test` -> false
  fn consume_while(&mut self, test: |char| -> bool) -> String {
    let mut result = String::new();
    while !self.eof() && test(self.next_char()) {
      result.push(self.consume_char());
    }

    result
  }

  // consume comment body
  fn consume_comment(&mut self) -> String {
    let mut result = String::new();
    while !self.eof() && self.next_chars(3).as_slice() != "-->" {
      result.push(self.consume_char());
    }

    result
  }

  // consume and throw out whitespace
  fn consume_whitespace(&mut self) {
    self.consume_while(|c| c.is_whitespace());
  }

  // parse tag or attr name
  fn parse_tag_name(&mut self) -> String {
    self.consume_while(|c| match c {
      'a'...'z' | 'A'...'Z' | '0'...'9' => true,
      _ => false
    })
  }

  // parse a node
  // return dom::Node (deciding type)
  fn parse_node(&mut self) -> dom::Node {
    match self.next_char() {
      '<' => self.element_or_comment(),
      _   => self.parse_text()
    }
  }

  // decide whether to parse node as element
  // or comment
  fn element_or_comment(&mut self) -> dom::Node {
    if self.next_chars(4).as_slice() == "<!--" {
      self.parse_comment()
    } else {
      self.parse_element()
    }
  }

  // parse text node
  // return dom::Node
  fn parse_text(&mut self) -> dom::Node {
    dom::text(self.consume_while(|c| c != '<'))
  }

  // parse comment node
  fn parse_comment(&mut self) -> dom::Node {
    assert!(self.consume_char() == '<');
    assert!(self.consume_char() == '!');
    assert!(self.consume_char() == '-');
    assert!(self.consume_char() == '-');

    let comment = self.consume_comment();

    assert!(self.consume_char() == '-');
    assert!(self.consume_char() == '-');
    assert!(self.consume_char() == '>');

    dom::comment(comment)
  }

  // parse element node
  // return dom::Node
  fn parse_element(&mut self) -> dom::Node {
    assert!(self.consume_char() == '<');
    let tag_name = self.parse_tag_name();
    let attrs = self.parse_attributes();
    assert!(self.consume_char() == '>');

    let children = self.parse_nodes();

    assert!(self.consume_char() == '<');
    assert!(self.consume_char() == '/');
    assert!(self.parse_tag_name() == tag_name);
    assert!(self.consume_char() == '>');

    dom::elem(tag_name, attrs, children)
  }

  // parse attribute pair (name="value")
  // return tuple (name, value)
  fn parse_attr(&mut self) -> (String, String) {
    let name = self.parse_tag_name();
    assert!(self.consume_char() == '=');
    let value = self.parse_attr_value();

    (name, value)
  }

  // parse attr value within quotes
  // return string value
  fn parse_attr_value(&mut self) -> String {
    let open_quote = self.consume_char();
    assert!(open_quote == '"' || open_quote == '\'');
    let value = self.consume_while(|c| c != open_quote);
    assert!(self.consume_char() == open_quote);

    value
  }

  // parse all attributes within element node
  // return AttrMap of attributes
  fn parse_attributes(&mut self) -> dom::AttrMap {
    let mut attributes = HashMap::new();
    loop {
      self.consume_whitespace();
      if self.next_char() == '>' { break }
      let (name, value) = self.parse_attr();
      attributes.insert(name, value);
    }

    attributes
  }

  // parse child nodes
  // return vector of nodes
  fn parse_nodes(&mut self) -> Vec<dom::Node> {
    let mut nodes = Vec::new();
    loop {
      self.consume_whitespace();
      if self.eof() || self.starts_with("</") { break }
      nodes.push(self.parse_node());
    }

    nodes
  }

  // parse HTML source and return a node
  // create root "html" node if there isn't
  // already a single root
  pub fn parse(source: String) -> dom::Node {
    let mut nodes = Parser {
      pos: 0u,
      input: source,
    }.parse_nodes();

    if nodes.len() == 1 {
      nodes.swap_remove(0).unwrap()
    } else {
      dom::elem("html".to_string(), HashMap::new(), nodes)
    }
  }
}
