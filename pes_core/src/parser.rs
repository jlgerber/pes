//! Provides parsers for the following use cases:
//! - versions and version ranges
//! - environment path settings, includiing single and multiple paths, as well as special tokens indicating prepending and appending
//!
//! There are two variants of public parsers in the `parser` module - the consuming and non-consuming variants. 
//!
//! - The consuming variant of a parser ensures that the input is completely consumed, eating any surounding whitespace
//! - The non-consuming variant of a parser is simply a `nom` parser, which returns a tuple of the remaining data to be parsed, along with the parse results (assuming a successful parse)
use std::path::PathBuf;
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::str::FromStr;

use pubgrub::{
    range::Range,
};

pub mod package;
pub mod paths;
pub mod semver;
pub mod variant;

pub use paths::*;
pub use package::*;
pub use semver::*;
pub use variant::*;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::digit1,
    combinator::{
        all_consuming, 
        recognize
    },
    multi::{
        many_m_n, 
        many0, many1, separated_list0
    },
    sequence::{
        delimited,
        pair,
        preceded,
        separated_pair,
        //terminated,
        tuple,
    },
};

use crate::{PNResult, PesNomError, PNCompleteResult, PesError, SemanticVersion, variant::Variant, ReleaseType};
use crate::env::{PathToken, PathMode};
use crate::parser_atoms::{alphaword_many0_underscore_word, ws};
pub use crate::traits::VarProvider;
pub use crate::env::BasicVarProvider;


//------------------//
// PUBLIC FUNCTIONS //
//------------------//


//---------------------//
//  PRIVATE FUNCTIONS  //
//---------------------//

// parse a package name with no version specified. In this case, we assume that the 
// version range is open to any version.
fn parse_package_any(s: &str) -> PNResult<&str, (&str, Range<SemanticVersion>)> {
    let (leftover, name) = alphaword_many0_underscore_word(s)?;
    Ok((leftover,(name, Range::<SemanticVersion>::any())))
}

fn parse_prerelease(s: &str) -> PNResult<&str, ReleaseType> {
    let (leftover, release_type) = alt((tag("rc"), tag("releaseCandidate"), tag("release_candidate"), tag("alpha"), tag("beta")))(s)?;
    Ok((leftover, ReleaseType::from_str(release_type)?))
}


fn parse_semver(s: &str) -> PNResult<&str, SemanticVersion> {
    let results = alt((  parse_semver_prerelease, parse_semver_release))(s)?;
    Ok(results)
}

// Given a string that represents a semantic version, that is an unsigned int,  followed by 
// zero to two period delimited unsigned ints, return a SemanticVersion instance
fn parse_semver_release(s: &str) -> PNResult<&str, SemanticVersion> {
    let (leftover,(first, rest)) = tuple((digit1, many_m_n(0, 2, preceded(tag("."), digit1))))(s)?;
    let semver = SemanticVersion::new(
        first.parse::<u32>().unwrap(),
        rest.get(0).unwrap_or(&"0").parse::<u32>().unwrap(),
        rest.get(1).unwrap_or(&"0").parse::<u32>().unwrap(),
        ReleaseType::Release
    );

    Ok((leftover,semver))
}

fn parse_semver_prerelease(s: &str) -> PNResult<&str, SemanticVersion> {
    let (leftover,(first, rest, release_type)) = tuple((digit1, many_m_n(0, 2, preceded(tag("."), digit1)),preceded(tag("-"), parse_prerelease) ))(s)?;
    let semver = SemanticVersion::new(
        first.parse::<u32>().unwrap(),
        rest.get(0).unwrap_or(&"0").parse::<u32>().unwrap(),
        rest.get(1).unwrap_or(&"0").parse::<u32>().unwrap(),
       release_type
    );

    Ok((leftover,semver))
}

