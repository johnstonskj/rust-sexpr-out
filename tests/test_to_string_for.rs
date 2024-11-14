use pretty_assertions::assert_eq;
use sexpr_out::writer::{LanguageStyle, ToStringFor};

#[test]
fn test_boolean_racket() {
    assert_eq!(true.to_string_for(LanguageStyle::Racket), "#t".to_string());
    assert_eq!(false.to_string_for(LanguageStyle::Racket), "#f".to_string());
}

#[test]
fn test_boolean_treesitter() {
    assert_eq!(
        true.to_string_for(LanguageStyle::TreeSitter),
        "true".to_string()
    );
    assert_eq!(
        false.to_string_for(LanguageStyle::TreeSitter),
        "false".to_string()
    );
}

#[test]
fn test_boolean_clisp() {
    assert_eq!(
        true.to_string_for(LanguageStyle::CommonLisp),
        "t".to_string()
    );
    assert_eq!(
        false.to_string_for(LanguageStyle::CommonLisp),
        "nil".to_string()
    );
}

#[test]
fn test_boolean_scheme() {
    assert_eq!(true.to_string_for(LanguageStyle::Scheme), "#t".to_string());
    assert_eq!(false.to_string_for(LanguageStyle::Scheme), "#f".to_string());
}

#[test]
fn test_boolean_elisp() {
    assert_eq!(
        true.to_string_for(LanguageStyle::EmacsLisp),
        "t".to_string()
    );
    assert_eq!(
        false.to_string_for(LanguageStyle::EmacsLisp),
        "nil".to_string()
    );
}

#[test]
fn test_char_racket() {
    assert_eq!('a'.to_string_for(LanguageStyle::Racket), r"#\a".to_string());
    assert_eq!(
        '\n'.to_string_for(LanguageStyle::Racket),
        r"#\newline".to_string()
    );
    assert_eq!('§'.to_string_for(LanguageStyle::Racket), r"#\§".to_string());
    assert_eq!(
        '\u{3001}'.to_string_for(LanguageStyle::Racket),
        r"#\u3001".to_string()
    );
    assert_eq!(
        '\u{E0101}'.to_string_for(LanguageStyle::Racket),
        r"#\U0E0101".to_string()
    );
}

#[test]
fn test_char_treesitter() {
    assert_eq!(
        'a'.to_string_for(LanguageStyle::TreeSitter),
        r"'a'".to_string()
    );
    assert_eq!(
        '\n'.to_string_for(LanguageStyle::TreeSitter),
        r"'\n'".to_string()
    );
    assert_eq!(
        '§'.to_string_for(LanguageStyle::TreeSitter),
        r"'§'".to_string()
    );
    assert_eq!(
        '\u{30F0}'.to_string_for(LanguageStyle::TreeSitter),
        r"'ヰ'".to_string()
    );
    assert_eq!(
        '\u{E0101}'.to_string_for(LanguageStyle::TreeSitter),
        r"'\u{e0101}'".to_string()
    );
}

#[test]
fn test_char_clisp() {
    assert_eq!(
        'a'.to_string_for(LanguageStyle::CommonLisp),
        r"#\a".to_string()
    );
    assert_eq!(
        '\n'.to_string_for(LanguageStyle::CommonLisp),
        r"#\Newline".to_string()
    );
    assert_eq!(
        '§'.to_string_for(LanguageStyle::CommonLisp),
        r"#\§".to_string()
    );
    assert_eq!(
        '\u{3001}'.to_string_for(LanguageStyle::CommonLisp),
        r"#\U3001".to_string()
    );
    assert_eq!(
        '\u{E0101}'.to_string_for(LanguageStyle::CommonLisp),
        r"#\U0E0101".to_string()
    );
}

#[test]
fn test_char_scheme() {
    assert_eq!('a'.to_string_for(LanguageStyle::Scheme), r"#\a".to_string());
    assert_eq!(
        '\n'.to_string_for(LanguageStyle::Scheme),
        r"#\newline".to_string()
    );
    assert_eq!('§'.to_string_for(LanguageStyle::Scheme), r"#\§".to_string());
    assert_eq!(
        '\u{3001}'.to_string_for(LanguageStyle::Scheme),
        r"#\x3001".to_string()
    );
    assert_eq!(
        '\u{E0101}'.to_string_for(LanguageStyle::Scheme),
        r"#\x0E0101".to_string()
    );
}

#[test]
fn test_char_elisp() {
    assert_eq!(
        'a'.to_string_for(LanguageStyle::EmacsLisp),
        r"?a".to_string()
    );
    assert_eq!(
        '\n'.to_string_for(LanguageStyle::EmacsLisp),
        r"?\n".to_string()
    );
    assert_eq!(
        '§'.to_string_for(LanguageStyle::EmacsLisp),
        r"?§".to_string()
    );
    assert_eq!(
        ','.to_string_for(LanguageStyle::EmacsLisp),
        r"?\,".to_string()
    );
    assert_eq!(
        '\u{3001}'.to_string_for(LanguageStyle::EmacsLisp),
        r"?\u3001".to_string()
    );
    assert_eq!(
        '\u{E0101}'.to_string_for(LanguageStyle::EmacsLisp),
        r"?\U0E0101".to_string()
    );
}

#[test]
fn test_string_racket() {
    assert_eq!(
        String::from("hello").to_string_for(LanguageStyle::Racket),
        "\"hello\"".to_string()
    );
    assert_eq!(
        String::from("hel\tlo").to_string_for(LanguageStyle::Racket),
        "he#\\tablo".to_string()
    );
}
