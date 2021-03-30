
#![allow(non_snake_case)]

use super::*;
use nom::error::ErrorKind;
use crate::PesNomError;
use crate::{SemanticVersion, ReleaseType};


use crate::{
    constants,
    parser::variant::{
        parse_carrot_explicit_variant_semver_range,
        parse_carrot_variant_semver_range,
        parse_consuming_variant_semver_exact_range,
    }
};



mod parse_consuming_package_variants {
    use super::*;

    #[test]
    fn given_an_explicit_variant__succeeds() {
        let result = parse_consuming_package_variants("maya-1.2.3-beta@1");
        assert_eq!(
            result.unwrap(), 
            (
                "maya",
                Range::exact(Variant::new(SemanticVersion::new(1,2,3, ReleaseType::Beta), 1) )
            )
        );
    }
    
    #[test]
    fn given_an_implicit_variant__succeeds() {
        let result = parse_consuming_package_variants("maya-1.2.3-beta");
        assert_eq!(
            result.unwrap(), 
            
            (
                "maya",
                Range::between(
                    Variant::new(SemanticVersion::new(1,2,3, ReleaseType::Beta), 0),
                    Variant::new(SemanticVersion::new(1,2,3, ReleaseType::Beta), constants::MAX_VARIANTS)
                )
            )
            
        );
    }
    
    #[test]
    fn given_a_short_input__succeeds() {
        let result = parse_consuming_package_variants("maya-1");
        assert_eq!(
            result.unwrap(), 
            
            (
                "maya",
                Range::between(
                    Variant::new(SemanticVersion::new(1, 0, 0, ReleaseType::Release), 0),
                    Variant::new(SemanticVersion::new(1, 0, 0, ReleaseType::Release), constants::MAX_VARIANTS)
                )
            )
            
        );
    }
    
    #[test]
    fn given_other_input__fails() {
        let failures = &["maya-^1.2.3",  "foo-1.2.3+<4.3.3"];
        for failure in failures.iter() {
            let result = parse_consuming_package_variants(failure);
            // just for reporting purposes. If we inadvertently find something which should be 
            // an error but isn't, lets get a good look at it.
            // if !result.is_err() {
            //     assert_eq!(result.unwrap(), (*failure, Range::exact(Variant::new(SemanticVersion::new(1,2,3, ReleaseType::Release), 0))) );
            // } 
            assert!(result.is_err());
    
        }
    }
}

mod parse_package_variants {
    use super::*;

    #[test]
    fn given_an_explicit_variant__succeeds() {
        let result = parse_package_variants("maya-1.2.3-beta@1");
        assert_eq!(
            result.unwrap(), 
            (
                "",
                (
                    "maya",
                    Range::exact(Variant::new(SemanticVersion::new(1,2,3, ReleaseType::Beta), 1) )
                )
            )
        );
    }
    
    #[test]
    fn given_an_implicit_variant__succeeds() {
        let result = parse_package_variants("maya-1.2.3-beta");
        assert_eq!(
            result.unwrap(), 
            (
                "",
                (
                    "maya",
                    Range::between(
                        Variant::new(SemanticVersion::new(1,2,3, ReleaseType::Beta), 0),
                        Variant::new(SemanticVersion::new(1,2,3, ReleaseType::Beta), constants::MAX_VARIANTS)
                    )
                )
            )
        );
    }
    #[test]
    fn given_carrot_semver__succeeds() {
        let result = parse_package_variants("maya-1.2.3-beta");
        assert_eq!(
            result.unwrap(), 
            (
                "",
                (
                    "maya",
                    Range::between(
                        Variant::new(SemanticVersion::new(1,2,3, ReleaseType::Beta), 0),
                        Variant::new(SemanticVersion::new(1,2,3, ReleaseType::Beta), constants::MAX_VARIANTS)
                    )
                )
            )
        );
    }
}

