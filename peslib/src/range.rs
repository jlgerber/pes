//! Provides an implementation of the `FrmStr` trait for `Range<SematicVersion>`
use crate::PesError;
use crate::parser::parse_consuming_semver_range;
use crate::traits::FrmStr;
use crate::SemanticVersion;

use pubgrub::range::Range;
//use pubgrub::version::SemanticVersion;

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