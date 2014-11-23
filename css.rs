//! CSS parser
//! (only implements a very basic subset of CSS)

use std::ascii::OwnedAsciiExt;
use std::num::FromStrRadix;
use parser;

// id, class, tag
pub type Specificity = (uint, uint, uint);

#[deriving(Show)]
pub struct Stylesheet {
  rules: Vec<Rule>,
}

impl Stylesheet {
  // pretty print a stylesheet
  pub fn pretty_print(&self) {
    fn indent(indent_level: uint) -> String {
      let mut spaces = String::new();
      let mut counter = 0u;

      loop {
        if counter == indent_level { break }
        counter += 1;
        spaces.push_str("  ");
      }

      spaces
    }

    for rule in self.rules.iter() {

      for selector in rule.selectors.iter() {
        println!("{}", selector)
      }
      for declaration in rule.declarations.iter() {
        println!("{}{}", indent(1), declaration.name)
        println!("{}{}", indent(2), declaration.value)
      }

    }
  }
}

#[deriving(Show)]
struct Rule {
  selectors: Vec<Selector>,
  declarations: Vec<Declaration>,
}

#[deriving(Show)]
enum Selector {
  Simple(SimpleSelector),
}

impl Selector {
  // get back (id, class, tag) specificity of a Selector
  pub fn specificity(&self) -> Specificity {
    let Simple(ref simple) = *self;
    let id = simple.id.iter().len();
    let class = simple.class.len();
    let tag = simple.tag_name.iter().len();
    (id, class, tag)
  }
}

#[deriving(Show)]
struct SimpleSelector {
  tag_name: Option<String>,
  id: Option<String>,
  class: Vec<String>,
}

#[deriving(Show)]
struct Declaration {
  name: String,
  value: Value,
}

#[deriving(Show)]
enum Value {
  Keyword(String),
  Length(f32, Unit),
  ColorValue(Color),
}

#[deriving(Show)]
enum Unit {
  Px,
  Percentage,
  Em,
  UnknownUnit,
}

#[deriving(Show)]
struct Color {
  r: u8,
  g: u8,
  b: u8,
  a: u8,
}

struct Parser {
  p : parser::Parser,
}

impl Parser {

  // parse rules until EOF
  fn parse_rules(&mut self) -> Vec<Rule> {
    let mut rules = Vec::new();
    loop {
      self.consume_whitespace_and_comments();
      if self.p.eof() { break }
      rules.push(self.parse_rule());
    }
    rules
  }

  // parse string of valid identifier characters
  fn parse_identifier(&mut self) -> String {
    self.p.consume_while(valid_identifier_char)
  }

  // parse and return a Rule
  fn parse_rule(&mut self) -> Rule {
    Rule {
      selectors: self.parse_selectors(),
      declarations: self.parse_declarations()
    }
  }

  // consume and discard whitespace and comments
  fn consume_whitespace_and_comments(&mut self) {
    self.p.consume_whitespace();
    if self.p.starts_with("/*") {
      assert!(self.p.consume_char() == '/');
      assert!(self.p.consume_char() == '*');

      loop {
        if self.p.starts_with("*/") {
          break;
        }
        self.p.consume_char();
      }

      assert!(self.p.consume_char() == '*');
      assert!(self.p.consume_char() == '/');
    }
    self.p.consume_whitespace();

    // handle case where we have comment-whitespace-comment
    if self.p.starts_with("/*") {
      self.consume_whitespace_and_comments();
    }
  }

  // parse list of simple selectors
  fn parse_selectors(&mut self) -> Vec<Selector> {
    let mut selectors = Vec::new();

    loop {
      selectors.push(Simple(self.parse_simple_selector()));

      self.consume_whitespace_and_comments();

      match self.p.next_char() {
        ',' => {
          self.p.consume_char();
          self.consume_whitespace_and_comments();
        }
        '{' => break,
        c => break // instead we should exit/throw exception here
      }
    }

    // sort by specificity (highest first)
    selectors.sort_by(|a,b| b.specificity().cmp(&a.specificity()));
    
    selectors
  }

