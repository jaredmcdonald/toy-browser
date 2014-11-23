//! HTML parser
//! (only implements a very basic subset
//!  of HTML)

use std::collections::HashMap;
use parser;
use dom;

pub struct Parser {
  p : parser::Parser,
}

impl Parser {
  // consume comment body
  fn consume_comment(&mut self) -> String {
    let mut result = String::new();
    while !self.p.eof() && !self.p.starts_with("-->") {
      result.push(self.p.consume_char());
    }

    result
  }

  // parse tag or attr name
  fn parse_tag_name(&mut self) -> String {
    self.p.consume_while(|c| match c {
      'a'...'z' | 'A'...'Z' | '0'...'9' => true,
      _ => false
    })
  }

  // parse a node
  // return dom::Node (deciding type)
  fn parse_node(&mut self) -> dom::Node {
    match self.p.next_char() {
      '<' => self.element_or_comment(),
      _   => self.parse_text()
    }
  }

  // decide whether to parse node as element
  // or comment
  fn element_or_comment(&mut self) -> dom::Node {
    if self.p.starts_with("<!--") {
      self.parse_comment()
    } else {
      self.parse_element()
    }
  }

  // parse text node
  // return dom::Node
  fn parse_text(&mut self) -> dom::Node {
    dom::text(self.p.consume_while(|c| c != '<'))
  }

  // parse comment node
  fn parse_comment(&mut self) -> dom::Node {
    assert!(self.p.consume_char() == '<');
    assert!(self.p.consume_char() == '!');
    assert!(self.p.consume_char() == '-');
    assert!(self.p.consume_char() == '-');

    let comment = self.consume_comment();

    assert!(self.p.consume_char() == '-');
    assert!(self.p.consume_char() == '-');
    assert!(self.p.consume_char() == '>');

    dom::comment(comment)
  }

  // parse element node
  // return dom::Node
  fn parse_element(&mut self) -> dom::Node {
    assert!(self.p.consume_char() == '<');
    let tag_name = self.parse_tag_name();
    let attrs = self.parse_attributes();
    assert!(self.p.consume_char() == '>');

    let children = self.parse_nodes();

    assert!(self.p.consume_char() == '<');
    assert!(self.p.consume_char() == '/');
    assert!(self.parse_tag_name() == tag_name);
    assert!(self.p.consume_char() == '>');

    dom::elem(tag_name, attrs, children)
  }

  // parse attribute pair (name="value")
  // return tuple (name, value)
  fn parse_attr(&mut self) -> (String, String) {
    let name = self.parse_tag_name();
    assert!(self.p.consume_char() == '=');
    let value = self.parse_attr_value();

    (name, value)
  }

  // parse attr value within quotes
  // return string value
  fn parse_attr_value(&mut self) -> String {
    let open_quote = self.p.consume_char();
    assert!(open_quote == '"' || open_quote == '\'');
    let value = self.p.consume_while(|c| c != open_quote);
    assert!(self.p.consume_char() == open_quote);

    value
  }

  // parse all attributes within element node
  // return AttrMap of attributes
  fn parse_attributes(&mut self) -> dom::AttrMap {
    let mut attributes = HashMap::new();
    loop {
      self.p.consume_whitespace();
      if self.p.next_char() == '>' { break }
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
      self.p.consume_whitespace();
      if self.p.eof() || self.p.starts_with("</") { break }
      nodes.push(self.parse_node());
    }

    nodes
  }
}

// parse HTML source and return a node
// create root "html" node if there isn't
// already a single root
pub fn parse(source: String) -> dom::Node {
  let mut nodes = Parser {
    p: parser::Parser {
      pos: 0u,
      input: source,
    }
  }.parse_nodes();

  if nodes.len() == 1 {
    nodes.swap_remove(0).unwrap()
  } else {
    dom::elem("html".to_string(), HashMap::new(), nodes)
  }
}
