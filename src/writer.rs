/*!
This module provides the ability to write `Value`s in different language styles with an option to
*pretty print* nested values.

# Example

```rust
use objio::{HasOptions, ObjectWriter};
use sexpr_out::{value::Value, writer::{LanguageStyle, Options, Writer}};

let writer = Writer::default().with_options(
    Options::default().with_style(LanguageStyle::Racket)
);

assert_eq!(
    writer.write_to_string(&Value::from(true)).unwrap(),
    "#t".to_string()
);
```

 */

/*
Copyright 2024 Simon Johnston <johnstonskj@gmail.com>

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
*/

use crate::{
    value::{Keyword, Number, Symbol},
    Error, Value,
};
use itertools::{Itertools, Position};
use objio::{HasOptions, ObjectWriter};
use std::io::Write;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum LanguageStyle {
    #[default]
    Racket,
    TreeSitter,
    CommonLisp,
    Scheme, // as of R7RS
    EmacsLisp,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum QuoteStyle {
    #[default]
    None,
    All(bool),
    AsNeeded(bool),
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct Options {
    line_width: usize,
    pair_kw_args: bool,
    wrap_in_define: bool,
    style: LanguageStyle,
    quote: QuoteStyle,
}

#[derive(Debug, Default)]
pub struct Writer {
    options: Options,
    pretty_print: bool,
}

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

pub(crate) trait Printable {
    fn stringify(&self, style: LanguageStyle) -> String;
    fn printed_length(&self, style: LanguageStyle) -> usize {
        self.stringify(style).len()
    }

    fn print<W>(&self, w: &mut W, style: LanguageStyle) -> Result<(), Error>
    where
        W: Write,
    {
        Ok(w.write_all(self.stringify(style).as_bytes())?)
    }

    fn pretty_print<W>(
        &self,
        w: &mut W,
        _current_indentation: usize,
        _line_width: usize,
        style: LanguageStyle,
    ) -> Result<(), Error>
    where
        W: Write,
    {
        self.print(w, style)
    }
}

// ------------------------------------------------------------------------------------------------
// Private Values
// ------------------------------------------------------------------------------------------------

/// List Delimiters
const CHAR_LIST_OPEN: char = '(';
const CHAR_LIST_CLOSE: char = ')';
const LIST_OPEN: &[u8] = b"(";
const LIST_CLOSE: &[u8] = b")";
const EMPTY_LIST: &[u8] = b"()";
const CHAR_SQLIST_OPEN: char = '[';
const CHAR_SQLIST_CLOSE: char = ']';

/// Separators Separator
const STR_EMPTY: &str = "";
const SPACE: &[u8] = b" ";
const NEWLINE: &[u8] = b"\n";
const CHAR_NEWLINE: char = '\n';

/// Special Reader Syntax
const CHAR_NUMBER_SIGN: char = '#';
const CHAR_COLON: char = ':';
const CHAR_SEMICOLON: char = ';';
const CHAR_PERIOD: char = '.';

/// Escaping
const CHAR_BACKSLASH: char = '\\';
const CHAR_VERTICAL_BAR: char = '|';
const STR_VERTICAL_BAR: &str = "|";

/// Quoting
const CHAR_QUOTE: char = '\'';
const CHAR_QUASI_QUOTE: char = '`';
const CHAR_UNQUOTE: char = ',';
const CHAR_OTHER_QUOTE: char = '‘';

const KEYWORD_FALSE: &str = "f";
const KEYWORD_FALSE_LONG: &str = "false";
const KEYWORD_TRUE: &str = "t";
const KEYWORD_TRUE_LONG: &str = "true";
const KEYWORD_NIL: &str = "nil";

const RACKET_SYMBOL_PREFIX: &str = "#%";
const CHAR_PREFIX: &str = "#\\";
const CHAR_PREFIX_UNICODE: &str = "#\\u";
const CHAR_PREFIX_UNICODE_LONG: &str = "#\\U";
const SCHEME_CHAR_PREFIX_UNICODE: &str = "#\\x";
const ELISP_CHAR_PREFIX: &str = "?";
const ELISP_CHAR_PREFIX_ESC: &str = "?\\";
const ELISP_CHAR_PREFIX_UNICODE: &str = "?\\u";
const ELISP_CHAR_PREFIX_UNICODE_LONG: &str = "?\\U";

// ------------------------------------------------------------------------------------------------
// Implementations ❱ QuoteStyle
// ------------------------------------------------------------------------------------------------

impl QuoteStyle {
    pub fn is_long_form(&self) -> bool {
        match self {
            Self::None => false,
            Self::All(v) => *v,
            Self::AsNeeded(v) => *v,
        }
    }
}

// ------------------------------------------------------------------------------------------------
// Implementations ❱ Options
// ------------------------------------------------------------------------------------------------

impl Options {
    pub fn with_line_width(self, line_width: usize) -> Self {
        let mut self_mut = self;
        self_mut.set_line_width(line_width);
        self_mut
    }

    pub fn line_width(&self) -> &usize {
        &self.line_width
    }

    pub fn set_line_width(&mut self, line_width: usize) {
        assert!(line_width >= 20);
        self.line_width = line_width;
    }

    // --------------------------------------------------------------------------------------------

    pub fn with_style(self, style: LanguageStyle) -> Self {
        let mut self_mut = self;
        self_mut.set_style(style);
        self_mut
    }

    pub fn style(&self) -> &LanguageStyle {
        &self.style
    }

    pub fn set_style(&mut self, style: LanguageStyle) {
        self.style = style;
    }

    // --------------------------------------------------------------------------------------------

    pub fn with_wrap_in_define(self, wrap_in_define: bool) -> Self {
        let mut self_mut = self;
        self_mut.set_wrap_in_define(wrap_in_define);
        self_mut
    }

    pub fn wrap_in_define(&self) -> &bool {
        &self.wrap_in_define
    }

    pub fn set_wrap_in_define(&mut self, wrap_in_define: bool) {
        self.wrap_in_define = wrap_in_define;
    }

    // --------------------------------------------------------------------------------------------

    pub fn with_quote(self, quote: QuoteStyle) -> Self {
        let mut self_mut = self;
        self_mut.set_quote(quote);
        self_mut
    }

    pub fn quote(&self) -> &QuoteStyle {
        &self.quote
    }

    pub fn set_quote(&mut self, quote: QuoteStyle) {
        self.quote = quote;
    }
}

// ------------------------------------------------------------------------------------------------
// Implementations ❱ Writer
// ------------------------------------------------------------------------------------------------

impl HasOptions<Options> for Writer {
    fn set_options(&mut self, options: Options) {
        self.options = options;
    }

    fn options(&self) -> &Options {
        &self.options
    }
}

impl ObjectWriter<Value> for Writer {
    type Error = Error;

    fn write<W>(&self, w: &mut W, object: &Value) -> Result<(), Self::Error>
    where
        W: Write,
    {
        if self.pretty_print {
            object.pretty_print(w, 0, self.options.line_width, self.options.style)?;
            w.write_all(NEWLINE)?;
            Ok(())
        } else {
            object.print(w, self.options.style)
        }
    }
}

impl Writer {
    pub fn pretty_printed(self, pretty_print: bool) -> Self {
        let mut self_mut = self;
        self_mut.set_pretty_print(pretty_print);
        self_mut
    }

    pub fn pretty_print(&self) -> bool {
        self.pretty_print
    }

    pub fn set_pretty_print(&mut self, pretty_print: bool) {
        self.pretty_print = pretty_print;
    }
}

// ------------------------------------------------------------------------------------------------
// Implementations ❱ Printables
// ------------------------------------------------------------------------------------------------

macro_rules! display_to_printable {
    ($type:ty) => {
        impl Printable for $type {
            fn stringify(&self, _: LanguageStyle) -> String {
                self.to_string()
            }
        }
    };
}

// ------------------------------------------------------------------------------------------------

display_to_printable!(i64);

// ------------------------------------------------------------------------------------------------

display_to_printable!(f64);

// ------------------------------------------------------------------------------------------------

impl Printable for Number {
    fn stringify(&self, style: LanguageStyle) -> String {
        match self {
            Number::Integer(v) => v.stringify(style),
            Number::Flonum(v) => v.stringify(style),
        }
    }
}

// ------------------------------------------------------------------------------------------------

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
impl Printable for Symbol {
    fn stringify(&self, style: LanguageStyle) -> String {
        fn inner(s: &str, style: LanguageStyle) -> String {
            let mut add_multi_escape = false;
            let new_s = s
                .chars()
                .tuple_windows()
                .map(|(c0, c1)| {
                    if c1 == CHAR_VERTICAL_BAR && c0 != CHAR_BACKSLASH {
                        add_multi_escape = true;
                        format!("{c0}{CHAR_BACKSLASH}{CHAR_VERTICAL_BAR}")
                    } else if is_char_nonprintable(c0) {
                        add_multi_escape = true;
                        format!("{}{c1}", string_escape_char(c0, style))
                    } else {
                        format!("{c0}{c1}")
                    }
                })
                .collect::<String>();
            if add_multi_escape || new_s != s {
                format!("{CHAR_VERTICAL_BAR}{new_s}{CHAR_VERTICAL_BAR}")
            } else {
                new_s
            }
        }
        if style == LanguageStyle::TreeSitter {
            // a shame...
            self.inner()
        } else {
            let s = self.as_ref();
            if style == LanguageStyle::Racket && s.starts_with(RACKET_SYMBOL_PREFIX) {
                inner(&s[2..], style)
            } else if s.starts_with(CHAR_VERTICAL_BAR) && s.ends_with(CHAR_VERTICAL_BAR) {
                let len = s.len() - 2;
                inner(&s[1..len], style)
            } else {
                inner(s, style)
            }
        }
    }

    fn printed_length(&self, style: LanguageStyle) -> usize {
        self.stringify(style).len()
    }

    fn print<W>(&self, w: &mut W, style: LanguageStyle) -> Result<(), Error>
    where
        W: Write,
    {
        Ok(w.write_all(self.stringify(style).as_bytes())?)
    }

    fn pretty_print<W>(
        &self,
        w: &mut W,
        _current_indentation: usize,
        _line_width: usize,
        style: LanguageStyle,
    ) -> Result<(), Error>
    where
        W: Write,
    {
        self.print(w, style)
    }
}

// ------------------------------------------------------------------------------------------------

impl Printable for Keyword {
    fn stringify(&self, style: LanguageStyle) -> String {
        let inner = self.inner().stringify(style);
        match style {
            LanguageStyle::Racket => format!("{CHAR_NUMBER_SIGN}{CHAR_COLON}{inner}"),
            LanguageStyle::TreeSitter => format!("{inner}{CHAR_COLON}"),
            LanguageStyle::CommonLisp => format!("{CHAR_COLON}{inner}"),
            LanguageStyle::Scheme => format!("{CHAR_COLON}{inner}"),
            LanguageStyle::EmacsLisp => format!("{CHAR_COLON}{inner}"),
        }
    }
}

// ------------------------------------------------------------------------------------------------

impl Printable for bool {
    fn stringify(&self, style: LanguageStyle) -> String {
        match (style, *self) {
            (LanguageStyle::Racket, true) => format!("{CHAR_NUMBER_SIGN}{KEYWORD_TRUE}"),
            (LanguageStyle::Racket, false) => format!("{CHAR_NUMBER_SIGN}{KEYWORD_FALSE}"),
            (LanguageStyle::TreeSitter, true) => KEYWORD_TRUE_LONG.to_string(),
            (LanguageStyle::TreeSitter, false) => KEYWORD_FALSE_LONG.to_string(),
            (LanguageStyle::CommonLisp, true) => KEYWORD_TRUE.to_string(),
            (LanguageStyle::CommonLisp, false) => KEYWORD_NIL.to_string(),
            (LanguageStyle::Scheme, true) => format!("{CHAR_NUMBER_SIGN}{KEYWORD_TRUE}"),
            (LanguageStyle::Scheme, false) => format!("{CHAR_NUMBER_SIGN}{KEYWORD_FALSE}"),
            (LanguageStyle::EmacsLisp, true) => KEYWORD_TRUE.to_string(),
            (LanguageStyle::EmacsLisp, false) => KEYWORD_NIL.to_string(),
        }
    }
}

// ------------------------------------------------------------------------------------------------

impl Printable for char {
    fn stringify(&self, style: LanguageStyle) -> String {
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
            ('\u{00}', LanguageStyle::Racket) => format!("{CHAR_PREFIX}null"),
            ('\u{08}', LanguageStyle::Racket) => {
                format!("{CHAR_PREFIX}backspace")
            }
            ('\u{09}', LanguageStyle::Racket) => format!("{CHAR_PREFIX}tab"),
            ('\u{0A}', LanguageStyle::Racket) => {
                format!("{CHAR_PREFIX}newline")
            }
            ('\u{0B}', LanguageStyle::Racket) => format!("{CHAR_PREFIX}vtab"),
            ('\u{0C}', LanguageStyle::Racket) => format!("{CHAR_PREFIX}page"),
            ('\u{0D}', LanguageStyle::Racket) => {
                format!("{CHAR_PREFIX}return")
            }
            ('\u{20}', LanguageStyle::Racket) => format!("{CHAR_PREFIX}space"),
            ('\u{7F}', LanguageStyle::Racket) => {
                format!("{CHAR_PREFIX}rubout")
            }
            (c, LanguageStyle::Racket) => escape(
                c,
                CHAR_PREFIX,
                CHAR_PREFIX_UNICODE,
                Some(CHAR_PREFIX_UNICODE_LONG),
            ),
            (c, LanguageStyle::TreeSitter) => format!("{c:?}"),
            ('\u{08}', LanguageStyle::CommonLisp) => format!("{CHAR_PREFIX}Backspace"),
            ('\u{09}', LanguageStyle::CommonLisp) => format!("{CHAR_PREFIX}Tab"),
            ('\u{0A}', LanguageStyle::CommonLisp) => format!("{CHAR_PREFIX}Newline"),
            ('\u{0C}', LanguageStyle::CommonLisp) => format!("{CHAR_PREFIX}Page"),
            ('\u{0D}', LanguageStyle::CommonLisp) => format!("{CHAR_PREFIX}Return"),
            ('\u{20}', LanguageStyle::CommonLisp) => format!("{CHAR_PREFIX}Space"),
            ('\u{7F}', LanguageStyle::CommonLisp) => format!("{CHAR_PREFIX}Rubout"),
            (c, LanguageStyle::CommonLisp) => {
                escape(c, CHAR_PREFIX, CHAR_PREFIX_UNICODE_LONG, None)
            }
            ('\u{00}', LanguageStyle::Scheme) => format!("{CHAR_PREFIX}null"),
            ('\u{07}', LanguageStyle::Scheme) => format!("{CHAR_PREFIX}alarm"),
            ('\u{08}', LanguageStyle::Scheme) => format!("{CHAR_PREFIX}backspace"),
            ('\u{09}', LanguageStyle::Scheme) => format!("{CHAR_PREFIX}tab"),
            ('\u{0A}', LanguageStyle::Scheme) => format!("{CHAR_PREFIX}newline"),
            ('\u{0D}', LanguageStyle::Scheme) => format!("{CHAR_PREFIX}return"),
            ('\u{1B}', LanguageStyle::Scheme) => format!("{CHAR_PREFIX}escape"),
            ('\u{20}', LanguageStyle::Scheme) => format!("{CHAR_PREFIX}space"),
            ('\u{7F}', LanguageStyle::Scheme) => format!("{CHAR_PREFIX}delete"),
            (c, LanguageStyle::Scheme) => escape(c, CHAR_PREFIX, SCHEME_CHAR_PREFIX_UNICODE, None),
            ('\u{00}', LanguageStyle::EmacsLisp) => format!("{ELISP_CHAR_PREFIX_UNICODE}00"),
            ('\u{07}', LanguageStyle::EmacsLisp) => format!("{ELISP_CHAR_PREFIX_ESC}a"),
            ('\u{08}', LanguageStyle::EmacsLisp) => format!("{ELISP_CHAR_PREFIX_ESC}b"),
            ('\u{09}', LanguageStyle::EmacsLisp) => format!("{ELISP_CHAR_PREFIX_ESC}t"),
            ('\u{0A}', LanguageStyle::EmacsLisp) => format!("{ELISP_CHAR_PREFIX_ESC}n"),
            ('\u{0B}', LanguageStyle::EmacsLisp) => format!("{ELISP_CHAR_PREFIX_ESC}v"),
            ('\u{0C}', LanguageStyle::EmacsLisp) => format!("{ELISP_CHAR_PREFIX_ESC}f"),
            ('\u{0D}', LanguageStyle::EmacsLisp) => format!("{ELISP_CHAR_PREFIX_ESC}r"),
            ('\u{1B}', LanguageStyle::EmacsLisp) => format!("{ELISP_CHAR_PREFIX_ESC}e"),
            ('\u{20}', LanguageStyle::EmacsLisp) => format!("{ELISP_CHAR_PREFIX_ESC}s"),
            ('\u{7F}', LanguageStyle::EmacsLisp) => format!("{ELISP_CHAR_PREFIX_ESC}d"),
            (
                c @ (CHAR_LIST_OPEN | CHAR_LIST_CLOSE | CHAR_SQLIST_OPEN | CHAR_SQLIST_CLOSE
                | CHAR_BACKSLASH | CHAR_SEMICOLON | CHAR_VERTICAL_BAR | CHAR_QUOTE
                | CHAR_QUASI_QUOTE | CHAR_NUMBER_SIGN | CHAR_PERIOD | CHAR_UNQUOTE
                | CHAR_OTHER_QUOTE),
                LanguageStyle::EmacsLisp,
            ) => format!("{ELISP_CHAR_PREFIX_ESC}{c}"),
            (c, LanguageStyle::EmacsLisp) => escape(
                c,
                ELISP_CHAR_PREFIX,
                ELISP_CHAR_PREFIX_UNICODE,
                Some(ELISP_CHAR_PREFIX_UNICODE_LONG),
            ),
        }
    }
}

// ------------------------------------------------------------------------------------------------

impl Printable for String {
    fn stringify(&self, style: LanguageStyle) -> String {
        format!(
            "{:?}",
            if self.len() == 1 {
                let c = self.chars().next().unwrap();
                if is_char_nonprintable(c) {
                    string_escape_char(c, style)
                } else {
                    c.to_string()
                }
            } else {
                self.chars()
                    .tuple_windows()
                    .with_position()
                    .map(|(p, (c0, c1))| {
                        if c1 == CHAR_VERTICAL_BAR && c0 != CHAR_BACKSLASH {
                            format!(
                                "{c0}{CHAR_BACKSLASH}{}",
                                if p == Position::Last {
                                    STR_VERTICAL_BAR
                                } else {
                                    STR_EMPTY
                                }
                            )
                        } else {
                            match (p, is_char_nonprintable(c0), is_char_nonprintable(c1)) {
                                (Position::Last | Position::Only, true, true) => {
                                    format!(
                                        "{}{}",
                                        string_escape_char(c0, style),
                                        string_escape_char(c1, style)
                                    )
                                }
                                (Position::Last | Position::Only, true, false) => {
                                    format!("{}{}", string_escape_char(c0, style), c1)
                                }
                                (Position::Last | Position::Only, false, true) => {
                                    format!("{}{}", c0, string_escape_char(c1, style))
                                }
                                (Position::Last | Position::Only, false, false) => {
                                    format!("{}{}", c0, c1)
                                }
                                (_, true, _) => string_escape_char(c0, style),
                                (_, false, _) => c0.to_string(),
                            }
                        }
                    })
                    .collect::<String>()
            }
        )
    }
}

// ------------------------------------------------------------------------------------------------

impl Printable for Vec<Value> {
    fn stringify(&self, style: LanguageStyle) -> String {
        format!(
            "{}{}{}",
            CHAR_LIST_OPEN,
            self.iter()
                .map(|v| v.stringify(style))
                .collect::<Vec<String>>()
                .join(" "),
            CHAR_LIST_CLOSE,
        )
    }
    fn printed_length(&self, style: LanguageStyle) -> usize {
        self.iter().fold(0, |t, v| t + v.printed_length(style))
        // add inter-datum spaces
            + if self.len() < 2 { 0 } else { self.len() - 1 }
    }
    fn print<W>(&self, w: &mut W, style: LanguageStyle) -> Result<(), crate::Error>
    where
        W: std::io::Write,
    {
        if self.is_empty() {
            w.write_all(EMPTY_LIST)?;
        } else {
            w.write_all(LIST_OPEN)?;
            for (value, is_last) in self
                .iter()
                .enumerate()
                .map(|(i, v)| (v, i == self.len() - 1))
            {
                value.print(w, style)?;
                if !is_last {
                    w.write_all(SPACE)?;
                }
            }
            w.write_all(LIST_CLOSE)?;
        }
        Ok(())
    }
    fn pretty_print<W>(
        &self,
        w: &mut W,
        current_indentation: usize,
        line_width: usize,
        style: LanguageStyle,
    ) -> Result<(), crate::Error>
    where
        W: std::io::Write,
    {
        let print_width = self.printed_length(style);
        if self.is_empty() {
            w.write_all(EMPTY_LIST)?;
        } else if current_indentation + print_width < line_width {
            self.print(w, style)?;
        } else {
            let current_indentation = current_indentation + 1; // one '('
            let mut current_width = current_indentation;
            w.write_all(LIST_OPEN)?;
            let last_value_index: usize = self.len() - 1;
            for (i, v) in self.iter().enumerate() {
                current_width += v.printed_length(style);
                v.pretty_print(w, current_indentation, line_width, style)?;
                if i < last_value_index {
                    let next_width = self.get(i + 1).unwrap().printed_length(style);
                    if (current_width + next_width + 1) >= line_width {
                        newline_and_indent(current_indentation, w)?;
                        current_width = current_indentation;
                    } else {
                        w.write_all(SPACE)?;
                    }
                }
            }
            w.write_all(LIST_CLOSE)?;
        }
        Ok(())
    }
}

// ------------------------------------------------------------------------------------------------

impl Printable for Value {
    fn stringify(&self, style: LanguageStyle) -> String {
        match self {
            Value::Bool(v) => v.stringify(style),
            Value::Number(v) => v.stringify(style),
            Value::Character(v) => v.stringify(style),
            Value::String(v) => v.stringify(style),
            Value::Symbol(v) => v.stringify(style),
            Value::Keyword(v) => v.stringify(style),
            Value::List(v) => v.stringify(style),
        }
    }
    fn pretty_print<W>(
        &self,
        w: &mut W,
        current_indentation: usize,
        line_width: usize,
        style: LanguageStyle,
    ) -> Result<(), Error>
    where
        W: Write,
    {
        match self {
            Value::Bool(v) => v.pretty_print(w, current_indentation, line_width, style),
            Value::Number(v) => v.pretty_print(w, current_indentation, line_width, style),
            Value::Character(v) => v.pretty_print(w, current_indentation, line_width, style),
            Value::String(v) => v.pretty_print(w, current_indentation, line_width, style),
            Value::Symbol(v) => v.pretty_print(w, current_indentation, line_width, style),
            Value::Keyword(v) => v.pretty_print(w, current_indentation, line_width, style),
            Value::List(v) => v.pretty_print(w, current_indentation, line_width, style),
        }
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

#[inline(always)]
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

#[inline(always)]
fn newline_and_indent<W>(indent: usize, writer: &mut W) -> Result<(), Error>
where
    W: Write,
{
    writer.write_all(format!("{CHAR_NEWLINE}{:indent$}", " ").as_bytes())?;
    Ok(())
}

fn string_escape_char(c: char, style: LanguageStyle) -> String {
    match style {
        LanguageStyle::Racket => {
            if c.is_control() {
                format!("\\u{:04X}", c as u32)
            } else {
                c.to_string()
            }
        }
        LanguageStyle::TreeSitter => todo!(),
        LanguageStyle::CommonLisp => todo!(),
        LanguageStyle::Scheme => todo!(),
        LanguageStyle::EmacsLisp => todo!(),
    }
}
