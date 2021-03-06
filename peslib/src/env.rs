//! Components involved in setting up the environment for a target
//! - PathToken: enum modeling the tokens one may decompose a env var path string into
//! - PathMode: enum describing the way that a path or paths are composed - eg by prepending, appending or replacing an existing variable
//! - BasicVarProvider: struct used to store and provide path variables to a parser. This implements the `VarProvider` trait found in the `traits` module

use std::collections::HashMap;
use std::collections::VecDeque;
use std::path::{
    Path,
};

use crate::{
    PesError,
    VarProvider,
};

/// Parsed environment paths can be decomposed into a vector of PathTokens
/// 
/// # Example
///
/// ```{root}/python``` -> ```vec![PathToken::RootVar,PathToken::Relpath(Path::new("python")]```
///
#[derive(Debug, PartialEq, Eq)]
pub enum PathToken<'a> {
    /// Variable representing the root of a package
    RootVar,
    /// Variable, rendered as ```{<VAR NAME>}```
    Variable(&'a str),
    /// OwnedVariable, making lifetime gymnastics simpler at the cost of an allocation.
    OwnedVariable(String),
    /// Error state. unknown variable
    UnknownVariable(&'a str),
    /// A relative path
    Relpath(&'a std::path::Path),
    /// An absolute path
    Abspath(&'a std::path::Path),
}

impl<'a> PathToken<'a> {
    /// Construct a PathToken::Variable
    pub fn variable<'b: 'a>(value: &'b str) -> Self {
        Self::Variable(value)
    }
    /// Construct a PathToken::OwnedVariable
    pub fn owned_variable<V: Into<String>>(value: V) -> Self {
        Self::OwnedVariable(value.into())
    }

    /// Construct a relpath or abspath
    pub fn path<'b: 'a, P: AsRef<Path> +?Sized>(value: &'b P) -> Self {
        let path = value.as_ref();
        if path.is_absolute() {
            Self::Abspath(path)
        } else {
            Self::Relpath(path)
        }
    }

    /// Construct a PathToken::Relpath
    pub fn relpath<'b: 'a, P: AsRef<Path> + ?Sized>(value: &'b P) -> Self {
        let value = value.as_ref();
        if value.is_absolute() {
            panic!("{:?} is absolute path", value);
        }
        Self::Relpath(value)
    }

    /// Construct a PathToken::Abspath
    pub fn abspath<'b: 'a, P: AsRef<Path>+ ?Sized>(value: &'b P) -> Self {
        let value = value.as_ref();
        if value.is_relative() {
            panic!("{:?} is relative path", value);
        }
        Self::Abspath(value)
    }
}


/// An environment variable setting may be tranformed into a Mode 
/// wrapping a vector of PathBuf or PathBuf
///
#[derive(Debug, PartialEq, Eq)]
pub enum PathMode {
    Append(VecDeque<String>),
    Prepend(VecDeque<String>),
    Exact(VecDeque<String>)
}

impl PathMode {
    pub fn inner(self) -> VecDeque<String> {
        match self {
            Self::Append(me) | Self::Prepend(me) | Self::Exact(me) => {
                me
            }
        }
    }
}
use std::ops::{Add, AddAssign};
impl Add for PathMode {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        match self {
            PathMode::Append(mut me) | PathMode::Prepend(mut me) | PathMode::Exact(mut me) =>  {
                match other {
                    PathMode::Prepend(you) => {
                        for v in you {
                            me.push_front(v);
                        }
                        PathMode::Prepend(me)
                    },
                    PathMode::Append(you) => {
                        for v in you {
                            me.push_back(v);
                        }
                        PathMode::Append(me)
                    },
                    PathMode::Exact(you) => PathMode::Exact(you)
                }
            }
        }
    }
}

impl AddAssign for PathMode {

    fn add_assign(&mut self, mut other: Self)  {
         match self {
            PathMode::Append(ref mut me) | PathMode::Prepend(ref mut me) | PathMode::Exact(ref mut me) =>  {
                match other {
                    PathMode::Prepend(you) => {
                        for v in you {
                            me.push_front(v);
                        }
                        
                    },
                    PathMode::Append(you) => {
                        for v in you {
                            me.push_back(v);
                        }
                    },
                    PathMode::Exact(ref mut you) => {
                        me.clear();
                        me.append(you);
                    }
                }
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
/// Provides variables to the parser. It is up to the user
/// to insert variables prior to passing to the parser.
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
    /// Construct a default ```BasicVarProvider```
    pub fn new() -> Self {
        Self::default()
    }

    /// specify an environment variable name to look up the value for. If it does not 
    /// exist, then an Err(PesError::MissinvEnvVar) is returned. Otherwise, an Ok(())
    /// is returned.
    pub fn insert_env_var(&mut self, variable: &str) -> Result<(), PesError> {
        let value = std::env::var(variable)?;
        let _ = self.insert(variable, value);
        Ok(())
    }

    /// Given a key and value, insert them into the provider
    pub fn insert_var(&mut self, key: impl Into<String>, value: impl Into<String>)  {
        self.insert(key, value);
    }
}

impl<'a> VarProvider<'a> for BasicVarProvider {
    type Returns = &'a str;
    type Key = String;
    type Value = String;

    fn insert<K: Into<Self::Key>, V: Into<Self::Value> >(&mut self, k: K, v: V) -> Option<Self::Value> {
        self.inner.insert(k.into(),v.into())
    }
    
    fn get(&'a self, value: impl AsRef<str>) -> Option<Self::Returns> {
        self.inner.get(value.as_ref()).map(|x| x.as_ref())
    }
}


#[cfg(test)]
#[path = "./unit_tests/env.rs"]
mod unit_tests;