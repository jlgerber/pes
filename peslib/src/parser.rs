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
    //version::SemanticVersion,
};


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

use crate::{PNResult, PesNomError, PNCompleteResult, PesError, SemanticVersion, ReleaseType};
use crate::env::{PathToken, PathMode};
use crate::parser_atoms::{alphaword_many0_underscore_word, ws};
pub use crate::traits::VarProvider;
pub use crate::env::BasicVarProvider;


//------------------//
// PUBLIC FUNCTIONS //
//------------------//


/// Given an Rc wrapped provider, return a parser which parses the paths from a string
///
/// # Example
///
/// ```
/// # use peslib::parser::{BasicVarProvider, parse_all_paths_with_provider};
/// # use peslib::traits::VarProvider;
/// # use peslib::env::PathMode;
/// # use std::rc::Rc;
/// # use std::collections::VecDeque;
/// # use std::cell::RefCell;
/// #
/// # fn main()  {
/// let mut provider = BasicVarProvider::new();
/// provider.insert("root", "foobar");
/// provider.insert("name", "fred");
/// let provider = std::rc::Rc::new(RefCell::new(provider));
/// let result = parse_all_paths_with_provider(Rc::clone(&provider))("/packages/{root}/stuff/{name}:/foo/bar/bla").unwrap();
/// assert_eq!(result.0, "");
/// assert_eq!(result.1, PathMode::Exact(VecDeque::from(vec![
///     "/packages/foobar/stuff/fred".to_string(),
///     "/foo/bar/bla".to_string()
/// ])));
/// # }
// todo: make these generic over VarProvider
pub fn parse_all_paths_with_provider<'a>(provider: Rc<RefCell<BasicVarProvider>>) 
    -> impl Fn(&'a str) -> PNResult<&'a str, PathMode> 
{
    //let provider = provider.clone();
    move |s: &'a str| {
        alt((
            parse_append_paths_with_provider(Rc::clone(&provider)), 
            parse_prepend_paths_with_provider(Rc::clone(&provider)),
            parse_exact_paths_with_provider(Rc::clone(&provider))
        ))(s)
    }
}


/// Given an Rc wrapped BasicVarProvider and a path str, parse the path str, returning a PathMode or error
///
/// # Example
/// ```
/// # use peslib::parser::{BasicVarProvider, parse_consuming_all_paths_with_provider};
/// # use peslib::traits::VarProvider;
/// # use peslib::env::PathMode;
/// # use std::cell::RefCell;
/// # use std::rc::Rc;
/// # use std::collections::VecDeque;
/// # fn main()  {
/// let mut provider = BasicVarProvider::new();
/// provider.insert("root", "foobar");
/// provider.insert("name", "fred");
///
/// let provider = Rc::new(RefCell::new(provider));
///
/// let result = parse_consuming_all_paths_with_provider(
///                     Rc::clone(&provider), 
///                     " /packages/{root}/stuff/{name}:/foo/bar/bla "
///              ).unwrap();
///
/// assert_eq!(result, PathMode::Exact(VecDeque::from(vec![
///     "/packages/foobar/stuff/fred".to_string(),
///     "/foo/bar/bla".to_string()
/// ])));
/// # }
pub fn parse_consuming_all_paths_with_provider(provider: Rc<RefCell<BasicVarProvider>>, s: &str) 
    //-> PNResult<&'a str, PathMode> 
    -> PNCompleteResult<&str, PathMode>
{
    let (_, result) = all_consuming(
        ws( // drop surrounding whitespace
            parse_all_paths_with_provider(provider)
        )
    )(s)?;
    Ok(result)

}


/// Given a string representing a semantic version range - return a Range of SemanticVersion
/// 
/// # Example
/// ```
/// # use peslib::{parser::parse_semver_range, SemanticVersion, ReleaseType};
/// # use pubgrub::range::Range;
/// # fn main()  {
/// let range = parse_semver_range("1.2.3+<3.0.0");
/// assert_eq!(
///     range, 
///     Ok(
///         ("", Range::between(SemanticVersion::new(1,2,3,ReleaseType::Release), SemanticVersion::new(3,0,0,ReleaseType::Release)))
///     )
/// );
/// # }
/// ```
pub fn parse_semver_range(s: &str) -> PNResult<&str, Range<SemanticVersion>> {
        alt((parse_semver_carrot, parse_semver_between, parse_semver_exact)) //,
    (s)
}

