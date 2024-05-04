# html-parser

Simple HTML parser that parses a HTML string and returns a list of tags and their
attributes without needing ownership over the string.

## Usage

```rust
use crate::HtmlParser;

let html = r#"<html>
    <head>
        <title>Test</title>
    </head>
    <body>
        <h1>Test</h1>
        <p>Test</p>
    </body>
</html>"#;

let parsed = parse_html(html).unwrap();
let root = parsed[0];
assert_eq!(root.tag(), "html");
assert_eq!(root.children().len(), 2);
```