mod parse_package_variant {
    use super::*;
    #[test]
    fn given_expected_input__succeeds() {
        let result = parse_package_variant("maya-1.2.3@1");
        assert_eq!(result.unwrap(), ("",("maya",Variant::new(SemanticVersion::new(1,2,3, ReleaseType::Release),1))));
    }
    
}

mod parse_consuming_package_variant {
    use super::*;
   
    #[test]
    fn given_expected_input__succeeds() {
        let result = parse_consuming_package_variant("maya-1.2.3@1");
        assert_eq!(result.unwrap(), ("maya",Variant::new(SemanticVersion::new(1,2,3, ReleaseType::Release),1)));
    }

}

mod parse_consuming_variant_semver {
    use super::*;

    #[test]
    fn given_release__succeeds() {
        let result = parse_consuming_variant_semver("1.2.3@1");
        assert_eq!(result.unwrap(), Variant::new(SemanticVersion::new(1,2,3, ReleaseType::Release),1));
    }

    #[test]
    fn given_prerelease__succeeds() {
        let result = parse_consuming_variant_semver("1.2.3-beta@1");
        assert_eq!(result.unwrap(), Variant::new(SemanticVersion::new(1,2,3, ReleaseType::Beta),1));
    }

}


// RANGE<VARIANT<SEMANTICVERSION>> with Exact Range (eg 1.2.3@1)
mod parse_consuming_variant_semver_exact_range {
    use super::*;
    
    #[test]
    fn given_release__succeeds() {
        let result = parse_consuming_variant_semver_exact_range("1.2.3@1");
        assert_eq!(result.unwrap(), Range::exact(Variant::new(SemanticVersion::new(1,2,3, ReleaseType::Release),1)));
    }
    
    #[test]
    fn given_prerelease__succeeds() {
        let result = parse_consuming_variant_semver_exact_range("1.2.3-beta@1");
        assert_eq!(result.unwrap(), Range::exact(Variant::new(SemanticVersion::new(1,2,3, ReleaseType::Beta),1)));
    }
}

/**
        parse_consuming_variant_semver_implicit_range tests
*/
mod parse_consuming_variant_semver_implicit_range {
    use super::*;

    #[test]
    fn given_release__succeeds() {
        let result = parse_consuming_semver_with_implicit_variant_range("1.2.3");
        assert_eq!(
            result.unwrap(), 
            Range::between(
                Variant::new(SemanticVersion::new(1,2,3, ReleaseType::Release),0),
                Variant::new(SemanticVersion::new(1,2,3, ReleaseType::Release),constants::MAX_VARIANTS),
            )
        );
    }
    
    #[test]
    fn given_prerelease__succeeds() {
        let result = parse_consuming_semver_with_implicit_variant_range("1.2.3-beta");
        assert_eq!(
            result.unwrap(), 
            Range::between(
                Variant::new(SemanticVersion::new(1,2,3, ReleaseType::Beta),0),
                Variant::new(SemanticVersion::new(1,2,3, ReleaseType::Beta),constants::MAX_VARIANTS),
            )
        );
    }
}

/* 
        parse_carrot_variant_semver_range tests
*/
mod parse_carrot_variant_semver_range {
    use super::*;

    #[test]
    fn given_explicit_variant__succeeds() {
        let result = parse_carrot_variant_semver_range("^1.2.3@1");
        let expected = (
            "",
            Range::between(
                Variant::new(SemanticVersion::new(1,2,3, ReleaseType::Release), 1),
                Variant::new(SemanticVersion::new(1,2,4, ReleaseType::Release),1)
            )
        );
        assert_eq!(result.unwrap(), expected);
    }
    
    #[test]
    fn given_implicit_variant__succeeds() {
        let result = parse_carrot_variant_semver_range("^1.2.3");
        let expected = (
            "",
            Range::between(
                Variant::new(SemanticVersion::new(1,2,3, ReleaseType::Release), 0),
                Variant::new(SemanticVersion::new(1,2,4, ReleaseType::Release), constants::MAX_VARIANTS)
            )
        );
        assert_eq!(result.unwrap(), expected);
    }
}