  // parse declarations in declaration block
  fn parse_declarations(&mut self) -> Vec<Declaration> {
    assert!(self.p.consume_char() == '{');
    
    let mut declarations = Vec::new();
    loop {
      self.consume_whitespace_and_comments();
      
      if self.p.next_char() == '}' {
        // end of declaration block
        self.p.consume_char();
        break;
      }

      declarations.push(self.parse_declaration());
    }

    declarations
  }

  // parse and return a single Declaration
  fn parse_declaration(&mut self) -> Declaration {
    let name = self.parse_identifier();

    self.consume_whitespace_and_comments();
    assert!(self.p.consume_char() == ':');
    self.consume_whitespace_and_comments();

    let value = self.parse_value();

    self.consume_whitespace_and_comments();
    assert!(self.p.consume_char() == ';');

    Declaration {
      name: name,
      value: value,
    }
  }

  // parse a declaration value
  fn parse_value(&mut self) -> Value {
    match self.p.next_char() {
      '0'...'9' => self.parse_length(),
      '#' => self.parse_color(),
      _ => Keyword(self.parse_identifier())
    }
  }

  // parse a Value::Length, e.g., "123.4px"
  fn parse_length(&mut self) -> Value {
    Length(self.parse_float(), self.parse_unit())
  }

  // parse 32-bit float
  fn parse_float(&mut self) -> f32 {
    let s = self.p.consume_while(|c| match c {
      '0'...'9' | '.' => true,
      _ => false
    });
    let f: Option<f32> = from_str(s.as_slice());
    f.unwrap()
  }

  // parse unit (only support px for now)
  fn parse_unit(&mut self) -> Unit {
    match self.parse_unit_value().as_slice() {
      "px" => Px,
      "%" => Percentage,
      "em" => Em,
      _ => UnknownUnit // TODO: handle this better
    }
  }

  // parse a valid unit value
  // TODO: improve `consume_while` test
  fn parse_unit_value(&mut self) -> String {
    self.p.consume_while(|c| !c.is_whitespace() && c != ';').into_ascii_lower()
  }

  // parse hex color (only hex for now)
  fn parse_color(&mut self) -> Value {
    assert!(self.p.consume_char() == '#');
    ColorValue(Color {
      r: self.parse_hex_pair(),
      g: self.parse_hex_pair(),
      b: self.parse_hex_pair(),
      a: 255
    })
  }

  // parse a hex pair
  fn parse_hex_pair(&mut self) -> u8 {
    let s = self.p.input.as_slice().slice(self.p.pos, self.p.pos + 2);
    self.p.pos += 2;
    FromStrRadix::from_str_radix(s, 0x10).unwrap()
  }

  // parse a single selector
  fn parse_simple_selector(&mut self) -> SimpleSelector {
    let mut selector = SimpleSelector {
      tag_name: None,
      id: None,
      class: Vec::new(),
    };

    while !self.p.eof() {
      match self.p.next_char() {
        '#' => {
          self.p.consume_char();
          selector.id = Some(self.parse_identifier());
        }
        '.' => {
          self.p.consume_char();
          selector.class.push(self.parse_identifier());
        }
        '*' => {
          self.p.consume_char();
        }
        c if valid_identifier_char(c) => {
          selector.tag_name = Some(self.parse_identifier())
        }
        _ => break
      }
    }

    selector
  }
}

// parse a source string and return a Stylesheet
pub fn parse(source: String) -> Stylesheet {
  let mut parser = Parser {
    p: parser::Parser {
      pos: 0u,
      input: source,
    }
  };
  Stylesheet {
    rules: parser.parse_rules()
  }
}

// is `c` a valid identifier char?
// (needs better unicode support)
fn valid_identifier_char(c: char) -> bool {
  match c {
    'a'...'z' | 'A'...'Z' | '0'...'9' | '-' | '_' => true,
    _ => false,
  }
}
