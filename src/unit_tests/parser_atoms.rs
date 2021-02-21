#![allow(non_snake_case)]

use super::*;
use nom::combinator::complete;
//use nom::Err;
use nom::error::ErrorKind;
use nom::error::Error as NomError;
use nom::Err::Error as NomErr;
use crate::error::PesNomError;

mod alphaword {
    use super::*;

    #[test]
    fn given_word_starting_with_letter_can_parse() {
        fn parser(input: &str) -> PNResult<&str, &str> {
            complete(alphaword)(input)
        }
        let result = parser("a13b");
        assert_eq!(result, Ok(("", "a13b")));
    }

    #[test]
    fn given_word_starting_with_number_fails_to_parse() {
        fn parser(input: &str) -> PNResult<&str, &str> {
            complete(alphaword)(input)
        }
        let result = parser("1abc");
        //assert_eq!(result, Err(NomErr(NomError{input: "1abc", code: ErrorKind::Alpha})));
        assert_eq!(result, Err(NomErr(PesNomError::Nom("1abc", ErrorKind::Alpha))));
    }
}

mod underscore {
    use super::*;

    // Here we are testing that the parser for an underscore followed
    // by a word (a-zA-Z0-9) can parse
    #[test]
    fn given_input_starting_with_number_can_parse() {
        fn parser(input: &str) -> PNResult<&str, &str> {
            complete(underscore_word)(input)
        }
        let result = parser("_1fofo");
        assert_eq!(result, Ok(("", "_1fofo")));
    }

    #[test]
    fn given_input_starting_with_letter_can_parse() {
        fn parser(input: &str) -> PNResult<&str, &str> {
            complete(underscore_word)(input)
        }
        let result = parser("_fofo");
        assert_eq!(result, Ok(("", "_fofo")));
    }
}

mod alphaword_many0_underscore_word {
    use super::*;

    // test that the parser which takes a word starting with a letter followed by
    // zero or more words separated by single underscores can parse. Note that other
    // than the first word, we do not care if subsequent words start with a number or
    // letter.
    #[test]
    fn given_input_starting_with_num_can_parse() {
        fn parser(input: &str) -> PNResult<&str, &str> {
            complete(alphaword_many0_underscore_word)(input)
        }
        let result = parser("dude_123_1fofo");
        assert_eq!(result, Ok(("", "dude_123_1fofo")));
    }
}

mod space0_eol {
    use super::*;

    #[test]
    fn given_spaces() {
        let result = space0_eol("     a");
        assert_eq!(result, Ok(("a", "     ")));
    }

    #[test]
    fn given_spaces_and_newline() {
        let input = r#"        
"#;
        let result = space0_eol(input);
        assert_eq!(result, Ok(("", "        \n")));
    }

    #[test]
    fn given_hash_and_stuff() {
        let result = space0_eol("# oh boy");
        assert_eq!(result, Ok(("", "# oh boy")));
    }

    #[test]
    fn given_hash_stuff_and_newline() {
        let input = r#"# this is junk 
"#;
        let result = space0_eol(input);
        assert_eq!(result, Ok(("", "# this is junk \n")));
    }
}