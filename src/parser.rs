//! Provides parsers to convert from str to SemanticVersion and 
//! Range<SemanticVersion>

use std::path::PathBuf;
use std::rc::Rc;

use pubgrub::version::SemanticVersion;
use pubgrub::range::Range;

use nom::{
    //IResult, 
    branch::alt,
    bytes::complete::tag,
    character::complete::digit1,
    combinator::{all_consuming, recognize},
    multi::{many_m_n, many0, many1, separated_list0},
    sequence::{
        delimited,
        pair,
        preceded,
        separated_pair,
        terminated,
        tuple,
    },
};

use crate::error::{PNResult, PesNomError, PNCompleteResult, PesError};
use crate::env::{PathToken, PathMode};
use crate::parser_atoms::{alphaword_many0_underscore_word, ws};
pub use crate::traits::VarProvider;
pub use crate::env::BasicVarProvider;


//------------------//
// PUBLIC FUNCTIONS //
//------------------//


/// Given an Rc wrapped provider, parse the paths from a string
pub fn parse_all_paths_with_provider<'a>(provider: Rc<BasicVarProvider>) 
    -> impl Fn(&'a str) -> PNResult<&'a str, PathMode> 
{
    //let provider = provider.clone();
    move |s: &'a str| {
        alt((
            parse_append_paths_with_provider(provider.clone()), 
            parse_prepend_paths_with_provider(provider.clone()),
            parse_exact_paths_with_provider(provider.clone())
        ))(s)
    }
}

/// provide a VarProvider and the sring to parse, and get back a PathMode or Error
pub fn parse_consuming_all_paths_with_provider<'a>(provider: Rc<BasicVarProvider>, s: &'a str) 
    //-> PNResult<&'a str, PathMode> 
    -> PNCompleteResult<&'a str, PathMode>
{
    let (_, result) = all_consuming(
        ws( // drop surrounding whitespace
            parse_all_paths_with_provider(provider.clone())
        )
    )(s)?;
    Ok(result)

}

/// Given a string representing a semantic version range - return a Range of SemanticVersion
/// 
/// # Example
/// ```
/// # use pes::parser::parse_semver_range;
/// # use pubgrub::{version::SemanticVersion, range::Range};
/// # fn main()  {
/// let range = parse_semver_range("1.2.3+<3.0.0");
/// assert_eq!(range, Ok(("",Range::between(SemanticVersion::new(1,2,3), SemanticVersion::new(3,0,0)))));
/// # }
/// ```
pub fn parse_semver_range(s: &str) -> PNResult<&str, Range<SemanticVersion>> {
    // delimited( 
    //     space0,
        alt((parse_semver_carrot, parse_semver_between, parse_semver_exact)) //,
        // space0
    // )
    (s)
}
/// Wraps ```parse_semver_range```, ensuring that it completely consumes the input and 
/// simplifies the return signature. Failure to completely consume the input results in an error.
pub fn parse_consuming_semver_range(s: &str) 
    -> Result<Range<SemanticVersion>, PesError>     
{
    let result = all_consuming(
        ws(
            parse_semver_range
        )
    )(s).map_err(|_| PesError::ParsingFailure(s.into()))?;
    let (_, result) = result;
    Ok(result)
}


/// Given a string like this: <package name>-<semver> (eg internal-1.2.3), return a 
/// tuple of package name, SemanticVersion.
///
/// # Example
/// ```
/// # use pes::parser::parse_package_version;
/// # use pubgrub::{version::SemanticVersion, range::Range};
/// # fn main()  {
/// let range = parse_package_version("maya-1.2.3");
/// assert_eq!(range, Ok(("",("maya", SemanticVersion::new(1,2,3)))));
/// # }
/// ```
pub fn parse_package_version(input: &str) -> PNResult<&str, (&str, SemanticVersion)> {
    separated_pair(alphaword_many0_underscore_word, tag("-"), parse_semver)(input)

}

