#![allow(non_snake_case)]

use super::*;
use pubgrub::range::Range;
use pubgrub::version::SemanticVersion;

#[test]
fn frm_str__when_given_valid_str_returns_ok() {
    let ranges = &[
        (
            "1.2.3+<4", 
            Range::between(
                SemanticVersion::new(1,2,3),
                SemanticVersion::new(4,0,0)
            )
        ),
        // TODO: Did I get the spec right for ^? I think i mixed it up with ~
        (
            "^1", 
            Range::between(
                SemanticVersion::new(1,0,0),
                SemanticVersion::new(2,0,0)
            )
        ),
        (
            "^0.1", 
            Range::between(
                SemanticVersion::new(0,1,0),
                SemanticVersion::new(0,2,0)
            )
        )
    ];

    for range_tup in ranges {
        let range = SemVerRange::frm_str(range_tup.0);
        assert!(range.is_ok());
        
        assert_eq!(range.unwrap(), range_tup.1);
    }
}