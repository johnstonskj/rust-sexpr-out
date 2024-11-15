# Package sexpr-out

Tools for writing S-Expressions from Rust data.

# Example

```rust
use objio::{HasOptions, ObjectWriter};
use pretty_assertions::assert_eq;
use sexpr_out::value::Value;
use sexpr_out::writer::{LanguageStyle, Options, Writer};

let writer = Writer::default().pretty_printed(true).with_options(
    Options::default()
        .with_line_width(20)
        .with_style(LanguageStyle::Racket),
);

let list = Value::from(vec![
    Value::from("hello"),
    Value::from("this"),
    Value::from("is"),
    Value::from("a"),
    Value::from("lisp"),
    Value::from("list"),
]);

//           1         2         3
// 0123456789012345678901234567890
// ("hello" "this" "is"
//   "a" "lisp" "list")

assert_eq!(
    writer.write_to_string(&list).unwrap(),
    "(\"hello\" \"this\" \"is\"\n \"a\" \"lisp\" \"list\")".to_string()
);
```
