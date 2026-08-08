use scraper::{Html, Node};

pub fn html_to_text(html: &str) -> String {
    let document = Html::parse_document(html);
    let mut text = String::new();

    for node in document.tree.root().descendants() {
        if let Node::Text(value) = node.value() {
            text.push_str(value);
        }
    }

    text
}