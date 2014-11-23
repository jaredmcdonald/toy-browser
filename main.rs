use std::collections::HashMap;

mod dom;
mod parser;
mod html;
mod css;

fn main() {
  println!("Testing module \"dom\"...");
  test_dom();

  println!("Testing module \"html\"...");
  test_html();

  println!("Testing module \"css\"...");
  test_css();
}

fn test_css() {
  let source = "
    .foo,
    h1 {
      color: #000000;
    }

    div {
      width: 500px;
      height: 123px;
    }

    #someId,
    .some-class {
      position: absolute;
    }
  ".to_string();
  let parsed_css = css::parse(source);

  parsed_css.pretty_print();
}

fn test_html() {
  let source = "<html>
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
