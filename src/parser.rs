use crate::{reader::StrReader, HtmlElement, HtmlError};

#[derive(Debug, PartialEq)]
pub enum HtmlAst<'a> {
    StartTag(&'a str),
    Attribute(&'a str, Option<&'a str>),
    EndTag,
    Text(&'a str),
}

pub fn tokenize_html<'a>(data: &'a str) -> Result<Vec<HtmlAst<'a>>, HtmlError> {
    let mut reader = StrReader::new(data);
    let mut ast = Vec::new();

    loop {
        // Skip whitespace
        reader.skip_while(|ch| ch.is_whitespace());

        match reader.seek() {
            Some('<') => {
                reader.skip(1);

                if reader.seek() == Some('/') {
                    ast.push(HtmlAst::EndTag);
                    // Skip until closing bracket
                    reader.read_until('>').map_err(HtmlError::ReaderError)?;
                    reader.skip(1);
                } else if reader.seek() == Some('!') {
                    // Skip comment
                    reader.skip(1);
                    reader.skip_while(|ch| ch != '>');
                    reader.skip(1);
                } else {
                    let tag = reader.read_until('>').map_err(HtmlError::ReaderError)?;
                    reader.skip(1);

                    let (tag, attrs) = match tag.find(' ') {
                        Some(i) => {
                            let (tag, attrs) = tag.split_at(i);
                            (tag, Some(attrs))
                        }
                        None => (tag, None),
                    };

                    ast.push(HtmlAst::StartTag(tag));

                    if let Some(attrs) = attrs {
                        for attr in attrs.trim().split(' ') {
                            println!("attr: {:?}", attr);
                            let (name, value) = match attr.find('=') {
                                Some(i) => {
                                    let (name, value) = attr.split_at(i);
                                    (name, Some(&value[1..]))
                                }
                                None => (attr, None),
                            };

                            let name = name.trim();
                            let value = value.map(|v| v.trim().trim_matches('"'));
                            ast.push(HtmlAst::Attribute(name, value));
                        }
                    }
                }
            }
            Some(_) => {
                let text = reader.read_until('<').map_err(HtmlError::ReaderError)?;
                ast.push(HtmlAst::Text(text));
            }
            None => break,
        }
    }

    Ok(ast)
}

pub fn parse_html(data: &str) -> Result<Vec<HtmlElement>, HtmlError> {
    let mut reader = StrReader::new(data);

    let tokens = tokenize_html(data)?;
    let mut token_stack = Vec::new();
    let mut elements = Vec::new();

    for token in tokens {
        match token {
            HtmlAst::StartTag(element) => {
                token_stack.insert(0, HtmlElement::new(element));
            }
            HtmlAst::Attribute(name, value) => {
                let element = token_stack.first_mut().ok_or(HtmlError::InvalidAst)?;
                element.add_attribute(name, value);
            }
            HtmlAst::EndTag => {
                let element = token_stack.remove(0);
                if let Some(parent) = token_stack.first_mut() {
                    parent.add_child(element);
                } else {
                    elements.push(element);
                }
            }
            HtmlAst::Text(text) => {
                let element = token_stack.first_mut().ok_or(HtmlError::InvalidAst)?;
                element.inner_text = Some(text);
            }
        }
    }
    if token_stack.is_empty() {
        Ok(elements)
    } else {
        Err(HtmlError::InvalidAst)
    }
}

pub fn html_to_string(elements: Vec<HtmlElement<'_>>) -> String {
    let mut html = String::new();
    for element in elements {
        if element.attributes.len() > 0 {
            html.push_str("<");
            html.push_str(element.tag);
            for attr in element.attributes {
                match attr.value {
                    Some(value) => html.push_str(&format!(" {}=\"{}\"", attr.name, value)),
                    None => html.push_str(&format!(" {}", attr.name)),
                }
            }
            html.push_str(">");
            if let Some(text) = element.inner_text {
                html.push_str(text);
            } else {
                html.push_str(&html_to_string(element.children));
            }
            html.push_str(&format!("</{}>", element.tag));
        } else {
            html.push_str(&format!("<{}>", element.tag));
            if let Some(text) = element.inner_text {
                html.push_str(text);
            } else {
                html.push_str(&html_to_string(element.children));
            }
            html.push_str(&format!("</{}>", element.tag));
        }
    }
    html
}
#[cfg(test)]
mod tests {
    use crate::HtmlAttribute;

