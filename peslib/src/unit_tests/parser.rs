
#![allow(non_snake_case)]

use super::*;

use nom::error::ErrorKind;
//use nom::error::Error as NomError;
use crate::error::PesNomError;
mod semver_parsing {
    use super::*;
    use nom::Err::Error as NomErr;
    
    //TODO: add tests for consuming variants

    #[test]
    fn parse_semver_goodinput() {
        let result = parse_semver("1.2.3");
        assert_eq!(result, Ok(("",SemanticVersion::new(1,2,3))));
    }

    #[test]
    fn parse_semver_between_given_goodinput() {
        let result = parse_semver_between("1.2.3+<3.4.5");
        assert_eq!(result, Ok(("", Range::between(SemanticVersion::new(1,2,3), SemanticVersion::new(3,4,5)))));
    }

    #[test]
    fn parse_semver_between_given_badinput() {
        let result = parse_semver_between("1.2.3<3.4.5");
        //assert_eq!(result, Err(NomErr(PesNomError::Nom(NomError { input: "<3.4.5", code: ErrorKind::Tag }))));
        assert_eq!(result, Err(NomErr(PesNomError::Nom( "<3.4.5", ErrorKind::Tag ))));
    }

    #[test]
    fn parse_range_with_spaces() {
        let result = parse_semver_between("1.2.3 +< 3.4.5");
        assert_eq!(result, Ok(("", Range::between(SemanticVersion::new(1,2,3), SemanticVersion::new(3,4,5)))));
    }

    #[test]
    fn parse_str_starting_with_carot_major() {
        let result = parse_semver_carrot("^1");
        assert_eq!(result, Ok(("", Range::between(SemanticVersion::new(1,0,0), SemanticVersion::new(2,0,0)))));
    }

    #[test]
    fn parse_str_starting_with_carot_minor() {
        let result = parse_semver_carrot("^2.5");
        assert_eq!(result, Ok(("", Range::between(SemanticVersion::new(2,5,0), SemanticVersion::new(2,6,0)))));
    }

    #[test]
    fn parse_str_starting_with_carot_path() {
        let result = parse_semver_carrot("^3.4.2");
        assert_eq!(result, Ok(("", Range::between(SemanticVersion::new(3,4,2), SemanticVersion::new(3,4,3)))));
    }

    #[test]
    fn parse_str_exact() {
        let result = parse_semver_exact("1.2.3");
        assert_eq!(result, Ok(("", Range::exact(SemanticVersion::new(1,2,3)))));
    }

    #[test]
    fn parse_semver_from_table() {
        let versions = vec![
            //("   1.23.4   ", Ok(("", Range::exact(SemanticVersion::new(1,23,4))))) ,
            ("1.23.4", Ok(("", Range::exact(SemanticVersion::new(1,23,4))))) ,
            // (" 1.2.3 +< 3 ", Ok(("", Range::between(SemanticVersion::new(1,2,3), SemanticVersion::new(3,0,0))))),
            ("1.2.3+<3", Ok(("", Range::between(SemanticVersion::new(1,2,3), SemanticVersion::new(3,0,0))))),
            ("^2.2", Ok(("", Range::between(SemanticVersion::new(2,2,0), SemanticVersion::new(2,3,0) ))))
        ];

        for (input,expected) in versions {
            assert_eq!(parse_semver_range(input), expected);
        }
    }

    #[test]
    fn parse_package_name_and_version() {
        let result = parse_package_version("maya-1.2.3");
        assert_eq!(result, Ok(("", ("maya", SemanticVersion::new(1,2,3)))));
    }

    #[test]
    fn parse_package_name_and_range() {
        let versions = vec![
            ("maya-1.23.4", Ok(("", ("maya",Range::exact(SemanticVersion::new(1,23,4)))))) ,
            ("houdini-1.2.3+<3", Ok(("",("houdini", Range::between(SemanticVersion::new(1,2,3), SemanticVersion::new(3,0,0)))))),
            ("houdini-1.2.3 +< 3.0.0", Ok(("",("houdini", Range::between(SemanticVersion::new(1,2,3), SemanticVersion::new(3,0,0)))))),
            ("nuke-^2.2", Ok(("", ("nuke", Range::between(SemanticVersion::new(2,2,0), SemanticVersion::new(2,3,0) )))))
        ];
        for (input, expected) in versions {
            let result = parse_package_range(input);
            assert_eq!(result, expected);
        
        }
    }

}
//-----------------//
// ENV PARSE TESTS //
//-----------------//