mod parse_semver_variants_between {
    use super::*;

    #[test]
    fn given_two_implicit_variants_separated_by_pgt_succeeds() {
        let result = parse_semver_variants_between("1.2.3+<2.2.2");
        let expected = (
            "",
            Range::between(
                Variant::new(SemanticVersion::new(1,2,3, ReleaseType::Release), 0),
                Variant::new(SemanticVersion::new(2,2,2, ReleaseType::Release),constants::MAX_VARIANTS)
            )
        );

        assert_eq!(result.unwrap(), expected);
    }

    #[test]
    fn given_two_implicit_variants_separated_by_dotdot_succeeds() {
        let result = parse_semver_variants_between("1.2.3..2.2.2");
        let expected = (
            "",
            Range::between(
                Variant::new(SemanticVersion::new(1,2,3, ReleaseType::Release), 0),
                Variant::new(SemanticVersion::new(2,2,2, ReleaseType::Release),constants::MAX_VARIANTS)
            )
        );

        assert_eq!(result.unwrap(), expected);
    }


    #[test]
    fn given_one_explicit_and_one_implicit_variant_separated_by_pgt_succeeds() {
        let result = parse_semver_variants_between("1.2.3@2+<2.2.2");
        let expected = (
            "",
            Range::between(
                Variant::new(SemanticVersion::new(1,2,3, ReleaseType::Release), 2),
                Variant::new(SemanticVersion::new(2,2,2, ReleaseType::Release),constants::MAX_VARIANTS)
            )
        );

        assert_eq!(result.unwrap(), expected);
    }

    #[test]
    fn given_one_explicit_and_one_implicit_variant_separated_by_dotdot_succeeds() {
        let result = parse_semver_variants_between("1.2.3@2..2.2.2");
        let expected = (
            "",
            Range::between(
                Variant::new(SemanticVersion::new(1,2,3, ReleaseType::Release), 2),
                Variant::new(SemanticVersion::new(2,2,2, ReleaseType::Release),constants::MAX_VARIANTS)
            )
        );

        assert_eq!(result.unwrap(), expected);
    }

    #[test]
    fn given_one_implicit_and_one_explicit_variant_separated_by_pgt_succeeds() {
        let result = parse_semver_variants_between("1.2.3+<2.2.2@4");
        let expected = (
            "",
            Range::between(
                Variant::new(SemanticVersion::new(1,2,3, ReleaseType::Release), 0),
                Variant::new(SemanticVersion::new(2,2,2, ReleaseType::Release), 4)
            )
        );

        assert_eq!(result.unwrap(), expected);
    }

    #[test]
    fn given_one_implicit_and_one_explicit_variant_separated_by_dotdot_succeeds() {
        let result = parse_semver_variants_between("1.2.3..2.2.2@4");
        let expected = (
            "",
            Range::between(
                Variant::new(SemanticVersion::new(1,2,3, ReleaseType::Release), 0),
                Variant::new(SemanticVersion::new(2,2,2, ReleaseType::Release), 4)
            )
        );

        assert_eq!(result.unwrap(), expected);
    }

    #[test]
    fn given_two_explicit_variants_separated_by_pgt_succeeds() {
        let result = parse_semver_variants_between("1.2.3@2+<2.2.2@4");
        let expected = (
            "",
            Range::between(
                Variant::new(SemanticVersion::new(1,2,3, ReleaseType::Release), 2),
                Variant::new(SemanticVersion::new(2,2,2, ReleaseType::Release), 4)
            )
        );

        assert_eq!(result.unwrap(), expected);
    }