/// Given an input str representing a named package and version range separated by a dash,
/// parse and return the package name and a semantic version range. 
/// The Range may be either an Exact range or a Range between two SemanticVersion instances.
///
/// # Example Inputs
/// - maya-1.2.3+<4 
/// - maya-^3.2
///
/// # Example
/// ```
/// # use pes::parser::parse_package_range;
/// # use pubgrub::{version::SemanticVersion, range::Range};
/// # fn main()  {
/// let range = parse_package_range("maya-1.2.3+<3");
/// assert_eq!(range, Ok(("",("maya", Range::between(SemanticVersion::new(1,2,3), SemanticVersion::new(3,0,0))))));
/// # }
/// ```
pub fn parse_package_range(input: &str) -> PNResult<&str, (&str, Range<SemanticVersion>)> {
    alt((separated_pair(alphaword_many0_underscore_word, tag("-"), parse_semver_range), parse_package_any))(input)
}

/// Wraps ```parse_package_range```, ensuring that the wrapped parser completely consumes the input, with the 
/// bonus of simplifying the return signature
pub fn parse_consuming_package_range(input: &str) -> Result<(&str, Range<SemanticVersion>), PesError> {
    let (_,result) = all_consuming(parse_package_range)(input).map_err(|e| PesError::ParsingFailure(format!("{:?}", e)))?;
    Ok(result)
}

/// Wraps ```parse_semver```, ensuring that it completely consumes the input, and simplifies the 
/// return signature. Failure to consume the input results in an error.
pub fn parse_consuming_semver(input: &str) -> Result<SemanticVersion, PesError> {
    let result = all_consuming(parse_semver)(input).map_err(|_| PesError::ParsingFailure(input.into()))?;
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


// Given a string that represents a semantic version, that is an unsigned int,  followed by 
// zero to two period delimited unsigned ints, return a SemanticVersion instance
fn parse_semver(s: &str) -> PNResult<&str, SemanticVersion> {
    let (leftover,(first, rest)) = tuple((digit1, many_m_n(0, 2, preceded(tag("."), digit1))))(s)?;
    let semver = SemanticVersion::new(
        first.parse::<u32>().unwrap(),
        rest.get(0).unwrap_or_else(|| &"0").parse::<u32>().unwrap(),
        rest.get(1).unwrap_or_else(|| &"0").parse::<u32>().unwrap()
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
    let minor =  rest.get(0).unwrap_or_else(|| &"0").parse::<u32>().unwrap();
    let patch =  rest.get(1).unwrap_or_else(|| &"0").parse::<u32>().unwrap();

    let semver = SemanticVersion::new(
        major,
        minor,
        patch,
    );
   
    let semver2 = match rest.len() {
        0 => SemanticVersion::new(major+1, 0, 0),
        1 => SemanticVersion::new(major, minor+1, 0),
        2 => SemanticVersion::new(major, minor, patch+1),
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
/*
fn parse_prepend<'a>(s: &'a str) -> PNResult<&str, PathToken<'a>> {
    let (leftover, _prepend) = tag(":@")(s)?;
    Ok((leftover, PathToken::Prepend))
}

fn parse_append<'a>(s: &'a str) -> PNResult<&str, PathToken<'a>> {
    let (leftover, _append) = tag("@:")(s)?;
    Ok((leftover, PathToken::Append))
}

fn parse_rootvar<'a>(s: &'a str) -> PNResult<&str, PathToken<'a>> {
    let (leftover, _variable) = delimited(tag("{"),tag("root"), tag("}"))(s)?;
    Ok((leftover, PathToken::RootVar))
}
*/
#[allow(dead_code)] // not really dead code. it is used in a subparser
fn parse_var<'a>(s: &'a str) -> PNResult<&str, PathToken<'a>> {
    let (leftover, variable) = delimited(tag("{"),alphaword_many0_underscore_word, tag("}"))(s)?;
    Ok((leftover, PathToken::Variable(variable)))
}
/*
fn parse_separator<'a>(s: &'a str) -> PNResult<&str, PathToken<'a>> {
    let (leftover,_tag) = tag(":")(s)?;
    Ok((leftover, PathToken::Separator))
}
*/

// just found nom::comnbinator::recognize. No need to do this on my own
/*
fn parse_relpath<'a>(s: &'a str) -> PNResult<&str, PathToken<'a>> {
    let (leftover, relpath) = pair(alphaword_many0_underscore_word,many0(alt((tag("/"), alphaword_many0_underscore_word))))(s)?;
    let  sz = relpath.0.len();
    let sz = relpath.1.iter().fold(sz, |sum, val| sum + val.len());
    let (result,_) = s.split_at(sz);
    Ok((leftover, PathToken::relpath(result)))
}
*/
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

/*
fn parse_abspath<'a>(s: &'a str) -> PNResult<&str, PathToken<'a>> {
    let (leftover, abspath) = pair(tag("/"),many0(alt((tag("/"), alphaword_many0_underscore_word))))(s)?;
    let  sz = abspath.0.len();
    let sz = abspath.1.iter().fold(sz, |sum, val| sum + val.len());
    let (result,_) = s.split_at(sz);
    Ok((leftover, PathToken::abspath(result)))
}
*/

fn parse_abspath<'a>(s: &'a str) -> PNResult<&str, PathToken<'a>> {
    let (leftover, abspath) = recognize(pair(tag("/"),many0(alt((tag("/"), alphaword_many0_underscore_word)))))(s)?;
    Ok((leftover, PathToken::abspath(abspath)))
}