mod env_parsing {
    use super::*;

    // parse_var
    #[test]
    fn parse_var__given_appropriate_str__succeeds() {
        let result = parse_var("{othervar}/foo/bar").unwrap();
        assert_eq!(result.0, "/foo/bar");
        assert_eq!(result.1, PathToken::Variable("othervar".into()));
    }
    
    // parse_relpath
    #[test]
    fn parse_relpath__given_relpath__succeeds() {
        let result = parse_relpath("foo/bar/bla:").unwrap();
        assert_eq!(result.0, ":");
        assert_eq!(result.1, PathToken::relpath("foo/bar/bla"));
    }


    #[test]
    fn parse_relpath__given_abspath__fails() {
        let result = parse_relpath("/foo/bar/bla:");
        assert!(result.is_err());
    }

    // parse_abspath
    #[test]
    fn parse_abspath__given_relpath__succeeds() {
        let result = parse_abspath("/foo/bar/bla:").unwrap();
        assert_eq!(result.0, ":");
        assert_eq!(result.1, PathToken::abspath("/foo/bar/bla"));
    }
}


mod BasicVarProvider_test {
    use super::*;
    //use nom::Err::Error as NomErr;
    use std::rc::Rc;
    
    #[test]
    fn parse_var_with_provider__given_known_var() {
        let mut provider = BasicVarProvider::new();
        provider.insert("root", "foobar");
        let provider = Rc::new(provider);
        let result = parse_var_with_provider(provider)("{root}").unwrap();
        assert_eq!(result.0, "");
        assert_eq!(result.1, PathToken::OwnedVariable("foobar".into()));

    }

    #[test]
    fn parse_path_with_provider__given_valid_path__succeeds() {
        let mut provider = BasicVarProvider::new();
        provider.insert("root", "foobar");
        provider.insert("name", "fred");
        let provider = Rc::new(provider);
        let result = parse_path_with_provider(provider)("/packages/{root}/stuff/{name}").unwrap();
        let expected = PathBuf::from("/packages/foobar/stuff/fred");
        assert_eq!(result.0, "");
        assert_eq!(result.1, expected);
    }

    #[test]
    fn parse_paths_with_provider__given_valid_path__succeeds() {
        let mut provider = BasicVarProvider::new();
        provider.insert("root", "foobar");
        provider.insert("name", "fred");
        let provider = Rc::new(provider);
        let result = parse_paths_with_provider(provider)("/packages/{root}/stuff/{name}:/foo/bar/bla").unwrap();
        assert_eq!(result.0, "");
        assert_eq!(result.1, vec![
            PathBuf::from("/packages/foobar/stuff/fred"),
            PathBuf::from("/foo/bar/bla")
        ]);
    }

    #[test]
    fn parse_append_paths_with_provider__given_valid_path__succeeds() {
        let mut provider = BasicVarProvider::new();
        provider.insert("root", "foobar");
        provider.insert("name", "fred");
        let provider = Rc::new(provider);
        let result = parse_append_paths_with_provider(provider)("@:/packages/{root}/stuff/{name}:/foo/bar/bla").unwrap();
        assert_eq!(result.0, "");
        assert_eq!(result.1, PathMode::Append(vec![
            PathBuf::from("/packages/foobar/stuff/fred"),
            PathBuf::from("/foo/bar/bla")
        ]));
    }

    #[test]
    fn parse_prepend_paths_with_provider__given_valid_path__succeeds() {
        let mut provider = BasicVarProvider::new();
        provider.insert("root", "foobar");
        provider.insert("name", "fred");
        let provider = Rc::new(provider);
        let result = parse_prepend_paths_with_provider(provider)("/packages/{root}/stuff/{name}:/foo/bar/bla:@").unwrap();
        assert_eq!(result.0, "");
        assert_eq!(result.1, PathMode::Prepend(vec![
            PathBuf::from("/packages/foobar/stuff/fred"),
            PathBuf::from("/foo/bar/bla")
        ]));
    }


