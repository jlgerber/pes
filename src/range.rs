use crate::parser::parse_consuming_semver_range;
use crate::error::PesError;

use pubgrub::range::Range;
use pubgrub::version::SemanticVersion;

/// Trait to provide an alternative, falible constructor from a &str
pub trait FrmStr {
    type FrmStrErr;

    /// Given a str, construct an instance of Self
    fn frm_str(value: &str) -> Result<Self, Self::FrmStrErr> where Self: Sized;
}

pub type SemVerRange = Range<SemanticVersion>;

impl FrmStr for Range<SemanticVersion> {
    type FrmStrErr = PesError;

    fn frm_str(value: &str) -> Result<Self, Self::FrmStrErr> where Self: Sized {
        parse_consuming_semver_range(value)
    }
}



#[cfg(test)]
#[path = "./unit_tests/range.rs"]
mod unit_tests;