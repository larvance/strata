//! Strata JS: Porting modern JavaScript frontend paradigms, dynamic typing, and Web APIs into safe Rust.

use std::collections::BTreeMap;
use std::ops::Index;

/// Tagged union representing dynamic JavaScript values.
#[derive(Debug, Clone, PartialEq)]
pub enum JsValue {
    Undefined,
    Null,
    Bool(bool),
    Number(f64),
    String(String),
    Array(Vec<JsValue>),
    Object(BTreeMap<String, JsValue>),
}

impl JsValue {
    pub fn is_null(&self) -> bool {
        matches!(self, Self::Null)
    }

    pub fn is_undefined(&self) -> bool {
        matches!(self, Self::Undefined)
    }

    pub fn is_truthy(&self) -> bool {
        match self {
            Self::Undefined | Self::Null => false,
            Self::Bool(b) => *b,
            Self::Number(n) => *n != 0.0 && !n.is_nan(),
            Self::String(s) => !s.is_empty(),
            Self::Array(_) | Self::Object(_) => true,
        }
    }

    pub fn as_f64(&self) -> Option<f64> {
        match self {
            Self::Number(n) => Some(*n),
            _ => None,
        }
    }

    pub fn as_str(&self) -> Option<&str> {
        match self {
            Self::String(s) => Some(s.as_str()),
            _ => None,
        }
    }

    pub fn as_bool(&self) -> Option<bool> {
        match self {
            Self::Bool(b) => Some(*b),
            _ => None,
        }
    }

    pub fn as_array(&self) -> Option<&[JsValue]> {
        match self {
            Self::Array(arr) => Some(arr.as_slice()),
            _ => None,
        }
    }

    pub fn as_array_mut(&mut self) -> Option<&mut Vec<JsValue>> {
        match self {
            Self::Array(arr) => Some(arr),
            _ => None,
        }
    }

    pub fn map<F>(&self, mut f: F) -> Self
    where
        F: FnMut(&JsValue) -> JsValue,
    {
        match self {
            Self::Array(arr) => Self::Array(arr.iter().map(&mut f).collect()),
            _ => Self::Array(Vec::new()),
        }
    }

    pub fn filter<F>(&self, mut f: F) -> Self
    where
        F: FnMut(&JsValue) -> bool,
    {
        match self {
            Self::Array(arr) => Self::Array(arr.iter().filter(|item| f(item)).cloned().collect()),
            _ => Self::Array(Vec::new()),
        }
    }

    pub fn reduce<F>(&self, mut f: F, initial: JsValue) -> JsValue
    where
        F: FnMut(JsValue, &JsValue) -> JsValue,
    {
        match self {
            Self::Array(arr) => arr.iter().fold(initial, |acc, item| f(acc, item)),
            _ => initial,
        }
    }

    pub fn len(&self) -> usize {
        match self {
            Self::Array(arr) => arr.len(),
            Self::String(s) => s.len(),
            Self::Object(obj) => obj.len(),
            _ => 0,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl Index<&str> for JsValue {
    type Output = JsValue;

    fn index(&self, key: &str) -> &Self::Output {
        static UNDEFINED: JsValue = JsValue::Undefined;
        match self {
            Self::Object(map) => map.get(key).unwrap_or(&UNDEFINED),
            _ => &UNDEFINED,
        }
    }
}

impl Index<usize> for JsValue {
    type Output = JsValue;

    fn index(&self, index: usize) -> &Self::Output {
        static UNDEFINED: JsValue = JsValue::Undefined;
        match self {
            Self::Array(arr) => arr.get(index).unwrap_or(&UNDEFINED),
            _ => &UNDEFINED,
        }
    }
}

impl From<bool> for JsValue {
    fn from(v: bool) -> Self {
        Self::Bool(v)
    }
}

impl From<f64> for JsValue {
    fn from(v: f64) -> Self {
        Self::Number(v)
    }
}

impl From<i32> for JsValue {
    fn from(v: i32) -> Self {
        Self::Number(v as f64)
    }
}

impl From<usize> for JsValue {
    fn from(v: usize) -> Self {
        Self::Number(v as f64)
    }
}

impl From<&str> for JsValue {
    fn from(v: &str) -> Self {
        Self::String(v.to_string())
    }
}

impl From<String> for JsValue {
    fn from(v: String) -> Self {
        Self::String(v)
    }
}

/// Web APIs
pub mod web {
    use super::*;

    #[derive(Debug, Default, Clone)]
    pub struct Headers {
        entries: BTreeMap<String, String>,
    }

    impl Headers {
        pub fn new() -> Self {
            Self::default()
        }

        pub fn set(&mut self, key: impl Into<String>, val: impl Into<String>) {
            self.entries.insert(key.into().to_lowercase(), val.into());
        }

        pub fn get(&self, key: &str) -> Option<&str> {
            self.entries.get(&key.to_lowercase()).map(|s| s.as_str())
        }
    }

    #[derive(Debug, Default, Clone)]
    pub struct RequestInit {
        pub method: String,
        pub headers: Headers,
        pub body: Option<String>,
    }

    #[derive(Debug, Clone)]
    pub struct Response {
        pub status: u16,
        pub ok: bool,
        pub body: String,
    }

    impl Response {
        pub async fn text(self) -> Result<String, String> {
            Ok(self.body)
        }

        pub async fn json(self) -> Result<JsValue, String> {
            // Emulated JSON parser
            Ok(JsValue::Null)
        }
    }

    pub async fn fetch(_url: &str, _init: RequestInit) -> Result<Response, String> {
        Ok(Response {
            status: 200,
            ok: true,
            body: "{}".into(),
        })
    }
}

pub mod prelude {
    pub use super::web::*;
    pub use super::JsValue;
}
