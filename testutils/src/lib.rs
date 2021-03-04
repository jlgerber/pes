use std::path::PathBuf;
use uuid::Uuid;

pub use tempfile;

/// generate a random name, given an optional prefix and suffix
pub fn rand_name(prefix: Option<&str>, suffix: Option<&str>) -> String {
    let mut name = Uuid::new_v4().to_string();
    if let Some(prefix) = prefix {
        name = format!("{}{}", prefix, name);
    } 
    if let Some(suffix) = suffix {
        name = format!("{}{}", name, suffix);
    }

    name
}
pub fn rand_file<P: Into<PathBuf>>(prefix: Option<&str>, suffix: Option<&str>, location: P) -> PathBuf {
    let name = rand_name(prefix, suffix);
    let mut location = location.into();
    location.push(name);
    location
}