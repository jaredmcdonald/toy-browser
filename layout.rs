//! a layout tree module
//! px sizes for now

use style;

#[deriving(Show)]
struct Dimensions {
  content: Rect,
  padding: EdgeSizes,
  border: EdgeSizes,
  margin: EdgeSizes,
}

#[deriving(Show)]
struct Rect {
  x: f32,
  y: f32,
  width: f32,
  height: f32,
}

#[deriving(Show)]
struct EdgeSizes {
  left: f32,
  right: f32,
  top: f32,
  bottom : f32,
}

#[deriving(Show)]
struct LayoutBox<'a> {
  dimensions: Dimensions,
  box_type: BoxType<'a>,
  children: Vec<LayoutBox<'a>>,
}

#[deriving(Show)]
enum BoxType<'a> {
  BlockNode(&'a style::StyledNode<'a>),
  InlineNode(&'a style::StyledNode<'a>),
  AnonymousBlock,
}

#[deriving(Show)]
pub enum Display {
  Inline,
  Block,
  None,
}

// build and return a layout tree
pub fn build_layout_tree<'a>(style_node: &'a style::StyledNode<'a>) -> LayoutBox<'a> {
  // root element
  let mut root = LayoutBox::new(match style_node.display() {
    Display::Block => BoxType::BlockNode(style_node),
    Display::Inline => BoxType::InlineNode(style_node),
    Display::None => panic!("root node has display: none"),
  });

  for child in style_node.children.iter() {
    match child.display() {
      Display::Block => root.children.push(build_layout_tree(child)),
      Display::Inline => root.get_inline_container().children.push(build_layout_tree(child)),
      Display::None => {},
    }
  }

  root
}

impl<'a> LayoutBox<'a> {
  fn new(box_type: BoxType) -> LayoutBox {
    LayoutBox {
      box_type: box_type,
      dimensions: Dimensions { // 0.0: temporary default
        content: Rect {
          x: 0.0,
          y: 0.0,
          width: 0.0,
          height: 0.0,
        },
        padding: EdgeSizes {
          left: 0.0,
          right: 0.0,
          top: 0.0,
          bottom: 0.0,
        },
        border: EdgeSizes {
          left: 0.0,
          right: 0.0,
          top: 0.0,
          bottom: 0.0,
        },
        margin: EdgeSizes {
          left: 0.0,
          right: 0.0,
          top: 0.0,
          bottom: 0.0,
        },
      },
      children: Vec::new(),
    }
  }

  fn get_inline_container(&mut self) -> &mut LayoutBox<'a> {
    match self.box_type {
      BoxType::InlineNode(_) | BoxType::AnonymousBlock => self,
      BoxType::BlockNode(_) => {
        match self.children.last() {
          Some(&LayoutBox { box_type: BoxType::AnonymousBlock,..}) => {}
          _ => self.children.push(LayoutBox::new(BoxType::AnonymousBlock))
        }
        self.children.last_mut().unwrap()
      }
    }
  }

  // print out the tree
  pub fn pretty_print(&self, indent_level: uint) {
    let mut spaces = String::new();
    let mut counter = 0;

    loop {
      if counter == indent_level { break }
      counter += 1;
      spaces.push_str("  ");
    }

    // println!("{}{}", spaces, self.dimensions);
    println!("{}{}", spaces, self.box_type);

    for child in self.children.iter() {
      child.pretty_print(indent_level + 1);
    }
  }
}
