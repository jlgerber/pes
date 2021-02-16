//! what do we want to parse?
//! ^1.2.3
//! 1.2.3+<4.0.0
use pubgrub::version::SemanticVersion;
use pubgrub::range::Range;

use nom::{
    IResult, 
    branch::alt,
    character::complete::digit1,
    bytes::complete::tag,
    sequence::terminated,
    sequence::preceded,
    sequence::tuple,
    sequence::separated_pair,
    multi::many_m_n,
    multi::many0,
    sequence::delimited,
    character::complete::space0,
};

use crate::parser_atoms::alphaword_many0_underscore_word;


// Given a string that represents a semantic version, that is an unsigned int,  followed by 
// zero to two period delimited unsigned ints, return a SemanticVersion instance
fn parse_semver(s: &str) -> IResult<&str, SemanticVersion> {
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
fn parse_semver_between(s: &str) -> IResult<&str, Range<SemanticVersion>> {
    let (leftover, (sm1,sm2)) = separated_pair(parse_semver, delimited(many0(tag(" ")),tag("+<"), many0(tag(" "))), parse_semver)(s)?;
    Ok((leftover, Range::between(sm1, sm2)))
}

// Given a str reference starting with a '^' followed by a valid semantic version str, return a Range::between two 
// SemanticVersions
fn parse_semver_carrot(s: &str) -> IResult<&str, Range<SemanticVersion>> {
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
fn parse_semver_exact(s: &str) -> IResult<&str, Range<SemanticVersion>> {
    let (leftover, semver) = parse_semver(s)?;
    Ok((leftover, Range::exact(semver)))
}

/// Given a string representing a semantic version range - return a Range of SemanticVersion
pub fn parse_semver_range(s: &str) -> IResult<&str, Range<SemanticVersion>> {
    // delimited( 
    //     space0,
        alt((parse_semver_carrot, parse_semver_between, parse_semver_exact)) //,
        // space0
    // )
    (s)
}


/// Given a string like this: <package name>-<semver> (eg internal-1.2.3), return a 
/// tuple of package name, SemanticVersion.
pub fn parse_package_version(input: &str) -> IResult<&str, (&str, SemanticVersion)> {
    separated_pair(alphaword_many0_underscore_word, tag("-"), parse_semver)(input)

}

/// Given a package range str (<package>-<range>) return a tuple of &str , Range
/// # Example Input
/// - maya-1.2.3+<4 
/// - maya-^3.2
/// etc 
pub fn parse_package_range(input: &str) -> IResult<&str, (&str, Range<SemanticVersion>)> {
    separated_pair(alphaword_many0_underscore_word, tag("-"), parse_semver_range)(input)
}


#[cfg(test)]
#[path = "./unit_tests/parser.rs"]
mod unit_tests;