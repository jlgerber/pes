use std::fmt;
use std::collections::HashMap;
use crate::VarProvider;

/// How to compose the environment
#[derive(Debug, PartialEq, Eq)]
pub enum Mode<T: fmt::Debug + PartialEq + Eq> {
    Append(T),
    Prepend(T),
    Exact(T)
}

pub struct BasicVarProvider {
    inner: HashMap<String, String>
}

impl Default for BasicVarProvider {
    fn default() -> Self {
        Self {
            inner: HashMap::new()
        }
    }
}

impl BasicVarProvider {
    pub fn new() -> Self {
        Self::default()
    }
}

impl<'a> VarProvider<'a> for BasicVarProvider {
    type Returns = &'a str;

    fn get(&'a self, value: impl AsRef<str>) -> Option<Self::Returns> {
        self.inner.get(value.as_ref()).map(|x| x.as_ref())
    }
}