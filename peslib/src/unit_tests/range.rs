#![allow(non_snake_case)]

use super::*;
use pubgrub::range::Range;
use crate::SemanticVersion;
use crate::ReleaseType;


#[test]
fn frm_str__when_given_valid_str_returns_ok() {
    let ranges = &[
        (
            "1.2.3+<4", 
            Range::between(
                SemanticVersion::new(1,2,3,ReleaseType::Release),
                SemanticVersion::new(4,0,0,ReleaseType::Release)
            )
        ),
        (
            "1.2.3-beta+<4", 
            Range::between(
                SemanticVersion::new(1,2,3,ReleaseType::Beta),
                SemanticVersion::new(4,0,0,ReleaseType::Release)
            )
        ),
        // TODO: Did I get the spec right for ^? I think i mixed it up with ~
        (
            "^1", 
            Range::between(
                SemanticVersion::new(1,0,0,ReleaseType::Release),
                SemanticVersion::new(2,0,0,ReleaseType::Release)
            )
        ),
        (
            "^0.1", 
            Range::between(
                SemanticVersion::new(0,1,0,ReleaseType::Release),
                SemanticVersion::new(0,2,0,ReleaseType::Release)
            )
        ),
        (
            "2.0.2", 
            Range::exact(
                SemanticVersion::new(2,0,2,ReleaseType::Release),
            )
        )
    ];

    for range_tup in ranges {
        let range = SemVerRange::frm_str(range_tup.0);
        assert!(range.is_ok());
        
        assert_eq!(range.unwrap(), range_tup.1);
    }
}