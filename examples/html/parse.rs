use html_parser::Dom;

static HTML_STRING: &str = include_str!("./index.html");

fn main() {
    assert!(Dom::parse(HTML_STRING).is_ok());
}