// Given a string representing two semantic versions separated by '+<', return a Range::between the first and second
// SemanticVersion instances
fn parse_semver_between(s: &str) -> PNResult<&str, Range<SemanticVersion>> {
    let (leftover, (sm1,sm2)) = separated_pair(parse_semver, delimited(many0(tag(" ")),tag("+<"), many0(tag(" "))), parse_semver)(s)?;
    Ok((leftover, Range::between(sm1, sm2)))
}

// Given a str reference starting with a '^' followed by a valid semantic version str, return a Range::between two 
// SemanticVersions
/* FROM CARGO MANUAL
^1.2.3  :=  >=1.2.3, <2.0.0
^1.2    :=  >=1.2.0, <2.0.0
^1      :=  >=1.0.0, <2.0.0
^0.2.3  :=  >=0.2.3, <0.3.0
^0.2    :=  >=0.2.0, <0.3.0
^0.0.3  :=  >=0.0.3, <0.0.4
^0.0    :=  >=0.0.0, <0.1.0
^0      :=  >=0.0.0, <1.0.0
*/
fn parse_semver_caret(s: &str) -> PNResult<&str, Range<SemanticVersion>> {
    let (leftover,(first, rest)) = preceded(tag("^"), tuple((digit1, many_m_n(0, 2, preceded(tag("."), digit1)))))(s)?;
    let major = first.parse::<u32>().unwrap();
    let minor =  rest.get(0).unwrap_or(&"0").parse::<u32>().unwrap();
    let patch =  rest.get(1).unwrap_or(&"0").parse::<u32>().unwrap();

    let semver = SemanticVersion::new(
        major,
        minor,
        patch,
        ReleaseType::Release
    );
   
    let semver2 = match rest.len() {
        0 => SemanticVersion::new(major+1, 0, 0, ReleaseType::Release),
        1 => {
            if major >= 1 {
                SemanticVersion::new(major+1, 0, 0, ReleaseType::Release)
            } else {
                SemanticVersion::new(major, minor+1, 0, ReleaseType::Release)
            }
        },
        2 => {
            if major >= 1 {
                SemanticVersion::new(major+1, 0, 0, ReleaseType::Release)
            } else if minor == 0{
                // eg ^0.0.3  :=  >=0.0.3, <0.0.4
                SemanticVersion::new(major, minor, patch+1, ReleaseType::Release)
            } else {
                // eg ^0.2.3  :=  >=0.2.3, <0.3.0
                SemanticVersion::new(major, minor + 1, 0, ReleaseType::Release)
            }
        
        },
        _ => panic!("invalid semantic version")
    };

    Ok((leftover, Range::between(semver, semver2)))
}

// Given a str reference representing a semver, return an exact Range over the SemanticVersion
fn parse_semver_exact(s: &str) -> PNResult<&str, Range<SemanticVersion>> {
    let (leftover, semver) = parse_semver(s)?;
    Ok((leftover, Range::exact(semver)))
}

//-----------------//
//   ENV PARSING   //
//-----------------//

#[allow(dead_code)] // not really dead code. it is used in a subparser
fn parse_var<'a>(s: &'a str) -> PNResult<&str, PathToken<'a>> {
    let (leftover, variable) = delimited(tag("{"),alphaword_many0_underscore_word, tag("}"))(s)?;
    Ok((leftover, PathToken::Variable(variable)))
}

fn parse_relpath<'a>(s: &'a str) -> PNResult<&str, PathToken<'a>> {
    let (leftover, relpath) = 
    recognize(
        pair(
            alphaword_many0_underscore_word,
            many0(
                alt(
                    (
                        tag("/"), 
                        alphaword_many0_underscore_word
                    )
                )
            )
        )
    )(s)?;
   
    Ok((leftover, PathToken::relpath(relpath)))
}

fn parse_abspath<'a>(s: &'a str) -> PNResult<&str, PathToken<'a>> {
    let (leftover, abspath) = recognize(pair(tag("/"),many0(alt((tag("/"), alphaword_many0_underscore_word)))))(s)?;
    Ok((leftover, PathToken::abspath(abspath)))
}

