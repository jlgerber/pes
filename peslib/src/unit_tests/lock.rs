//! tests for the lockfile
#![allow(non_snake_case)]

use super::*;

const LOCKFILE1: &str = 
r#"
schema = 1
request = "pez env -p bar-2.4.3 -t run -t build"
author = "jgerber"

[lock.run]
foo = "1.2.3"
bar = "2.4.3"

[lock.build]
foo = "1.2.4"
bar = "2.0.1"
somelib = "1.2.3"
"#;

#[test]
fn from_str__when_given_good_data__succeeds() {
    let lf = LockFile::from_str(LOCKFILE1);
    assert!(lf.is_ok());
}