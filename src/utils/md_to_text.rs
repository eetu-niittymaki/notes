use pulldown_cmark::{Event, Parser, Tag, TagEnd};

pub fn md_to_text(markdown: &str) -> String {
    let parser = Parser::new(markdown);
    let mut output = String::new();

    for event in parser {
        match event {
            Event::Text(text) => {
                output.push_str(&text);
            }

            Event::Code(text) => {
                output.push_str(&text);
            }

            Event::SoftBreak => {
                output.push('\n');
            }

            Event::HardBreak => {
                output.push('\n');
            }

            Event::Start(Tag::Paragraph)
            | Event::Start(Tag::Heading { .. })
            | Event::Start(Tag::Item)
            | Event::Start(Tag::BlockQuote(_)) => {
                // Don't add anything here.
            }

            Event::End(TagEnd::Paragraph)
            | Event::End(TagEnd::Heading(_))
            | Event::End(TagEnd::Item)
            | Event::End(TagEnd::BlockQuote(_)) => {
                output.push('\n');
            }

            _ => {}
        }
    }

    output
}