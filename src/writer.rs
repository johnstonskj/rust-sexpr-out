/*!
One-line description.

More detailed description, with

# Example

End of file during parsingSymbol’s value as variable is void: rustEnd of file during parsing

 */

use crate::{Error, Value};
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

pub trait ToStringFor {
    fn to_string_for(&self, style: LanguageStyle) -> String;
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct Options {
    line_width: usize,
    pair_kw_args: bool,
    wrap_in_define: bool,
    style: LanguageStyle,
}

#[derive(Debug, Default)]
pub struct Writer {
    options: Options,
    pretty_print: bool,
}

// ------------------------------------------------------------------------------------------------
// Implementations ❱ Options
// ------------------------------------------------------------------------------------------------

impl Options {
    pub fn with_line_width(self, line_width: usize) -> Self {
        let mut self_mut = self;
        self_mut.line_width = line_width;
        self_mut
    }

    pub fn line_width(&self) -> &usize {
        &self.line_width
    }

    pub fn set_line_width(&mut self, line_width: usize) {
        self.line_width = line_width;
    }

    // --------------------------------------------------------------------------------------------

    pub fn with_style(self, style: LanguageStyle) -> Self {
        let mut self_mut = self;
        self_mut.style = style;
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
        self_mut.wrap_in_define = wrap_in_define;
        self_mut
    }

    pub fn wrap_in_define(&self) -> &bool {
        &self.wrap_in_define
    }

    pub fn set_wrap_in_define(&mut self, wrap_in_define: bool) {
        self.wrap_in_define = wrap_in_define;
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
            todo!()
        } else {
            Ok(w.write_all(object.to_string_for(self.options.style).as_bytes())?)
        }
    }
}

impl Writer {
    pub fn pretty_printed(self, flag: bool) -> Self {
        let mut self_mut = self;
        self_mut.pretty_print = flag;
        self_mut
    }
}
