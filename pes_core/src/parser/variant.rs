use super::*;
use crate::constants;


/// Given a string like this: <package name>-<semver>@<variant> (eg internal-1.2.3@0), return a 
/// tuple of package name, SemanticVersion.
///
/// # Example
/// ```
/// # use pes_core::parser::parse_package_variant;
/// # use pes_core::{SemanticVersion, Variant, ReleaseType};
/// # fn main()  {
/// let range = parse_package_variant("maya-1.2.3@0");
/// assert_eq!(
///    range, 
///    Ok(
///         ("", ("maya", Variant::new(SemanticVersion::new(1,2,3,ReleaseType::Release), 0)))
///      )
/// );
/// # }
/// ```
pub fn parse_package_variant(input: &str) -> PNResult<&str, (&str, Variant<SemanticVersion>)> {
    separated_pair(alphaword_many0_underscore_word, tag("-"), parse_variant_semver)(input)
}

/// Wraps ```parse_package_variant```, ensuring that it completely consumes the input
///
/// # Example
/// ```
/// # use pes_core::parser::parse_consuming_package_variant;
/// # use pes_core::{SemanticVersion, Variant, ReleaseType};
/// # fn main()  {
/// let variant = parse_consuming_package_variant("maya-1.2.3@0");
/// assert_eq!(
///    variant.unwrap(), 
///    ("maya", Variant::new(SemanticVersion::new(1,2,3,ReleaseType::Release), 0))
/// );
/// # }
/// ```
pub fn parse_consuming_package_variant(input: &str) -> Result<(&str, Variant<SemanticVersion>), PesError> {
    let (_,result) = all_consuming(
        ws(parse_package_variant)
    )(input)
    .map_err(|_| 
        PesError::ParsingFailure(
            format!("parsse_consuming_package_variant failed {}", input)
        )
    )?;

    Ok(result)
}

/// variant wraps a semver adding the notion of a "you guessed it" variant
pub fn parse_variant_semver(s: &str) -> PNResult<&str, Variant<SemanticVersion>> {
    let (rest, (semver, version)) = separated_pair(parse_semver, tag("@"), digit1)(s)?;
    let version = version.parse::<u8>().unwrap();
    let variant = Variant::new(semver, version);
    Ok((rest, variant))
}

/// Wraps ```parse_variant_semver```, ensuring that it completely consumes the input
pub fn parse_consuming_variant_semver(input: &str) -> Result<Variant<SemanticVersion>, PesError> {
    let (_,result) = all_consuming(
        ws(
            parse_variant_semver
        )
    )(input)
    .map_err(|_| 
        PesError::ParsingFailure(format!("parse_consuming_variant_semver {}",input))
    )?;
    Ok(result)
}

/// Parse an explicit semver with variant, and return a Range::Exact. Even though we are returning
/// a range it will be comprised of a singe Variant<SemanticVersion>.
pub fn parse_variant_semver_exact_range(s: &str) -> PNResult<&str, Range<Variant<SemanticVersion>>> {
    let (rest, (semver, version)) = separated_pair(parse_semver, tag("@"), digit1)(s)?;
    let version = version.parse::<u8>().unwrap();
    let variant = Range::exact(Variant::new(semver, version));
    Ok((rest, variant))
}

/// Wraps ```parse_variant_semver_range```, ensuring that it completely consumes the input
pub fn parse_consuming_variant_semver_exact_range(input: &str) -> Result<Range<Variant<SemanticVersion>>, PesError> {
    let (_,result) = all_consuming(
        ws(
            parse_variant_semver_exact_range
        )
    )(input)
    .map_err(|_| 
        PesError::ParsingFailure(format!("parse_consuming_variant_semver_exact_range {}",input))
    )?;
    Ok(result)
}


/// Parse an SemVer with a variant range implied by the lack of an explicit variant
pub fn parse_variant_semver_implicit_range(s: &str) -> PNResult<&str, Range<Variant<SemanticVersion>>> {
    let (rest, semver) = parse_semver(s)?;
    let variant = Range::between(Variant::new(semver, 0), Variant::new(semver, constants::MAX_VARIANTS));
    Ok((rest, variant))
}

