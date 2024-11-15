use objio::{HasOptions, ObjectWriter};
use pretty_assertions::assert_eq;
use sexpr_out::value::Value;
use sexpr_out::writer::{LanguageStyle, Options, Writer};

#[test]
fn test_print_single_boolean_racket() {
    let writer =
        Writer::default().with_options(Options::default().with_style(LanguageStyle::Racket));

    assert_eq!(
        writer.write_to_string(&Value::from(true)).unwrap(),
        "#t".to_string()
    );

    assert_eq!(
        writer.write_to_string(&Value::from(false)).unwrap(),
        "#f".to_string()
    );
}

#[test]
fn test_print_single_boolean_treesitter() {
    let writer =
        Writer::default().with_options(Options::default().with_style(LanguageStyle::TreeSitter));

    assert_eq!(
        writer.write_to_string(&Value::from(true)).unwrap(),
        "true".to_string()
    );

    assert_eq!(
        writer.write_to_string(&Value::from(false)).unwrap(),
        "false".to_string()
    );
}

#[test]
fn test_print_single_boolean_clisp() {
    let writer =
        Writer::default().with_options(Options::default().with_style(LanguageStyle::CommonLisp));

    assert_eq!(
        writer.write_to_string(&Value::from(true)).unwrap(),
        "t".to_string()
    );

    assert_eq!(
        writer.write_to_string(&Value::from(false)).unwrap(),
        "nil".to_string()
    );
}

#[test]
fn test_print_single_boolean_scheme() {
    let writer =
        Writer::default().with_options(Options::default().with_style(LanguageStyle::Scheme));

    assert_eq!(
        writer.write_to_string(&Value::from(true)).unwrap(),
        "#t".to_string()
    );

    assert_eq!(
        writer.write_to_string(&Value::from(false)).unwrap(),
        "#f".to_string()
    );
}

#[test]
fn test_print_single_boolean_elisp() {
    let writer =
        Writer::default().with_options(Options::default().with_style(LanguageStyle::EmacsLisp));

    assert_eq!(
        writer.write_to_string(&Value::from(true)).unwrap(),
        "t".to_string()
    );

    assert_eq!(
        writer.write_to_string(&Value::from(false)).unwrap(),
        "nil".to_string()
    );
}

#[test]
fn test_print_single_char_racket() {
    let writer =
        Writer::default().with_options(Options::default().with_style(LanguageStyle::Racket));

    assert_eq!(
        writer.write_to_string(&Value::from('a')).unwrap(),
        r"#\a".to_string()
    );

    assert_eq!(
        writer.write_to_string(&Value::from('\n')).unwrap(),
        r"#\newline".to_string()
    );
    //    assert_eq!('§'.to_string_for(LanguageStyle::Racket), r"#\§".to_string());
    //    assert_eq!(
    //        '\u{3001}'.to_string_for(LanguageStyle::Racket),
    //        r"#\u3001".to_string()
    //    );
    //    assert_eq!(
    //        '\u{E0101}'.to_string_for(LanguageStyle::Racket),
    //        r"#\U0E0101".to_string()
    //    );
}

#[test]
fn test_print_single_char_treesitter() {
    let writer =
        Writer::default().with_options(Options::default().with_style(LanguageStyle::TreeSitter));

    assert_eq!(
        writer.write_to_string(&Value::from('a')).unwrap(),
        r"'a'".to_string()
    );

    assert_eq!(
        writer.write_to_string(&Value::from('\n')).unwrap(),
        r"'\n'".to_string()
    );
    //    assert_eq!(
    //        '§'.to_string_for(LanguageStyle::TreeSitter),
    //        r"'§'".to_string()
    //    );
    //    assert_eq!(
    //        '\u{30F0}'.to_string_for(LanguageStyle::TreeSitter),
    //        r"'ヰ'".to_string()
    //    );
    //    assert_eq!(
    //        '\u{E0101}'.to_string_for(LanguageStyle::TreeSitter),
    //        r"'\u{e0101}'".to_string()
    //    );
}

