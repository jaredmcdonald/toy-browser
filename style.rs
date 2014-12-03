///! matches DOM tree to style tree
///! (lots of real-browser stuff not implemented)

use std::collections::HashMap;
use css;
use dom;
use layout;

pub type PropertyMap = HashMap<String, css::Value>;
pub type MatchedRule<'a> = (css::Specificity, &'a css::Rule);

#[deriving(Show)]
pub struct StyledNode<'a> {
  pub node: &'a dom::Node,
  pub specified_values: PropertyMap,
  pub children: Vec<StyledNode<'a>>,
}

impl<'a> StyledNode<'a> {
  // get value of `name` if it has it
  pub fn value(&self, name: &str) -> Option<css::Value> {
    self.specified_values.get(name).map(|v| v.clone())
  }

  // get 'display' value (default: inline)
  pub fn display(&self) -> layout::Display {
    match self.value("display") {
      Some(css::Value::Keyword(s)) => match s.as_slice() {
        "block" => layout::Display::Block,
        "none" => layout::Display::None,
        _ => layout::Display::Inline,
      },
      _ => layout::Display::Inline
    }
  }
}

// does `elem` match `selector`?
fn matches(elem: &dom::ElementData, selector: &css::Selector) -> bool {
  match *selector {
    css::Selector::Simple(ref simple_selector) => matches_simple_selector(elem, simple_selector)
  }
}

// does `elem` match the given `css::SimpleSelector`?
fn matches_simple_selector(elem: &dom::ElementData, selector: &css::SimpleSelector) -> bool {
  // type
  if selector.tag_name.iter().any(|name| elem.tag_name != *name) {
    return false;
  }

  // check id
  if selector.id.iter().any(|id| elem.id() != Some(id)) {
    return false;
  }

  // check class
  let elem_classes = elem.classes();
  if selector.class.iter().any(|class| !elem_classes.contains(&class.as_slice())) {
    return false;
  }

  // it matches
  true
}

// match a single css::Rule to a dom::Element
// returns a MatchedRule if there's a match, None otherwise
fn match_rule<'a> (elem: &dom::ElementData, rule: &'a css::Rule) -> Option<MatchedRule<'a>> {
  rule.selectors.iter().find(|selector| matches(elem, *selector))
    .map(|selector| (selector.specificity(), rule))
}

// return rules that match the given element
fn matching_rules<'a> (elem: &dom::ElementData, stylesheet: &'a css::Stylesheet) -> Vec<MatchedRule<'a>> {
  stylesheet.rules.iter().filter_map(|rule| match_rule(elem, rule)).collect()
}

// apply styles to an element, returning specified values
fn specified_values(elem: &dom::ElementData, stylesheet: &css::Stylesheet) -> PropertyMap {
  let mut values = HashMap::new();
  let mut rules = matching_rules(elem, stylesheet);

  // go through rules in order of specificity
  rules.sort_by(|&(a, _), &(b, _)| a.cmp(&b));

  for &(_, rule) in rules.iter() {
    for declaration in rule.declarations.iter() {
      values.insert(declaration.name.clone(), declaration.value.clone());
    }
  }

  values
}

// create and return style tree
pub fn style_tree<'a>(root: &'a dom::Node, stylesheet: &'a css::Stylesheet) -> StyledNode<'a> {
  StyledNode {
    node: root,
    specified_values: match root.node_type {
      dom::NodeType::Element(ref elem) => specified_values(elem, stylesheet),
      _ => HashMap::new()
    },
    children: root.children.iter().map(|child| style_tree(child, stylesheet)).collect()
  }
}