fn parse_var_with_provider<'a>(provider: Rc<RefCell<BasicVarProvider>>) 
-> impl Fn(&'a str) -> PNResult<&'a str, PathToken<'a>> {
    let provider = Rc::clone(&provider);
    move |s: &'a str| {
        let (leftover, variable) = 
            delimited(
                tag("{"),
                alphaword_many0_underscore_word, 
                // we eat a slash if it follows the closing brace, as it interferes with subsequent path parsing
                alt((
                    tag("}/"),
                    tag("}")
                ))
            )(s)?;
        let provider = provider.borrow();
        let result = provider.get(variable).ok_or_else(|| PesNomError::<&str>::InvalidKey(variable.to_string()))?;
        Ok((leftover, PathToken::OwnedVariable(result.to_string())))
    }
}

// given a provider to resolve path variables, 
fn parse_path_with_provider<'a>(provider: Rc<RefCell<BasicVarProvider>>) -> impl Fn(&'a str) -> PNResult<&'a str, PathBuf> {
    //let provider = provider.clone();
    move |s: &'a str| {
        let (leftover, path_tokens) = many1(alt((parse_abspath, parse_relpath, parse_var_with_provider(Rc::clone(&provider) ))))(s)?;
        let mut retpath = PathBuf::new();
        
        for token in path_tokens {
            match token {
                PathToken::Relpath(path) => retpath.push(path),
                PathToken::OwnedVariable(ref var) => retpath.push(var),
                PathToken::RootVar => {
                    let provider = provider.borrow();
                    let result = provider.get("root").ok_or_else(|| PesNomError::<&str>::InvalidKey("root".to_string()))?;
                    retpath.push(result)
                },
                PathToken::Abspath(path) => retpath.push(path),
                _ => panic!("Invalid token {:?}",token)
            };
        }
        Ok((leftover, retpath))
    }
}

// given a provider to resolve path variables, 
fn parse_paths_with_provider<'a>(provider: Rc<RefCell<BasicVarProvider>>) -> impl Fn(&'a str) -> PNResult<&'a str, Vec<PathBuf>> {
    move |s: &'a str| {
        separated_list0(tag(":"), parse_path_with_provider(Rc::clone(&provider)))(s)
    }
}

// given a provider to resolve path variables, 
fn parse_append_paths_with_provider<'a>(provider: Rc<RefCell<BasicVarProvider>>) -> impl Fn(&'a str) -> PNResult<&'a str, PathMode> {
    move |s: &'a str| {
        let (leftover, result) = delimited(tag("append("), ws(parse_paths_with_provider(Rc::clone(&provider) )), tag(")"))(s)?;
        //let result = result.display().to_string();
        let result = result.iter().map(|x| x.display().to_string()).collect::<Vec<_>>();

        Ok((leftover, PathMode::Append(VecDeque::from(result))))
    }
}

// given a provider to resolve path variables, 
fn parse_prepend_paths_with_provider<'a>(provider: Rc<RefCell<BasicVarProvider>>) -> impl Fn(&'a str) -> PNResult<&'a str, PathMode> {
    move |s: &'a str| {
        let (leftover, result) = delimited(tag("prepend("), ws(parse_paths_with_provider(Rc::clone(&provider))),tag(")"))(s)?;
        let result = result.iter().map(|x| x.display().to_string()).collect::<Vec<_>>();
        Ok((leftover, PathMode::Prepend(VecDeque::from(result))))
    }
}

fn parse_exact_paths_with_provider<'a>(provider: Rc<RefCell<BasicVarProvider>>) -> impl Fn(&'a str) -> PNResult<&'a str, PathMode> {
    move |s: &'a str| {
        let (leftover, result) =  parse_paths_with_provider(Rc::clone(&provider) )(s)?;
        let result = result.iter().map(|x| x.display().to_string()).collect::<Vec<_>>();

        Ok((leftover, PathMode::Exact(VecDeque::from(result))))
    }
}


#[cfg(test)]
#[path = "./unit_tests/parser.rs"]
mod unit_tests;