#[test]
fn test_print_single_char_clisp() {
    let writer =
        Writer::default().with_options(Options::default().with_style(LanguageStyle::CommonLisp));

    assert_eq!(
        writer.write_to_string(&Value::from('a')).unwrap(),
        r"#\a".to_string()
    );

    assert_eq!(
        writer.write_to_string(&Value::from('\n')).unwrap(),
        r"#\Newline".to_string()
    );
    //    assert_eq!(
    //        '§'.to_string_for(LanguageStyle::CommonLisp),
    //        r"#\§".to_string()
    //    );
    //    assert_eq!(
    //        '\u{3001}'.to_string_for(LanguageStyle::CommonLisp),
    //        r"#\U3001".to_string()
    //    );
    //    assert_eq!(
    //        '\u{E0101}'.to_string_for(LanguageStyle::CommonLisp),
    //        r"#\U0E0101".to_string()
    //    );
}

#[test]
fn test_print_single_char_scheme() {
    let writer =
        Writer::default().with_options(Options::default().with_style(LanguageStyle::Scheme));

    assert_eq!(
        writer.write_to_string(&Value::from('a')).unwrap(),
        r"#\a".to_string()
    );

    assert_eq!(
        writer.write_to_string(&Value::from('\n')).unwrap(),
        r"#\newline".to_string()
    );
    //    assert_eq!('§'.to_string_for(LanguageStyle::Scheme), r"#\§".to_string());
    //    assert_eq!(
    //        '\u{3001}'.to_string_for(LanguageStyle::Scheme),
    //        r"#\x3001".to_string()
    //    );
    //    assert_eq!(
    //        '\u{E0101}'.to_string_for(LanguageStyle::Scheme),
    //        r"#\x0E0101".to_string()
    //    );
}

#[test]
fn test_print_single_char_elisp() {
    let writer =
        Writer::default().with_options(Options::default().with_style(LanguageStyle::EmacsLisp));

    assert_eq!(
        writer.write_to_string(&Value::from('a')).unwrap(),
        r"?a".to_string()
    );

    assert_eq!(
        writer.write_to_string(&Value::from('\n')).unwrap(),
        r"?\n".to_string()
    );
    //    assert_eq!(
    //        '§'.to_string_for(LanguageStyle::EmacsLisp),
    //        r"?§".to_string()
    //    );
    //    assert_eq!(
    //        ','.to_string_for(LanguageStyle::EmacsLisp),
    //        r"?\,".to_string()
    //    );
    //    assert_eq!(
    //        '\u{3001}'.to_string_for(LanguageStyle::EmacsLisp),
    //        r"?\u3001".to_string()
    //    );
    //    assert_eq!(
    //        '\u{E0101}'.to_string_for(LanguageStyle::EmacsLisp),
    //        r"?\U0E0101".to_string()
    //    );
}

#[test]
fn test_print_single_string_racket() {
    let writer =
        Writer::default().with_options(Options::default().with_style(LanguageStyle::Racket));

    assert_eq!(
        writer.write_to_string(&Value::from("hello")).unwrap(),
        "\"hello\"".to_string()
    );
    assert_eq!(
        writer.write_to_string(&Value::from("hel\tlo")).unwrap(),
        "\"hel\\tlo\"".to_string()
    );
    assert_eq!(
        writer.write_to_string(&Value::from("hel\u{00}lo")).unwrap(),
        "\"hel\\\\u0000lo\"".to_string()
    );
}

#[test]
fn test_print_single_string_treesitter() {}

#[test]
fn test_print_single_string_clisp() {}

#[test]
fn test_print_single_string_scheme() {}

#[test]
fn test_print_single_string_elisp() {}

#[test]
fn test_print_short_list_racket() {
    let writer =
        Writer::default().with_options(Options::default().with_style(LanguageStyle::Racket));

    let list = Value::from(vec![
        Value::from("hello"),
        Value::from("this"),
        Value::from("is"),
        Value::from("a"),
        Value::from("lisp"),
        Value::from("list"),
    ]);

    assert_eq!(
        writer.write_to_string(&list).unwrap(),
        "(\"hello\" \"this\" \"is\" \"a\" \"lisp\" \"list\")".to_string()
    );
}

#[test]
fn test_print_empty_list() {
    for style in [
        LanguageStyle::Racket,
        LanguageStyle::TreeSitter,
        LanguageStyle::CommonLisp,
        LanguageStyle::Scheme,
        LanguageStyle::EmacsLisp,
    ] {
        let writer = Writer::default().with_options(Options::default().with_style(style));

        assert_eq!(
            writer
                .write_to_string(&Value::from(Vec::default()))
                .unwrap(),
            r"()".to_string()
        );
    }
}