/// Given a str representing a semantic version range, return a `Range<SemanticVersion>` or an error
/// Note that unlike a normal `nom` parser, this parser expects to completely consume the inupt. Any remaining
/// is interpreted as an error.
/// Furthermore, note that the parsre consumes any whitespace surounding the version range str.
///
/// # Example
/// ```ignore
///     range.unwrap(), 
///     Range::between(SemanticVersion::new(1,2,3,ReleaseType::Release), SemanticVersion::new(3,0,0,ReleaseType::Release))
/// );
/// # }
/// ```
pub fn parse_consuming_semver_range(s: &str) 
    -> Result<Range<SemanticVersion>, PesError>     
{
    let result = all_consuming(
        ws(
            parse_semver_range
        )
    )(s).map_err(|_| PesError::ParsingFailure(format!("parse_consuming_semver_range {}",s)))?;
    let (_, result) = result;
    Ok(result)
}


/// Given a string like this: <package name>-<semver> (eg internal-1.2.3), return a 
/// tuple of package name, SemanticVersion.
///
/// # Example
/// ```
/// # use peslib::parser::parse_package_version;
/// # use pubgrub::{range::Range};
/// # use peslib::{SemanticVersion, ReleaseType};
/// # fn main()  {
/// let range = parse_package_version("maya-1.2.3");
/// assert_eq!(
///    range, 
///    Ok(
///         ("", ("maya", SemanticVersion::new(1,2,3,ReleaseType::Release)))
///      )
/// );
/// # }
/// ```
pub fn parse_package_version(input: &str) -> PNResult<&str, (&str, SemanticVersion)> {
    separated_pair(alphaword_many0_underscore_word, tag("-"), parse_semver)(input)

}

/// Given a string like this: package-version (eg foo-1.2.3) return a tuple of (package name, SemanticVersion)
pub fn parse_consuming_package_version(input: &str) -> Result <(&str, SemanticVersion), PesError> {
    let (_,result) = 
    all_consuming(
        ws(
            parse_package_version
        )
    )(input).map_err(|e| PesError::ParsingFailure(format!("parse_consuming_package_version {:?}", e)) )?;
    Ok(result)
}

/// Given an input str representing a named package and version range separated by a dash,
/// parse and return the package name and a semantic version range. 
/// The Range may be either an Exact range or a Range between two SemanticVersion instances.
///
/// # Example Inputs
/// - maya-1.2.3+<4 
/// - maya-^3.2
///PathMode
/// assert_eq!(range, Ok(("",("maya", Range::between(SemanticVersion::new(1,2,3,ReleaseType::Release), SemanticVersion::new(3,0,0,ReleaseType::Release))))));
/// # }
/// 
pub fn parse_package_range(input: &str) -> PNResult<&str, (&str, Range<SemanticVersion>)> {
    alt((separated_pair(alphaword_many0_underscore_word, tag("-"), parse_semver_range), parse_package_any))(input)
}

/// Wraps ```parse_package_range```, ensuring that the wrapped parser completely consumes the input, with the 
/// bonus of simplifying the return signature
///
/// # Example
/// ```
/// # use peslib::{parser::parse_consuming_package_range, SemanticVersion, ReleaseType };
/// # use pubgrub:: range::Range;
/// # fn main()  {
/// let range = parse_consuming_package_range("maya-1.2.3+<3");
/// assert_eq!(
///                range.unwrap(), 
///                (
///                   "maya", 
///                    Range::between(SemanticVersion::new(1,2,3,ReleaseType::Release), SemanticVersion::new(3,0,0,ReleaseType::Release))
///                )
///           );
/// # }
/// ```
pub fn parse_consuming_package_range(input: &str) -> Result<(&str, Range<SemanticVersion>), PesError> {
    let (_,result) = 
        all_consuming(
            ws(
                parse_package_range
            )
        )(input).map_err(|e| PesError::ParsingFailure(format!("{:?}", e)) )?;
    Ok(result)
}

/// Wraps ```parse_semver```, ensuring that it completely consumes the input, and simplifies the 
/// return signature. Failure to consume the input results in an error.
pub fn parse_consuming_semver(input: &str) -> Result<SemanticVersion, PesError> {
    let result = all_consuming(ws(parse_semver))(input).map_err(|_| PesError::ParsingFailure(format!("parse_consuming_semver {}",input)))?;
    let (_, result) = result;
    Ok(result)
}


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
fn parse_semver_carrot(s: &str) -> PNResult<&str, Range<SemanticVersion>> {
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
        1 => SemanticVersion::new(major, minor+1, 0, ReleaseType::Release),
        2 => SemanticVersion::new(major, minor, patch+1, ReleaseType::Release),
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