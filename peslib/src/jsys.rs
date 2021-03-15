use log::debug;
use std::env;
use std::ffi::CString;
use std::path::{
    PathBuf
};

use crate::{
    BaseEnv,
    PesError,
    ManifestLocationProvider,
    constants::MANIFEST_NAME,
    Manifest,
    env::PathMode,
};

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

use std::collections::HashMap;
use std::collections::VecDeque;
type EnvVarMap = HashMap<String, PathMode>;
impl JsysCleanEnv {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn base_env2(self) -> EnvVarMap {
        let mut retmap = EnvVarMap::new();
        for var in self.vars {
            let value = match env::var(&var) {
                Ok(val) => val,
                Err(_) =>  String::new()
            };
            debug!("Value: {} {}", var, &value);
            let paths = value.split(":").map(|x| x.to_string()).collect::<Vec<_>>();
            retmap.insert(var.to_string(), PathMode::Prepend(VecDeque::from(paths)));
        }
        retmap
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


    fn keys(&self) -> &'static [&'static str] {
        self.vars
    }
}


/// NOT CURRENTLY USED
#[derive(Debug)]
pub struct ManifestFactory;

impl ManifestLocationProvider for ManifestFactory {
    fn find<P: Into<PathBuf>>(&self, distribution: P) -> PathBuf {
        let mut path = distribution.into();
        path.push(MANIFEST_NAME);
        path
    }

    fn manifest<P: Into<PathBuf>>(&self, distribution: P) -> Result<Manifest, PesError> {
        let manifest = self.find(distribution);
        Manifest::from_path(manifest)
    }
}

#[cfg(test)]
#[path = "./unit_tests/jsys.rs"]
mod unit_tests;