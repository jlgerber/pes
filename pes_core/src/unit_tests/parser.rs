#![allow(non_snake_case)]

use super::*;
use nom::error::ErrorKind;
use crate::PesNomError;
use crate::{SemanticVersion, ReleaseType};


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
    use std::cell::RefCell;
    use std::collections::VecDeque;

    #[test]
    fn parse_var_with_provider__given_known_var() {
        let mut provider = BasicVarProvider::new();
        provider.insert("root", "foobar");
        let provider = Rc::new(RefCell::new(provider));
        let result = parse_var_with_provider(provider)("{root}").unwrap();
        assert_eq!(result.0, "");
        assert_eq!(result.1, PathToken::OwnedVariable("foobar".into()));

    }

    #[test]
    fn parse_path_with_provider__given_valid_path__succeeds() {
        let mut provider = BasicVarProvider::new();
        provider.insert("root", "foobar");
        provider.insert("name", "fred");
        let provider = Rc::new(RefCell::new(provider));
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
        let provider = Rc::new(RefCell::new(provider));
        let result = parse_paths_with_provider(provider)("/packages/{root}/stuff/{name}:/foo/bar/bla").unwrap();
        assert_eq!(result.0, "");
        // NOTE: parse_paths_with_provider returns a Result<Vec<PathBuf>,_>, not a Result<VecDeque<PathBuf>>,_> 
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
        let provider = Rc::new(RefCell::new(provider));
        let result = parse_append_paths_with_provider(provider)("append(/packages/{root}/stuff/{name}:/foo/bar/bla)").unwrap();
        assert_eq!(result.0, "");
        assert_eq!(result.1, PathMode::Append(VecDeque::from(vec![
            "/packages/foobar/stuff/fred".to_string(),
            "/foo/bar/bla".to_string()
        ])));
    }

    #[test]
    fn parse_prepend_paths_with_provider__given_valid_path__succeeds() {
        let mut provider = BasicVarProvider::new();
        provider.insert("root", "foobar");
        provider.insert("name", "fred");
        let provider = Rc::new(RefCell::new(provider));
        let result = parse_prepend_paths_with_provider(provider)("prepend(/packages/{root}/stuff/{name}:/foo/bar/bla)").unwrap();
        assert_eq!(result.0, "");
        assert_eq!(result.1, PathMode::Prepend(VecDeque::from(vec![
            "/packages/foobar/stuff/fred".to_string(),
            "/foo/bar/bla".to_string()
        ])));
    }


    #[test]
    fn parse_exact_paths_with_provider__given_valid_path__succeeds() {
        let mut provider = BasicVarProvider::new();
        provider.insert("root", "foobar");
        provider.insert("name", "fred");
        let provider = Rc::new(RefCell::new(provider));
        let result = parse_exact_paths_with_provider(provider)("/packages/{root}/stuff/{name}:/foo/bar/bla").unwrap();
        assert_eq!(result.0, "");
        assert_eq!(result.1, PathMode::Exact(VecDeque::from(vec![
            "/packages/foobar/stuff/fred".to_string(),
           "/foo/bar/bla".to_string()
        ])));
    }


    #[test]
    fn parse_all_paths_with_provider__given_valid_path__succeeds() {
        let mut provider = BasicVarProvider::new();
        provider.insert("root", "foobar");
        provider.insert("name", "fred");
        let provider = Rc::new(RefCell::new(provider));

        let result = parse_all_paths_with_provider(Rc::clone(&provider))("/packages/{root}/stuff/{name}:/foo/bar/bla").unwrap();
        assert_eq!(result.0, "");
        assert_eq!(result.1, PathMode::Exact(VecDeque::from(vec![
            "/packages/foobar/stuff/fred".to_string(),
            "/foo/bar/bla".to_string()
        ])));

        let result = parse_all_paths_with_provider(Rc::clone(&provider))("prepend(/packages/{root}/stuff/{name}:/foo/bar/bla)").unwrap();
        assert_eq!(result.0, "");
        assert_eq!(result.1, PathMode::Prepend(VecDeque::from(vec![
            "/packages/foobar/stuff/fred".to_string(),
            "/foo/bar/bla".to_string()
        ])));

        let result = parse_all_paths_with_provider(provider)("append(/packages/{root}/stuff/{name}:/foo/bar/bla)").unwrap();
        assert_eq!(result.0, "");
        assert_eq!(result.1, PathMode::Append(VecDeque::from(vec![
            "/packages/foobar/stuff/fred".to_string(),
            "/foo/bar/bla".to_string()
        ])));
    }

    #[test]
    fn parse_consuming_all_paths_with_provider__given_valid_path__succeeds() {
        let mut provider = BasicVarProvider::new();
        provider.insert("root", "foobar");
        provider.insert("name", "fred");
        let provider = Rc::new(RefCell::new(provider));
        // note that we introduce whitespace in front and behind to verify that the `ws` parser is working
        let result = parse_consuming_all_paths_with_provider(Rc::clone(&provider), " /packages/{root}/stuff/{name}:/foo/bar/bla ").unwrap();
        assert_eq!(result, PathMode::Exact(VecDeque::from(vec![
            "/packages/foobar/stuff/fred".to_string(),
            "/foo/bar/bla".to_string()
        ])));

        let result = parse_consuming_all_paths_with_provider(Rc::clone(&provider), " prepend( /packages/{root}/stuff/{name}:/foo/bar/bla ) ").unwrap();
        assert_eq!(result, PathMode::Prepend(VecDeque::from(vec![
            "/packages/foobar/stuff/fred".to_string(),
            "/foo/bar/bla".to_string()
        ])));

        let result = parse_consuming_all_paths_with_provider(provider, " append( /packages/{root}/stuff/{name}:/foo/bar/bla ) ").unwrap();
        assert_eq!(result, PathMode::Append(VecDeque::from(vec![
            "/packages/foobar/stuff/fred".to_string(),
            "/foo/bar/bla".to_string()
        ])));
    }

    // verify that the consuming version of the parser will error if provided with additional data
    #[test]
    fn parse_consuming_all_paths_with_provider__given_invalid_path__errors() {
        let mut provider = BasicVarProvider::new();
        provider.insert("root", "foobar");
        provider.insert("name", "fred");
        let provider = Rc::new(RefCell::new(provider));

        let result = parse_consuming_all_paths_with_provider(Rc::clone(&provider), "/packages/{root}/stuff/{name}:/foo/bar/bla other stuff");
        assert!(result.is_err());
        
        let result = parse_consuming_all_paths_with_provider(Rc::clone(&provider), "prepend(/packages/{root}/stuff/{name}:/foo/bar/bla) bla");
        assert!(result.is_err());


        let result = parse_consuming_all_paths_with_provider(provider, "append(/packages/{root}/stuff/{name}:/foo/bar/bla   )bla");
        assert!(result.is_err());

    }
}
