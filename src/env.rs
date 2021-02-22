//! Components involved in setting up the environment for a target
//! - PathToken: enum modeling the tokens one may decompose a env var path string into
//! - PathMode: enum describing the way that a path or paths are composed - eg by prepending, appending or replacing an existing variable
//! - BasicVarProvider: struct used to store and provide path variables to a parser. This implements the `VarProvider` trait found in the `traits` module

use std::collections::HashMap;
use std::path::{
    Path,
    PathBuf
};

use crate::{
    error::PesError,
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
    /// Separates tokes representing a path. Typically rendered as ":"
    Separator,
    /// Variable representing the root of a package
    RootVar,
    /// Variable, rendered as ```{<VAR NAME>}```
    Variable(&'a str),
    /// OwnedVariable, making lifetime gymnastics simpler at the cost of an allocation.
    OwnedVariable(String),
    /// Error state. unknown variable
    UnknownVariable(&'a str),
    /// A SubPath - that is a part of the path having no special 
    /// tokens represented by the other variants
    Relpath(&'a std::path::Path),
    Abspath(&'a std::path::Path),
    /// Token indicating that previous path compoennts should be 
    /// prepended to the existing environment path variable
    Prepend,
    /// Token indicating that subsequent path tokens should be 
    /// appended to the existing environment path variable
    Append,
}

impl<'a> PathToken<'a> {
    /// Construct a PathToken::Var
    pub fn variable<'b: 'a>(value: &'b str) -> Self {
        Self::Variable(value)
    }

    /// Construct a PathToken::Relpath
    pub fn relpath<'b: 'a, P: AsRef<Path> + ?Sized>(value: &'b P) -> Self {
        Self::Relpath(value.as_ref())
    }

    /// Construct a PathToken::Abspath
    pub fn abspath<'b: 'a, P: AsRef<Path>+ ?Sized>(value: &'b P) -> Self {
        Self::Abspath(value.as_ref())
    }
}


/// An environment variable setting may be tranformed into a Mode 
/// wrapping a vector of PathBuf or PathBuf
///
#[derive(Debug, PartialEq, Eq)]
pub enum PathMode {
    Append(Vec<PathBuf>),
    Prepend(Vec<PathBuf>),
    Exact(Vec<PathBuf>)
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