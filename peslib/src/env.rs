//! Components involved in setting up the environment for a target
//! - PathToken: enum modeling the tokens one may decompose a env var path string into
//! - PathMode: enum describing the way that a path or paths are composed - eg by prepending, appending or replacing an existing variable
//! - BasicVarProvider: struct used to store and provide path variables to a parser. This implements the `VarProvider` trait found in the `traits` module

use std::collections::HashMap;
use std::env;
use std::ffi::CString;
use std::path::{
    Path,
    PathBuf
};

use crate::{
    BaseEnv,
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


#[derive(Debug)]
/// Provides a base environment for the Jsys system
pub struct JsysCleanEnv {
    vars: &'static[&'static str]
}

impl Default for JsysCleanEnv {
    fn default() -> Self {
        Self {
            vars: &[
                "JSYS_PROJECT", 
                "JSYS_SEQUENCE", 
                "JSYS_SHOT", 
                "JSYS_LEVEL", 
                "JSYS_ROOT",
                "_",
                "COLORTERM",
                "DBUS_SESSION_BUS_ADDRESS",
                "DEFAULTS_PATH",
                "DESKTOP_SESSION",
                "DISPLAY",
                "GDMSESSION",
                "G_ENABLE_DIAGNOSTIC",
                "GNOME_DESKTOP_SESSION_ID",
                "GNOME_SHELL_SESSION_MODE",
                "GNOME_TERMINAL_SCREEN",
                "GNOME_TERMINAL_SERVICE",
                "GPG_AGENT_INFO",
                "GTK_IM_MODULE",
                "GTK_MODULES",
                "HOME",
                "INVOCATION_ID",
                "JOURNAL_STREAM",
                "LANG",
                "LANGUAGE",
                "LC_ADDRESS",
                "LC_IDENTIFICATION",
                "LC_MEASUREMENT",
                "LC_MONETARY",
                "LC_NAME",
                "LC_NUMERIC",
                "LC_PAPER",
                "LC_TELEPHONE",
                "LC_TIME",
                "LESSCLOSE",
                "LESSOPEN",
                "LOGNAME",
                "LS_COLORS",
                "MANAGERPID",
                "MANDATORY_PATH",
                "PAPERSIZE",
                "PATH",
                "PWD",
                "QT_ACCESSIBILITY",
                "QT_IM_MODULE",
                "SESSION_MANAGER",
                "SHELL",
                "SHLVL",
                "SSH_AGENT_PID",
                "SSH_AUTH_SOCK",
                "TERM",
                "USER",
                "USERNAME",
                "VTE_VERSION",
                "WINDOWPATH",
                "XAUTHORITY",
                "XDG_CONFIG_DIRS",
                "XDG_CURRENT_DESKTOP",
                "XDG_DATA_DIRS",
                "XDG_MENU_PREFIX",
                "XDG_RUNTIME_DIR",
                "XDG_SESSION_CLASS",
                "XDG_SESSION_DESKTOP",
                "XDG_SESSION_TYPE",
                "XMODIFIERS"
                
            ]
        }
    }
}

impl JsysCleanEnv {
    pub fn new() -> Self {
        Self::default()
    }
}

impl BaseEnv for JsysCleanEnv {

    fn base_env(&self) -> Vec<CString> {
        // TODO: could use partition here to split results into 
        // success and failure values instead of filtering out failures (assuming no failures really)
        self.vars.iter().filter_map(|x| 
            CString::new(match env::var(x) {
                Ok(val) => format!("{}={}",x, val),
                Err(_) => format!("{}=", x),
            }).ok()
        ).collect()
    }
}

#[cfg(test)]
#[path = "./unit_tests/env.rs"]
mod unit_tests;