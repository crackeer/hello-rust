use scraper::{Html, Selector};
static HTML_STRING: &str = include_str!("./open.html");

fn main() {
    let document = Html::parse_document(HTML_STRING);
    let script_selector = Selector::parse("script").unwrap();

    for script in document.select(&script_selector) {
        let script_content = script.inner_html();
        println!("JavaScript code: {}", script_content);
    }
}
