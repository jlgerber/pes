
#![allow(non_snake_case)]

use super::*;

mod PathToken_test {
    use super::*;
        
    #[test]
    fn variable__given_input__succeeds() {
        let value = PathToken::variable("foo");
        assert_eq!(value, PathToken::Variable("foo"));
    }


    #[test]
    fn owned_variable__given_str__succeeds() {
        let value = PathToken::owned_variable("foo");
        assert_eq!(value, PathToken::OwnedVariable("foo".to_string()));
    }

    #[test]
    fn owned_variable__given_string__succeeds() {
        let value = PathToken::owned_variable("foo".to_string());
        assert_eq!(value, PathToken::OwnedVariable("foo".to_string()));
    }


    #[test]
    fn path__given_relpath__succeeds() {
        let value = PathToken::path("foo/bar");
        assert_eq!(value, PathToken::Relpath(&Path::new("foo/bar")));
    }


    #[test]
    fn path__given_abspath__succeeds() {
        let value = PathToken::path("/foo/bar");
        assert_eq!(value, PathToken::Abspath(&Path::new("/foo/bar")));
    }
}

mod BasicVarProvider_test {
    use super::*;

    #[test]
    fn insert__given_new_key_value__returns_None() {
        let mut provider = BasicVarProvider::new();
        let old_value = provider.insert("foo", "bar");
        assert_eq!(old_value, None);
    }

    #[test]
    fn insert__given_new_key_value__returns_Some() {
        let mut provider = BasicVarProvider::new();
        provider.insert("foo", "blue");
        let old_value = provider.insert("foo", "bar");
        assert_eq!(old_value, Some("blue".to_string()));
    }

    #[test]
    fn get__given_extant_key__returns_Some_result() {
        let mut provider = BasicVarProvider::new();
        provider.insert("foo", "bar");
        let result = provider.get("foo");
        assert_eq!(result, Some("bar"));  
    }

    #[test]
    fn get__given_non_extant_key__returns_None() {
        let mut provider = BasicVarProvider::new();
        provider.insert("foo", "bar");
        let result = provider.get("MISSING");
        assert_eq!(result, None);  
    }

    #[test]
    fn insert_env_var__given_existing_var__returns_Ok() {
        let mut provider = BasicVarProvider::new();
        std::env::set_var("FOO", "definitely_bar");
        let result = provider.insert_env_var("FOO");
        assert!(result.is_ok());
       
        let result = provider.get("FOO");
        assert_eq!(result, Some("definitely_bar"));  
    }

    #[test]
    fn insert_env_var__given_non_extant_env_var__returns_PesError() {
        let mut provider = BasicVarProvider::new();
        let result = provider.insert_env_var("NOT_SET_FOR_SURE");
        assert!(result.is_err());
       
        let result = provider.get("NOT_SET_FOR_SURE");
        assert_eq!(result, None);  
    }
}