    #[test]
    fn parse_exact_paths_with_provider__given_valid_path__succeeds() {
        let mut provider = BasicVarProvider::new();
        provider.insert("root", "foobar");
        provider.insert("name", "fred");
        let provider = Rc::new(provider);
        let result = parse_exact_paths_with_provider(provider)("/packages/{root}/stuff/{name}:/foo/bar/bla").unwrap();
        assert_eq!(result.0, "");
        assert_eq!(result.1, PathMode::Exact(vec![
            PathBuf::from("/packages/foobar/stuff/fred"),
            PathBuf::from("/foo/bar/bla")
        ]));
    }


    #[test]
    fn parse_all_paths_with_provider__given_valid_path__succeeds() {
        let mut provider = BasicVarProvider::new();
        provider.insert("root", "foobar");
        provider.insert("name", "fred");
        let provider = Rc::new(provider);

        let result = parse_all_paths_with_provider(provider.clone())("/packages/{root}/stuff/{name}:/foo/bar/bla").unwrap();
        assert_eq!(result.0, "");
        assert_eq!(result.1, PathMode::Exact(vec![
            PathBuf::from("/packages/foobar/stuff/fred"),
            PathBuf::from("/foo/bar/bla")
        ]));

        let result = parse_all_paths_with_provider(provider.clone())("/packages/{root}/stuff/{name}:/foo/bar/bla:@").unwrap();
        assert_eq!(result.0, "");
        assert_eq!(result.1, PathMode::Prepend(vec![
            PathBuf::from("/packages/foobar/stuff/fred"),
            PathBuf::from("/foo/bar/bla")
        ]));

        let result = parse_all_paths_with_provider(provider)("@:/packages/{root}/stuff/{name}:/foo/bar/bla").unwrap();
        assert_eq!(result.0, "");
        assert_eq!(result.1, PathMode::Append(vec![
            PathBuf::from("/packages/foobar/stuff/fred"),
            PathBuf::from("/foo/bar/bla")
        ]));
    }


    #[test]
    fn parse_consuming_all_paths_with_provider__given_valid_path__succeeds() {
        let mut provider = BasicVarProvider::new();
        provider.insert("root", "foobar");
        provider.insert("name", "fred");
        let provider = Rc::new(provider);
        // note that we introduce whitespace in front and behind to verify that the `ws` parser is working
        let result = parse_consuming_all_paths_with_provider(provider.clone(), " /packages/{root}/stuff/{name}:/foo/bar/bla ").unwrap();
        assert_eq!(result, PathMode::Exact(vec![
            PathBuf::from("/packages/foobar/stuff/fred"),
            PathBuf::from("/foo/bar/bla")
        ]));

        let result = parse_consuming_all_paths_with_provider(provider.clone(), " /packages/{root}/stuff/{name}:/foo/bar/bla:@ ").unwrap();
        assert_eq!(result, PathMode::Prepend(vec![
            PathBuf::from("/packages/foobar/stuff/fred"),
            PathBuf::from("/foo/bar/bla")
        ]));

        let result = parse_consuming_all_paths_with_provider(provider, " @:/packages/{root}/stuff/{name}:/foo/bar/bla ").unwrap();
        assert_eq!(result, PathMode::Append(vec![
            PathBuf::from("/packages/foobar/stuff/fred"),
            PathBuf::from("/foo/bar/bla")
        ]));
    }

    // verify that the consuming version of the parser will error if provided with additional data
    #[test]
    fn parse_consuming_all_paths_with_provider__given_invalid_path__succeeds() {
        let mut provider = BasicVarProvider::new();
        provider.insert("root", "foobar");
        provider.insert("name", "fred");
        let provider = Rc::new(provider);

        let result = parse_consuming_all_paths_with_provider(provider.clone(), "/packages/{root}/stuff/{name}:/foo/bar/bla other stuff");
        assert!(result.is_err());
        
        let result = parse_consuming_all_paths_with_provider(provider.clone(), "/packages/{root}/stuff/{name}:/foo/bar/bla:@ bla");
        assert!(result.is_err());


        let result = parse_consuming_all_paths_with_provider(provider, "@:/packages/{root}/stuff/{name}:/foo/bar/bla   bla");
        assert!(result.is_err());

    }
}