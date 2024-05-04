#![allow(unused)]

mod parser;
mod reader;

#[derive(Debug, PartialEq, Eq)]
pub struct HtmlAttribute<'a> {
    name: &'a str,
    value: Option<&'a str>,
}

impl<'a> HtmlAttribute<'a> {
    pub fn new(name: &'a str, value: Option<&'a str>) -> Self {
        Self { name, value }
    }
}

#[derive(Debug, Default, PartialEq, Eq)]
pub struct HtmlElement<'a> {
    tag: &'a str,
    attributes: Vec<HtmlAttribute<'a>>,
    children: Vec<HtmlElement<'a>>,
    inner_text: Option<&'a str>,
}

impl<'a> HtmlElement<'a> {
    pub fn new(tag: &'a str) -> Self {
        Self {
            tag,
            ..Default::default()
        }
    }

    pub fn add_attribute(&mut self, name: &'a str, value: Option<&'a str>) {
        self.attributes.push(HtmlAttribute::new(name, value));
    }

    pub fn add_child(&mut self, child: HtmlElement<'a>) {
        self.children.push(child);
    }
}

#[derive(Debug)]
pub enum HtmlError {
    ReaderError(reader::ReadError),
    InvalidAst,
    DecodeFailed,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn html_element() {
        let mut root = HtmlElement::new("html");
        root.add_child(HtmlElement::new("p"));

        assert_eq!(root.children.len(), 1);
        assert_eq!(
            root,
            HtmlElement {
                tag: "html",
                attributes: vec![],
                children: vec![HtmlElement {
                    tag: "p",
                    attributes: vec![],
                    children: vec![],
                    inner_text: None
                }],
                inner_text: None,
            }
        )
    }
}
