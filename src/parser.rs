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

fn parse_semver_start(s: &str) -> IResult<&str, &str> {
    terminated(digit1,tag("."))(s)
}

fn parse_semver_opt(s: &str) -> IResult<&str, Vec<&str>> {
    many_m_n(0, 2, preceded(tag("."), digit1))(s)
}

fn parse_semver(s: &str) -> IResult<&str, SemanticVersion> {
    let (leftover,(first, rest)) = tuple((digit1, many_m_n(0, 2, preceded(tag("."), digit1))))(s)?;
    let semver = SemanticVersion::new(
        first.parse::<u32>().unwrap(),
        rest.get(0).unwrap_or_else(|| &"0").parse::<u32>().unwrap(),
        rest.get(1).unwrap_or_else(|| &"0").parse::<u32>().unwrap()
    );

    Ok((leftover,semver))
}

fn parse_semver_between(s: &str) -> IResult<&str, Range<SemanticVersion>> {
    let (leftover, (sm1,sm2)) = separated_pair(parse_semver, delimited(many0(tag(" ")),tag("+<"), many0(tag(" "))), parse_semver)(s)?;
    Ok((leftover, Range::between(sm1, sm2)))
}

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

fn parse_semver_exact(s: &str) -> IResult<&str, Range<SemanticVersion>> {
    let (leftover, semver) = parse_semver(s)?;
    Ok((leftover, Range::exact(semver)))
}

/// Given a string representing a semantic version range - either
pub fn parse_semver_range(s: &str) -> IResult<&str, Range<SemanticVersion>> {
    delimited( 
        space0,
        alt((parse_semver_carrot, parse_semver_between, parse_semver_exact)),
        space0
    )(s)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        let result = parse_semver_start("123.");
        assert_eq!(result, Ok(("","123")));
    }

    #[test]
    fn parse_opt() {
        let result = parse_semver_opt(".123.2");
        assert_eq!(result, Ok(("",vec!["123", "2"])));
    }

    #[test]
    fn parse_semver_goodinput() {
        let result = parse_semver("1.2.3");
        assert_eq!(result, Ok(("",SemanticVersion::new(1,2,3))));
    }

    #[test]
    fn parse_range_goodinput() {
        let result = parse_semver_between("1.2.3+<3.4.5");
        assert_eq!(result, Ok(("", Range::between(SemanticVersion::new(1,2,3), SemanticVersion::new(3,4,5)))));
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
            ("   1.23.4   ", Ok(("", Range::exact(SemanticVersion::new(1,23,4))))) ,
            ("1.23.4", Ok(("", Range::exact(SemanticVersion::new(1,23,4))))) ,
            (" 1.2.3 +< 3 ", Ok(("", Range::between(SemanticVersion::new(1,2,3), SemanticVersion::new(3,0,0))))),
            ("1.2.3+<3", Ok(("", Range::between(SemanticVersion::new(1,2,3), SemanticVersion::new(3,0,0))))),
            (" ^2.2", Ok(("", Range::between(SemanticVersion::new(2,2,0), SemanticVersion::new(2,3,0) ))))
        ];

        for (input,expected) in versions {
            assert_eq!(parse_semver_range(input), expected);
        }

    }
}