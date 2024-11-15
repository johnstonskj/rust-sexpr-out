use objio::{HasOptions, ObjectWriter};
use pretty_assertions::assert_eq;
use sexpr_out::value::Value;
use sexpr_out::writer::{LanguageStyle, Options, Writer};

#[test]
fn test_pretty_print_short_list_racket() {
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
        "(\"hello\" \"this\" \"is\"\n \"a\" \"lisp\" \"list\")\n".to_string()
    );
}

#[test]
fn test_pretty_print_short_nested_list_racket() {
    let writer = Writer::default().pretty_printed(true).with_options(
        Options::default()
            .with_line_width(20)
            .with_style(LanguageStyle::Racket),
    );

    let list = Value::from(vec![
        Value::from(1),
        Value::from(2),
        Value::from(3),
        Value::from(vec![
            Value::from(4),
            Value::from(5),
            Value::from(vec![
                Value::from(6),
                Value::from(7),
                Value::from(vec![Value::from(8)]),
            ]),
        ]),
        Value::from(9),
        Value::from(10),
    ]);

    //           1         2         3
    // 0123456789012345678901234567890
    // (1 2 3
    //  (4 5 (6 7 (8))) 9
    //  10)

    assert_eq!(
        writer.write_to_string(&list).unwrap(),
        "(1 2 3\n (4 5 (6 7 (8))) 9\n 10)\n".to_string()
    );
}
