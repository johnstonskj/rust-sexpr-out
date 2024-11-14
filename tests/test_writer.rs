use objio::{HasOptions, ObjectWriter};
use pretty_assertions::assert_eq;
use sexpr_out::value::Value;
use sexpr_out::writer::{LanguageStyle, Options, Writer};

#[test]
fn test_write_single_boolean_racket() {
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
fn test_write_single_boolean_treesitter() {
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
fn test_write_single_boolean_clisp() {
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
fn test_write_single_boolean_scheme() {
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
fn test_write_single_boolean_elisp() {
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
fn test_write_single_char_racket() {
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
}

#[test]
fn test_write_single_char_treesitter() {
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
}

#[test]
fn test_write_single_char_clisp() {
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
}

#[test]
fn test_write_single_char_scheme() {
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
}

#[test]
fn test_write_single_char_elisp() {
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
}

#[test]
fn test_write_single_string_racket() {}

#[test]
fn test_write_single_string_treesitter() {}

#[test]
fn test_write_single_string_clisp() {}

#[test]
fn test_write_single_string_scheme() {}

#[test]
fn test_write_single_string_elisp() {}

#[test]
fn test_write_empty_list() {
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
