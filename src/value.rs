/*!
One-line description.

More detailed description, with

# Example

End of file during parsingSymbol’s value as variable is void: rustEnd of file during parsing

 */

use crate::writer::{LanguageStyle, ToStringFor};
use itertools::{Itertools, Position};

// ------------------------------------------------------------------------------------------------
// Public Macros
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Copy, Debug)]
pub enum Number {
    Integer(i64),
    Flonum(f64),
}

#[derive(Clone, Debug)]
pub struct Symbol(String);

#[derive(Clone, Debug)]
pub struct Keyword(Symbol);

#[derive(Clone, Debug)]
pub enum Value {
    Bool(bool),
    Number(Number),
    Character(char),
    String(String),
    Symbol(Symbol),
    Keyword(Keyword),
    List(Vec<Value>),
}

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Macros
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Implementations ❱ Number
// ------------------------------------------------------------------------------------------------

impl From<i64> for Number {
    fn from(value: i64) -> Self {
        Self::Integer(value)
    }
}

impl From<&i64> for Number {
    fn from(value: &i64) -> Self {
        Self::Integer(*value)
    }
}

impl From<i32> for Number {
    fn from(value: i32) -> Self {
        Self::Integer(value as i64)
    }
}

impl From<&i32> for Number {
    fn from(value: &i32) -> Self {
        Self::Integer(*value as i64)
    }
}

impl From<i16> for Number {
    fn from(value: i16) -> Self {
        Self::Integer(value as i64)
    }
}

impl From<&i16> for Number {
    fn from(value: &i16) -> Self {
        Self::Integer(*value as i64)
    }
}

impl From<i8> for Number {
    fn from(value: i8) -> Self {
        Self::Integer(value as i64)
    }
}

impl From<&i8> for Number {
    fn from(value: &i8) -> Self {
        Self::Integer(*value as i64)
    }
}

impl From<f64> for Number {
    fn from(value: f64) -> Self {
        Self::Flonum(value)
    }
}

impl From<&f64> for Number {
    fn from(value: &f64) -> Self {
        Self::Flonum(*value)
    }
}

impl From<f32> for Number {
    fn from(value: f32) -> Self {
        Self::Flonum(value as f64)
    }
}

impl From<&f32> for Number {
    fn from(value: &f32) -> Self {
        Self::Flonum(*value as f64)
    }
}

impl ToStringFor for Number {
    fn to_string_for(&self, _: LanguageStyle) -> String {
        match self {
            Self::Integer(v) => v.to_string(),
            Self::Flonum(v) => v.to_string(),
        }
    }
}

impl Number {
    pub fn is_integer(&self) -> bool {
        matches!(self, Self::Integer(_))
    }

    pub fn as_integer(&self) -> Option<i64> {
        match self {
            Number::Integer(v) => Some(*v),
            _ => None,
        }
    }

    pub fn is_flonum(&self) -> bool {
        matches!(self, Self::Flonum(_))
    }

    pub fn as_flonum(&self) -> Option<f64> {
        match self {
            Number::Flonum(v) => Some(*v),
            _ => None,
        }
    }
}

// ------------------------------------------------------------------------------------------------
// Implementations ❱ Symbol
// ------------------------------------------------------------------------------------------------