    #[test]
    fn given_two_explicit_variants_separated_by_dotdot_succeeds() {
        let result = parse_semver_variants_between("1.2.3@2..2.2.2@4");
        let expected = (
            "",
            Range::between(
                Variant::new(SemanticVersion::new(1,2,3, ReleaseType::Release), 2),
                Variant::new(SemanticVersion::new(2,2,2, ReleaseType::Release), 4)
            )
        );

        assert_eq!(result.unwrap(), expected);
    }
}


mod parse_package_variants_range {
    use super::*;

    #[test]
    fn given_two_implicit_variants_separated_by_pgt_succeeds() {
        let result = parse_package_variants_range("maya-1.2.3+<2.2.2");
        let expected = (
            "",
            ("maya",
                Range::between(
                    Variant::new(SemanticVersion::new(1,2,3, ReleaseType::Release), 0),
                    Variant::new(SemanticVersion::new(2,2,2, ReleaseType::Release),constants::MAX_VARIANTS)
                )
            )
        );

        assert_eq!(result.unwrap(), expected);
    }

    #[test]
    fn given_two_implicit_variants_separated_by_dotdot_succeeds() {
        let result = parse_package_variants_range("maya-1.2.3..2.2.2");
        let expected = (
            "",
            ("maya",
                Range::between(
                    Variant::new(SemanticVersion::new(1,2,3, ReleaseType::Release), 0),
                    Variant::new(SemanticVersion::new(2,2,2, ReleaseType::Release),constants::MAX_VARIANTS)
                )
            )
        );

        assert_eq!(result.unwrap(), expected);
    }

    #[test]
    fn given_one_explicit_and_one_implicit_variant_separated_by_pgt_succeeds() {
        let result = parse_package_variants_range("maya-1.2.3@2+<2.2.2");
        let expected = (
            "",
            (
                "maya",
                Range::between(
                    Variant::new(SemanticVersion::new(1,2,3, ReleaseType::Release), 2),
                    Variant::new(SemanticVersion::new(2,2,2, ReleaseType::Release),constants::MAX_VARIANTS)
                )
            )
        );

        assert_eq!(result.unwrap(), expected);
    }

    #[test]
    fn given_one_explicit_and_one_implicit_variant_separated_by_dotdot_succeeds() {
        let result = parse_package_variants_range("maya-1.2.3@2..2.2.2");
        let expected = (
            "",
            ( 
                "maya",
                Range::between(
                    Variant::new(SemanticVersion::new(1,2,3, ReleaseType::Release), 2),
                    Variant::new(SemanticVersion::new(2,2,2, ReleaseType::Release),constants::MAX_VARIANTS)
                )
            )
        );

        assert_eq!(result.unwrap(), expected);
    }

    #[test]
    fn given_one_implicit_and_one_explicit_variant_separated_by_pgt_succeeds() {
        let result = parse_package_variants_range("maya-1.2.3+<2.2.2@4");
        let expected = (
            "",
            (
                "maya",
                Range::between(
                    Variant::new(SemanticVersion::new(1,2,3, ReleaseType::Release), 0),
                    Variant::new(SemanticVersion::new(2,2,2, ReleaseType::Release), 4)
                )
            )
        );

        assert_eq!(result.unwrap(), expected);
    }

    #[test]
    fn given_one_implicit_and_one_explicit_variant_separated_by_dotdot_succeeds() {
        let result = parse_package_variants_range("maya-1.2.3..2.2.2@4");
        let expected = (
            "",
            (
                "maya",
                Range::between(
                    Variant::new(SemanticVersion::new(1,2,3, ReleaseType::Release), 0),
                    Variant::new(SemanticVersion::new(2,2,2, ReleaseType::Release), 4)
                )
            )
        );

        assert_eq!(result.unwrap(), expected);
    }

    #[test]
    fn given_two_explicit_variants_separated_by_pgt_succeeds() {
        let result = parse_package_variants_range("maya-1.2.3@2+<2.2.2@4");
        let expected = (
            "",
            (
                "maya",
                Range::between(
                    Variant::new(SemanticVersion::new(1,2,3, ReleaseType::Release), 2),
                    Variant::new(SemanticVersion::new(2,2,2, ReleaseType::Release), 4)
                )
            )
        );

        assert_eq!(result.unwrap(), expected);
    }

