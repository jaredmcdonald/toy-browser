//! DOM (only implements text, comment and element nodes)

use std::collections::HashMap;
use css;

pub type AttrMap = HashMap<String, String>;

#[deriving(Show)]
pub struct Node {
  pub children: Vec<Node>,
  pub node_type: NodeType,
}

#[deriving(Show)]
pub enum NodeType {
  Text(String),
  Comment(String),
  Element(ElementData),
  Document(DocumentData),
}

#[deriving(Show)]
pub struct ElementData {
  pub tag_name: String,
  pub attributes: AttrMap,
}

#[deriving(Show)]
pub struct DocumentData {
  stylesheets: Vec<css::Stylesheet>,
}

// constructors
pub fn text (data: String) -> Node {
  Node {
    children: vec![],
    node_type: Text(data),
  }
}

pub fn comment (data: String) -> Node {
  Node {
    children: vec![],
    node_type: Comment(data),
  }
}

pub fn elem (tag_name: String, attrs: AttrMap, children: Vec<Node>) -> Node {
  Node {
    children: children,
    node_type: Element(ElementData {
      attributes: attrs,
      tag_name: tag_name,
    })
  }
}

pub fn document(children: Vec<Node>, stylesheets: Vec<css::Stylesheet>) -> Node {
  Node {
    children: children,
    node_type: Document(DocumentData {
      stylesheets: stylesheets
    })
  }
}

impl Node {
  // pretty print the DOM tree from `&self` down,
  // starting from indent level `indent_level`
  pub fn pretty_print(&self, indent_level: uint) {
    let mut spaces = String::new();
    let mut counter = 0;

    loop {
      if counter == indent_level { break }
      counter += 1;
      spaces.push_str("  ");
    }

    println!("{}{}", spaces, self.node_type);
    for child_node in self.children.iter() {
      child_node.pretty_print(indent_level + 1)
    }
  }
}
