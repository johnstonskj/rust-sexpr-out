/*!
This module provides the common [`Value`] enum for atomic values (bool, integer, float, character,
string, symbol, and keyword) and lists of values.

Similar in approach to crates such as `serde_json` the [`Value`] enum is the representation of data
supported directly by the representation, in this case Lisp-like s-expressions. Values are
independent of *style* (corresponding to common Lisp families) which is applied when written.

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

impl Symbol {
    pub fn new<S>(s: S) -> Self
    where
        S: Into<String>,
    {
        Self(s.into())
    }

    pub fn inner(&self) -> String {
        self.0.clone()
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

impl Keyword {
    pub fn new<S>(s: S) -> Self
    where
        S: Into<String>,
    {
        Self(Symbol::new(s.into()))
    }

    pub fn inner(&self) -> Symbol {
        self.0.clone()
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

// ------------------------------------------------------------------------------------------------
// Implementations ❱ Value
// ------------------------------------------------------------------------------------------------

impl Value {
    pub fn empty_list() -> Self {
        Self::List(Vec::default())
    }
}
