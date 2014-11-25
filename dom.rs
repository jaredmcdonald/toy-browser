//! DOM (only implements text, comment and element nodes)

use std::collections::HashMap;
use std::collections::HashSet;
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
  pub stylesheets: Vec<css::Stylesheet>,
}

// constructors
pub fn text (data: String) -> Node {
  Node {
    children: vec![],
    node_type: NodeType::Text(data),
  }
}

pub fn comment (data: String) -> Node {
  Node {
    children: vec![],
    node_type: NodeType::Comment(data),
  }
}

pub fn elem (tag_name: String, attrs: AttrMap, children: Vec<Node>) -> Node {
  Node {
    children: children,
    node_type: NodeType::Element(ElementData {
      attributes: attrs,
      tag_name: tag_name,
    })
  }
}

pub fn document(children: Vec<Node>, stylesheets: Vec<css::Stylesheet>) -> Node {
  Node {
    children: children,
    node_type: NodeType::Document(DocumentData {
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

impl ElementData {

  // get attribute from self.attributes
  pub fn get_attribute(&self, key: &str) -> Option<&String> {
    self.attributes.get(key)
  }

  // get id
  pub fn id(&self) -> Option<&String> {
    self.get_attribute("id")
  }

  // get classes as HashSet<&str>
  pub fn classes(&self) -> HashSet<&str> {
    match self.get_attribute("class") {
      Some(classlist) => classlist.as_slice().split(' ').collect(),
      None => HashSet::new()
    }
  }

}

impl DocumentData {

  pub fn stylesheets(&self) -> &Vec<css::Stylesheet> {
    &self.stylesheets
  }
}