// fn parse_path<'a>(s: &'a str) -> PNResult<&str, Vec<PathToken<'a>>> {
//     many1(alt((parse_abspath, parse_relpath, parse_var)))(s)
// }


// fn parse_paths<'a>(s: &'a str) -> PNResult<&str, Vec<Vec<PathToken<'a>>>> {
//     separated_list0(tag(":"), parse_path)(s)
// }



fn parse_var_with_provider<'a>(provider: Rc<BasicVarProvider>) 
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
        let result = provider.get(variable).ok_or_else(|| PesNomError::<&str>::InvalidKey(variable.to_string()))?;
        Ok((leftover, PathToken::OwnedVariable(result.to_string())))
    }
}

// given a provider to resolve path variables, 
fn parse_path_with_provider<'a>(provider: Rc<BasicVarProvider>) -> impl Fn(&'a str) -> PNResult<&'a str, PathBuf> {
    let provider = provider.clone();
    move |s: &'a str| {
        let (leftover, path_tokens) = many1(alt((parse_abspath, parse_relpath, parse_var_with_provider(provider.clone()))))(s)?;
        let mut retpath = PathBuf::new();
        
        for token in path_tokens {
            match token {
                PathToken::Relpath(path) => retpath.push(path),
                PathToken::OwnedVariable(ref var) => retpath.push(var),
                PathToken::RootVar => {
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
fn parse_paths_with_provider<'a>(provider: Rc<BasicVarProvider>) -> impl Fn(&'a str) -> PNResult<&'a str, Vec<PathBuf>> {
    //let provider = provider.clone();
    move |s: &'a str| {
        separated_list0(tag(":"), parse_path_with_provider(provider.clone()))(s)
    }
}

// given a provider to resolve path variables, 
fn parse_append_paths_with_provider<'a>(provider: Rc<BasicVarProvider>) -> impl Fn(&'a str) -> PNResult<&'a str, PathMode> {
    //let provider = provider.clone();
    move |s: &'a str| {
        let (leftover, result) = preceded(tag("@:"), parse_paths_with_provider(provider.clone()))(s)?;
        Ok((leftover, PathMode::Append(result)))
    }
}

// given a provider to resolve path variables, 
fn parse_prepend_paths_with_provider<'a>(provider: Rc<BasicVarProvider>) -> impl Fn(&'a str) -> PNResult<&'a str, PathMode> {
    //let provider = provider.clone();
    move |s: &'a str| {
        let (leftover, result) = terminated( parse_paths_with_provider(provider.clone()),tag(":@"))(s)?;
        Ok((leftover, PathMode::Prepend(result)))
    }
}

fn parse_exact_paths_with_provider<'a>(provider: Rc<BasicVarProvider>) -> impl Fn(&'a str) -> PNResult<&'a str, PathMode> {
    //let provider = provider.clone();
    move |s: &'a str| {
        let (leftover, result) =  parse_paths_with_provider(provider.clone())(s)?;
        Ok((leftover, PathMode::Exact(result)))
    }
}


#[cfg(test)]
#[path = "./unit_tests/parser.rs"]
mod unit_tests;