    #[test]
    fn given_two_explicit_variants_separated_by_dotdot__succeeds() {
        let result = parse_package_variants_range("maya-1.2.3@2..2.2.2@4");
        let expected = (
            "",
            (
                "maya",
                Range::between(
                    Variant::new(SemanticVersion::new(1,2,3, ReleaseType::Release), 2),
                    Variant::new(SemanticVersion::new(2,2,2, ReleaseType::Release), 4)
        
                )
            )
        );

        assert_eq!(result.unwrap(), expected);
    }

    #[test]
    fn given_explicit_variant_starting_with_carrot__succeeds() {
        let result = parse_package_variants_range("maya-^1.2.3@1");
        let expected = (
            "",
            (
                "maya",
                Range::between(
                    Variant::new(SemanticVersion::new(1,2,3, ReleaseType::Release), 1),
                    Variant::new(SemanticVersion::new(1,2,4, ReleaseType::Release),1)
                )
            )
        );
        assert_eq!(result.unwrap(), expected);

    }

    
    
    #[test]
    fn given_implicit_variant_starting_with_carrot__succeeds() {
        let result = parse_package_variants_range("maya-^1.2.3");
        let expected = (
            "",
            (
                "maya",
                Range::between(
                    Variant::new(SemanticVersion::new(1,2,3, ReleaseType::Release), 0),
                    Variant::new(SemanticVersion::new(1,2,4, ReleaseType::Release), constants::MAX_VARIANTS)
                )
            )
        );
        assert_eq!(result.unwrap(), expected);
    }
    
    #[test]
    fn given_explicit_variant_with_two_digit_semver_starting_with_carrot__succeeds() {
        let result = parse_package_variants_range("maya-^1.2@1");
        let expected = (
            "",
            (
                "maya",
                Range::between(
                    Variant::new(SemanticVersion::new(1,2,0, ReleaseType::Release), 1),
                    Variant::new(SemanticVersion::new(1,3,0, ReleaseType::Release),1)
                )
            )
        );
        assert_eq!(result.unwrap(), expected);
    }

    #[test]
    fn given_implicit_variant_with_two_digit_semver_starting_with_carrot__succeeds() {
        let result = parse_package_variants_range("maya-^1.2");
        let expected = (
            "",
            (
                "maya",
                Range::between(
                    Variant::new(SemanticVersion::new(1,2,0, ReleaseType::Release), 0),
                    Variant::new(SemanticVersion::new(1,3,0, ReleaseType::Release), constants::MAX_VARIANTS)
                )
            )
        );
        assert_eq!(result.unwrap(), expected);
    }
    
    #[test]
    fn given_explicit_variant_with_one_digit_semver_starting_with_carrot__succeeds() {
        let result = parse_package_variants_range("maya-^1@1");
        let expected = (
            "",
            (
                "maya",
                Range::between(
                    Variant::new(SemanticVersion::new(1,0,0, ReleaseType::Release), 1),
                    Variant::new(SemanticVersion::new(2,0,0, ReleaseType::Release),1)
                )
            )
        );
        assert_eq!(result.unwrap(), expected);
    }

    #[test]
    fn given_implicit_variant_with_one_digit_semver_starting_with_carrot__succeeds() {
        let result = parse_package_variants_range("maya-^1");
        let expected = (
            "",
            (
                "maya",
                Range::between(
                    Variant::new(SemanticVersion::new(1,0,0, ReleaseType::Release), 0),
                    Variant::new(SemanticVersion::new(2,0,0, ReleaseType::Release), constants::MAX_VARIANTS)
                )
            )
        );
        assert_eq!(result.unwrap(), expected);
    }
}