    use super::*;

    #[test]
    fn single_tag_tokenize() {
        let html = "<button>Hello</button>";
        let tokens = tokenize_html(html).unwrap();
        assert_eq!(
            tokens,
            vec![
                HtmlAst::StartTag("button"),
                HtmlAst::Text("Hello"),
                HtmlAst::EndTag
            ]
        );
    }

    #[test]
    fn tokenize_nested_tag() {
        let html = "<div><button>Hello</button></div>";
        let tokens = tokenize_html(html).unwrap();
        assert_eq!(
            tokens,
            vec![
                HtmlAst::StartTag("div"),
                HtmlAst::StartTag("button"),
                HtmlAst::Text("Hello"),
                HtmlAst::EndTag,
                HtmlAst::EndTag
            ]
        );
    }

    #[test]
    fn tokenize_attr() {
        let html = "<button class=\"btn\">Hello</button>";
        let tokens = tokenize_html(html).unwrap();
        assert_eq!(
            tokens,
            vec![
                HtmlAst::StartTag("button"),
                HtmlAst::Attribute("class", "btn".into()),
                HtmlAst::Text("Hello"),
                HtmlAst::EndTag
            ]
        );
    }

    #[test]
    fn tokenize_attr2() {
        let html = "<button class=\"btn\" disabled>Hello</button>";
        let tokens = tokenize_html(html).unwrap();
        assert_eq!(
            tokens,
            vec![
                HtmlAst::StartTag("button"),
                HtmlAst::Attribute("class", "btn".into()),
                HtmlAst::Attribute("disabled", None),
                HtmlAst::Text("Hello"),
                HtmlAst::EndTag
            ]
        );
    }

    #[test]
    fn decode_html() {
        let html = "<div><button class=\"btn\">Hello</button></div>";
        let element = parse_html(html).unwrap();
        assert_eq!(
            element,
            vec![HtmlElement {
                tag: "div",
                attributes: vec![],
                children: vec![HtmlElement {
                    tag: "button",
                    attributes: vec![HtmlAttribute::new("class", "btn".into())],
                    children: vec![],
                    inner_text: Some("Hello")
                }],
                inner_text: None
            }]
        );
    }

    #[test]
    fn decode_html_attr() {
        let html = "<button class=\"btn\" disabled>Hello</button>";
        let element = parse_html(html).unwrap();
        assert_eq!(
            element,
            vec![HtmlElement {
                tag: "button",
                attributes: vec![
                    HtmlAttribute::new("class", Some("btn")),
                    HtmlAttribute::new("disabled", None)
                ],
                children: vec![],
                inner_text: Some("Hello")
            }]
        );
    }

    #[test]
    fn decode_comment() {
        let html = "<!-- comment -->";
        let element = parse_html(html).unwrap();
        assert_eq!(element, vec![]);
    }

    #[test]
    fn encode_html() {
        let elements = vec![HtmlElement {
            tag: "div",
            attributes: vec![],
            children: vec![HtmlElement {
                tag: "button",
                attributes: vec![HtmlAttribute::new("class", Some("btn"))],
                children: vec![],
                inner_text: Some("Hello"),
            }],
            inner_text: None,
        }];
        let html = html_to_string(elements);
        assert_eq!(html, "<div><button class=\"btn\">Hello</button></div>");
    }

    #[test]
    fn encode_html_attr() {
        let elements = vec![HtmlElement {
            tag: "button",
            attributes: vec![
                HtmlAttribute::new("class", Some("btn")),
                HtmlAttribute::new("disabled", None),
            ],
            children: vec![],
            inner_text: Some("Hello"),
        }];
        let html = html_to_string(elements);
        assert_eq!(html, "<button class=\"btn\" disabled>Hello</button>");
    }
}