impl AsRef<str> for Symbol {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl From<Symbol> for String {
    fn from(value: Symbol) -> Self {
        value.0
    }
}

impl ToStringFor for Symbol {
    fn to_string_for(&self, style: LanguageStyle) -> String {
        fn inner(s: &str, style: LanguageStyle) -> String {
            let new_s = s
                .chars()
                .tuple_windows()
                .map(|(c0, c1)| {
                    if c1 == '|' && c0 != '\\' {
                        format!("{c0}\\|")
                    } else if is_char_nonprintable(c0) {
                        format!("{}{c1}", c0.to_string_for(style))
                    } else {
                        format!("{c0}{c1}")
                    }
                })
                .collect::<String>();
            if new_s != s {
                format!("|{new_s}|")
            } else {
                new_s
            }
        }
        if style == LanguageStyle::TreeSitter {
            // a shame...
            self.0.clone()
        } else {
            let s = &self.0;
            if style == LanguageStyle::Racket && s.starts_with("#%") {
                inner(&s[2..], style)
            } else if s.starts_with("|") && s.ends_with("|") {
                let len = s.len() - 2;
                inner(&s[1..len], style)
            } else {
                inner(s, style)
            }
        }
    }
}

///
/// From https://docs.racket-lang.org/guide/symbols.html
///
/// Any string (i.e., any character sequence) can be supplied to string->symbol to obtain the
/// corresponding symbol. For reader input, any character can appear directly in an identifier,
/// except for whitespace and the following special characters:
///
///   ( ) [ ] { } " , ' ` ; # | \
///
/// Actually, # is disallowed only at the beginning of a symbol, and then only if not followed by %;
/// otherwise, # is allowed, too. Also, . by itself is not a symbol.
///
/// Whitespace or special characters can be included in an identifier by quoting them with | or \.
/// These quoting mechanisms are used in the printed form of identifiers that contain special
/// characters or that might otherwise look like numbers.
///
/// A #% also starts a symbol. (From https://docs.racket-lang.org/reference/reader.html#%28part._parse-symbol%29)
///
impl Symbol {
    pub fn new<S>(s: S) -> Self
    where
        S: Into<String>,
    {
        Self(s.into())
    }
}

// ------------------------------------------------------------------------------------------------
// Implementations ❱ Keyword
// ------------------------------------------------------------------------------------------------

impl From<Symbol> for Keyword {
    fn from(value: Symbol) -> Self {
        Self(value)
    }
}

impl From<Keyword> for Symbol {
    fn from(value: Keyword) -> Self {
        value.0
    }
}

impl From<Keyword> for String {
    fn from(value: Keyword) -> Self {
        value.0.into()
    }
}

impl ToStringFor for Keyword {
    fn to_string_for(&self, style: LanguageStyle) -> String {
        let inner = self.0.to_string_for(style);
        match style {
            LanguageStyle::Racket => format!("#:{inner}"),
            LanguageStyle::TreeSitter => format!("{inner}:"),
            LanguageStyle::CommonLisp => format!(":{inner}"),
            LanguageStyle::Scheme => format!(":{inner}"),
            LanguageStyle::EmacsLisp => format!(":{inner}"),
        }
    }
}

impl Keyword {
    pub fn new<S>(s: S) -> Self
    where
        S: Into<String>,
    {
        Self(Symbol::new(s.into()))
    }
}

// ------------------------------------------------------------------------------------------------
// Implementations ❱ Value::Bool
// ------------------------------------------------------------------------------------------------

impl From<bool> for Value {
    fn from(value: bool) -> Self {
        Self::Bool(value)
    }
}

impl From<&bool> for Value {
    fn from(value: &bool) -> Self {
        Self::Bool(*value)
    }
}

impl ToStringFor for bool {
    fn to_string_for(&self, style: LanguageStyle) -> String {
        match (style, self) {
            (LanguageStyle::Racket, true) => "#t",
            (LanguageStyle::Racket, false) => "#f",
            (LanguageStyle::TreeSitter, true) => "true",
            (LanguageStyle::TreeSitter, false) => "false",
            (LanguageStyle::CommonLisp, true) => "t",
            (LanguageStyle::CommonLisp, false) => "nil",
            (LanguageStyle::Scheme, true) => "#t",
            (LanguageStyle::Scheme, false) => "#f",
            (LanguageStyle::EmacsLisp, true) => "t",
            (LanguageStyle::EmacsLisp, false) => "nil",
        }
        .to_string()
    }
}

// ------------------------------------------------------------------------------------------------
// Implementations ❱ Value::Number
// ------------------------------------------------------------------------------------------------

impl<N: Into<Number>> From<N> for Value {
    fn from(value: N) -> Self {
        Self::Number(value.into())
    }
}

// ------------------------------------------------------------------------------------------------
// Implementations ❱ Value::Character
// ------------------------------------------------------------------------------------------------

impl From<char> for Value {
    fn from(value: char) -> Self {
        Self::Character(value)
    }
}

impl From<&char> for Value {
    fn from(value: &char) -> Self {
        Self::Character(*value)
    }
}

fn is_char_nonprintable(c: char) -> bool {
    matches!(c,
        '\u{0000}'..'\u{0008}'
        | '\u{000B}'..'\u{001F}'
        | '\u{007F}'..'\u{009F}'
        | '\u{2000}'..'\u{200F}'
        | '\u{2028}'..'\u{202F}'
        | '\u{205F}'..'\u{206F}'
        | '\u{3000}'..'\u{FEFF}'
        | '\u{E0100}'..'\u{E01EF}')
}

impl ToStringFor for char {
    fn to_string_for(&self, style: LanguageStyle) -> String {
        fn escape(
            c: char,
            std_prefix: &str,
            esc_prefix: &str,
            long_esc_prefix: Option<&str>,
        ) -> String {
            println!("escape char {:06X}?", c as u32);
            match c {
                '\u{0000}'..'\u{0008}'
                | '\u{000B}'..'\u{001F}'
                | '\u{007F}'..'\u{009F}'
                | '\u{2000}'..'\u{200F}'
                | '\u{2028}'..'\u{202F}'
                | '\u{205F}'..'\u{206F}'
                | '\u{3000}'..'\u{FEFF}' => format!("{esc_prefix}{:04X}", c as u32),
                '\u{E0100}'..'\u{E01EF}' => {
                    format!("{}{:06X}", long_esc_prefix.unwrap_or(esc_prefix), c as u32)
                }
                _ => format!("{std_prefix}{c}"),
            }
        }
        match (*self, style) {
            ('\u{00}', LanguageStyle::Racket) => r"#\null".to_string(),
            ('\u{08}', LanguageStyle::Racket) => r"#\backspace".to_string(),
            ('\u{09}', LanguageStyle::Racket) => r"#\tab".to_string(),
            ('\u{0A}', LanguageStyle::Racket) => r"#\newline".to_string(),
            ('\u{0B}', LanguageStyle::Racket) => r"#\vtab".to_string(),
            ('\u{0C}', LanguageStyle::Racket) => r"#\page".to_string(),
            ('\u{0D}', LanguageStyle::Racket) => r"#\return".to_string(),
            ('\u{20}', LanguageStyle::Racket) => r"#\space".to_string(),
            ('\u{7F}', LanguageStyle::Racket) => r"#\rubout".to_string(),
            (c, LanguageStyle::Racket) => escape(c, r"#\", r"#\u", Some(r"#\U")),
            (c, LanguageStyle::TreeSitter) => format!("{c:?}"),
            ('\u{08}', LanguageStyle::CommonLisp) => r"#\Backspace".to_string(),
            ('\u{09}', LanguageStyle::CommonLisp) => r"#\Tab".to_string(),
            ('\u{0A}', LanguageStyle::CommonLisp) => r"#\Newline".to_string(),
            ('\u{0C}', LanguageStyle::CommonLisp) => r"#\Page".to_string(),
            ('\u{0D}', LanguageStyle::CommonLisp) => r"#\Return".to_string(),
            ('\u{20}', LanguageStyle::CommonLisp) => r"#\Space".to_string(),
            ('\u{7F}', LanguageStyle::CommonLisp) => r"#\Rubout".to_string(),
            (c, LanguageStyle::CommonLisp) => escape(c, r"#\", r"#\U", None),
            ('\u{00}', LanguageStyle::Scheme) => r"#\null".to_string(),
            ('\u{07}', LanguageStyle::Scheme) => r"#\alarm".to_string(),
            ('\u{08}', LanguageStyle::Scheme) => r"#\backspace".to_string(),
            ('\u{09}', LanguageStyle::Scheme) => r"#\tab".to_string(),
            ('\u{0A}', LanguageStyle::Scheme) => r"#\newline".to_string(),
            ('\u{0D}', LanguageStyle::Scheme) => r"#\return".to_string(),
            ('\u{1B}', LanguageStyle::Scheme) => r"#\escape".to_string(),
            ('\u{20}', LanguageStyle::Scheme) => r"#\space".to_string(),
            ('\u{7F}', LanguageStyle::Scheme) => r"#\delete".to_string(),
            (c, LanguageStyle::Scheme) => escape(c, r"#\", r"#\x", None),
            ('\u{00}', LanguageStyle::EmacsLisp) => r"?u00".to_string(),
            ('\u{07}', LanguageStyle::EmacsLisp) => r"?\a".to_string(),
            ('\u{08}', LanguageStyle::EmacsLisp) => r"?\b".to_string(),
            ('\u{09}', LanguageStyle::EmacsLisp) => r"?\t".to_string(),
            ('\u{0A}', LanguageStyle::EmacsLisp) => r"?\n".to_string(),
            ('\u{0B}', LanguageStyle::EmacsLisp) => r"?\v".to_string(),
            ('\u{0C}', LanguageStyle::EmacsLisp) => r"?\f".to_string(),
            ('\u{0D}', LanguageStyle::EmacsLisp) => r"?\r".to_string(),
            ('\u{1B}', LanguageStyle::EmacsLisp) => r"?\e".to_string(),
            ('\u{20}', LanguageStyle::EmacsLisp) => r"?\s".to_string(),
            ('\u{7F}', LanguageStyle::EmacsLisp) => r"?\d".to_string(),
            (
                c @ ('(' | ')' | '[' | ']' | '\\' | ';' | '|' | '\'' | '`' | '#' | '.' | ',' | '‘'),
                LanguageStyle::EmacsLisp,
            ) => format!("?\\{c}"),
            (c, LanguageStyle::EmacsLisp) => escape(c, r"?", r"?\u", Some(r"?\U")),
        }
    }
}

// ------------------------------------------------------------------------------------------------
// Implementations ❱ Value::String
// ------------------------------------------------------------------------------------------------

impl From<String> for Value {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}

impl From<&String> for Value {
    fn from(value: &String) -> Self {
        Self::String(value.clone())
    }
}

impl From<&str> for Value {
    fn from(value: &str) -> Self {
        Self::String(value.to_string())
    }
}

impl ToStringFor for String {
    fn to_string_for(&self, style: LanguageStyle) -> String {
        format!(
            "{:?}",
            self.chars()
                .tuple_windows()
                .with_position()
                .map(|(p, (c0, c1))| {
                    if c1 == '|' && c0 != '\\' {
                        format!("{c0}\\{}", if p == Position::Last { "|" } else { "" })
                    } else {
                        match (p, is_char_nonprintable(c0), is_char_nonprintable(c1)) {
                            (Position::Last, true, true) => {
                                format!("{}{}", c0.to_string_for(style), c1.to_string_for(style))
                            }
                            (Position::Last, true, false) => {
                                format!("{}{}", c0.to_string_for(style), c1)
                            }
                            (Position::Last, false, true) => {
                                format!("{}{}", c0, c1.to_string_for(style))
                            }
                            (Position::Last, false, false) => format!("{}{}", c0, c1),
                            (_, true, _) => c0.to_string_for(style),
                            (_, false, _) => c0.to_string(),
                        }
                    }
                })
                .collect::<String>()
        )
    }
}

// ------------------------------------------------------------------------------------------------
// Implementations ❱ Value::Symbol
// ------------------------------------------------------------------------------------------------

impl From<Symbol> for Value {
    fn from(value: Symbol) -> Self {
        Self::Symbol(value)
    }
}

impl From<&Symbol> for Value {
    fn from(value: &Symbol) -> Self {
        Self::Symbol(value.clone())
    }
}

// ------------------------------------------------------------------------------------------------
// Implementations ❱ Value::Keyword
// ------------------------------------------------------------------------------------------------

impl From<Keyword> for Value {
    fn from(value: Keyword) -> Self {
        Self::Keyword(value)
    }
}

impl From<&Keyword> for Value {
    fn from(value: &Keyword) -> Self {
        Self::Keyword(value.clone())
    }
}

// ------------------------------------------------------------------------------------------------
// Implementations ❱ Value::List
// ------------------------------------------------------------------------------------------------

impl From<Vec<Value>> for Value {
    fn from(value: Vec<Value>) -> Self {
        Self::List(value)
    }
}

impl From<&Vec<Value>> for Value {
    fn from(value: &Vec<Value>) -> Self {
        Self::List(value.clone())
    }
}

impl From<&[Value]> for Value {
    fn from(value: &[Value]) -> Self {
        Self::List(value.to_vec())
    }
}

impl FromIterator<Value> for Value {
    fn from_iter<T: IntoIterator<Item = Value>>(iter: T) -> Self {
        Self::List(Vec::from_iter(iter))
    }
}

impl ToStringFor for Vec<Value> {
    fn to_string_for(&self, style: LanguageStyle) -> String {
        format!(
            "({})",
            self.iter()
                .map(|v| v.to_string_for(style))
                .collect::<Vec<String>>()
                .join(" ")
        )
    }
}

// ------------------------------------------------------------------------------------------------
// Implementations ❱ Value
// ------------------------------------------------------------------------------------------------

impl ToStringFor for Value {
    fn to_string_for(&self, style: LanguageStyle) -> String {
        match self {
            Value::Bool(v) => v.to_string_for(style),
            Value::Number(v) => v.to_string_for(style),
            Value::Character(v) => v.to_string_for(style),
            Value::String(v) => v.to_string_for(style),
            Value::Symbol(v) => v.to_string_for(style),
            Value::Keyword(v) => v.to_string_for(style),
            Value::List(v) => v.to_string_for(style),
        }
    }
}

impl Value {}
