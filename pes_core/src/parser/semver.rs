use super::*;



/// Given a string representing a semantic version range - return a Range of SemanticVersion
/// 
/// # Example
/// ```
/// # use pes_core::{parser::parse_semver_range, SemanticVersion, ReleaseType};
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


/// Wraps ```parse_semver```, ensuring that it completely consumes the input, and simplifies the 
/// return signature. Failure to consume the input results in an error.
pub fn parse_consuming_semver(input: &str) -> Result<SemanticVersion, PesError> {
let result = all_consuming(ws(parse_semver))(input).map_err(|_| PesError::ParsingFailure(format!("parse_consuming_semver {}",input)))?;
let (_, result) = result;
Ok(result)
}