/// Wraps ```parse_variant_semver```, ensuring that it completely consumes the input
pub fn parse_consuming_variant_semver_implicit_range(input: &str) -> Result<Range<Variant<SemanticVersion>>, PesError> {
    let (_,result) = all_consuming(
        ws(
            parse_variant_semver_implicit_range
        )
    )(input)
    .map_err(|_| 
        PesError::ParsingFailure(format!("parse_consuming_variant_semver_implicit_range {}",input))
    )?;
    Ok(result)
}
/*
/// Given a string representing a semantic version range - return a Range of SemanticVersion
/// 
/// # Example
/// ```
/// # use pes_core::{parser::parse_variant_semver_range, Variant, SemanticVersion, ReleaseType};
/// # use pubgrub::range::Range;
/// # fn main()  {
/// let range = parse_variant_semver_range("1.2.3+<3.0.0");
/// assert_eq!(
///     range, 
///     Ok(
///         ("", Range::between(Variant::new(SemanticVersion::new(1,2,3,ReleaseType::Release),0), Variant::new(SemanticVersion::new(3,0,0,ReleaseType::Release),0)))
///     )
/// );
/// # }
/// ```
*/
// pub fn parse_variant_semver_range(s: &str) -> PNResult<&str, Range<Variant<SemanticVersion>>> {
//     //alt((parse_variant_semver_carrot, parse_semver_between, parse_semver_exact))(s)
//     todo!()
// }

/// Given a str representing a semantic version range, return a `Range<Variant<SemanticVersion>>` or an error
/// Note that unlike a normal `nom` parser, this parser expects to completely consume the inupt. Any remaining
/// is interpreted as an error.
/// Furthermore, note that the parsre consumes any whitespace surounding the version range str.
///
/// # Example
/// ```ignore
///     range.unwrap(), 
///     Range::between(Variant::new(SemanticVersion::new(1,2,3,ReleaseType::Release),0), Varaint::new(SemanticVersion::new(3,0,0,ReleaseType::Release),4))
/// );
/// # }
/// ```



//*************************/
//   PRIVATE FUNCTIONS    */
//*************************/

// parse a variant like so (^1.2 or ^1.2.3@1). 
// Note that ^1.2 evaluates to a semantic version range betwen 1.2 and 2 AND
// a variant range between 0 and constants::MAX_VARIANT
fn parse_variant_semver_carrot(s: &str) -> PNResult<&str, Range<Variant<SemanticVersion>>> {
    alt((parse_explicit_variant_semver_carrot, parse_implicit_variant_semver_carrot))(s)
}

// given an input with an explicit variant (eg ^1.2.3@0), return an exact range
pub(crate) fn parse_explicit_variant_semver_carrot(s: &str) -> PNResult<&str, Range<Variant<SemanticVersion>>> {
   
    let (leftover,(first, rest, index)) = preceded(
                                        tag("^"), 
                                        tuple((
                                            digit1, 
                                            many_m_n(0, 2, preceded(tag("."), digit1)),
                                            preceded(tag("@"),digit1)
                                        ))
                                    )(s)?;
    let major = first.parse::<u32>().unwrap();
    let minor =  rest.get(0).unwrap_or(&"0").parse::<u32>().unwrap();
    let patch =  rest.get(1).unwrap_or(&"0").parse::<u32>().unwrap();
    let index = index.parse::<u8>().unwrap();

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

    Ok(
        (
            leftover, 
            Range::between(
                Variant::new(semver, index), 
                Variant::new(semver2,index)
            ) 
        )
    )
}

// given an input with an implicit variant (eg 1.2.3), return an exact range
fn parse_implicit_variant_semver_carrot(s: &str) -> PNResult<&str, Range<Variant<SemanticVersion>>> {
   
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

    Ok(
        (
            leftover, 
            Range::between(
                Variant::new(semver, 0), 
                Variant::new(semver2, constants::MAX_VARIANTS)
            )
        )
    )
}
