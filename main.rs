use std::collections::HashMap;

mod dom;
mod parser;
mod html;
mod css;
mod style;

fn main() {
  println!("\nTesting module \"dom\"...\n");
  test_dom();

  println!("\nTesting module \"html\"...\n");
  test_html();

  println!("\nTesting module \"css\"...\n");
  test_css();

  println!("\nTesting module \"style\"...\n");
  test_style();
}

fn test_style() {
  let stylesheet = css::parse(".foo { color: #000000; }".to_string());
  let dom_tree = html::parse("<body><div class=\"foo\">hello world</div></body>".to_string());
  let styled_node = style::style_tree(&dom_tree, &stylesheet);
  println!("(1)\n{}\n", styled_node);

  let html_source = "
    <html>
      <style>
        .test, p { color: red; }
      </style>
      <body>
        <!-- A comment -->
        <h1>Title</h1>
          <div id=\"main\" class=\"test\">
              <p>Hello <em>world</em>!</p>
          </div>
      </body>
    </html>".to_string();
  let dom_tree_2 = html::parse(html_source);
  let new_vec = Vec::new();
  let stylesheets = match dom_tree_2.node_type {
    dom::NodeType::Document(ref elem) => elem.stylesheets(),
    _ => &new_vec
  };
  let styled_node_2 = style::style_tree(&dom_tree_2, &stylesheets[0]);

  println!("(2)\n{}\n", styled_node_2);
}

fn test_css() {
  let source = "
    .foo,
    h1 {
      color: #000000;
      font-size: 1.5em;
    }

    /*
      a css comment
    */ /* and another one */ /*
      how about a third in a row?
    */

    div { /* another css comment */
      width: 500px;
      height: 23.5%;
    }

    #someId,
    .some-class {
      position: /*here we have a really annoying comment*/ absolute;
    }
  ".to_string();
  let parsed_css = css::parse(source);

  parsed_css.pretty_print();
}

fn test_html() {
  let source = "<html>
      <style>
        .foo { color: red; }
      </style>
      <body>
          <!-- A comment -->
          <h1>Title</h1>
          <div id=\"main\" class=\"test\">
              <p>Hello <em>world</em>!</p>
          </div>
      </body>
  </html>".to_string();

  let parsed_html = html::parse(source);
  parsed_html.pretty_print(0);
}

fn test_dom() {
  // just test out nodes for now
  let comment_node = dom::comment("this is a comment".to_string());
  let text_node = dom::text("here is some text".to_string());

  let mut children = Vec::new();
  children.push(comment_node);
  children.push(text_node);

  let mut attrs = HashMap::new();
  attrs.insert("id".to_string(), "someId".to_string());
  attrs.insert("class".to_string(), "element-class".to_string());

  let element_node = dom::elem("p".to_string(), attrs, children);
  let comment_node2 = dom::comment("second comment".to_string());

  let mut children2 = Vec::new();
  children2.push(element_node);
  children2.push(comment_node2);

  let element_node_parent = dom::elem("div".to_string(), HashMap::new(), children2);

  element_node_parent.pretty_print(0);
}
