use super::*;

/// Given a string like this: <package name>-<semver> (eg internal-1.2.3), return a 
/// tuple of package name, SemanticVersion.
///
/// # Example
/// ```
/// # use pes_core::parser::parse_package_version;
/// # use pubgrub::{range::Range};
/// # use pes_core::{SemanticVersion, ReleaseType};
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
/// # use pes_core::{parser::parse_consuming_package_range, SemanticVersion, ReleaseType };
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
