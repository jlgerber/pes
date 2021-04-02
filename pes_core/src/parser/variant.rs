use super::*;
use nom::character::complete::space0;
use crate::constants;

/// parses implicit and explicit packages with variant ranges (including caret prefixed).
///
/// #Examples
///
/// ```ignore
/// maya-1.2.3+<2.3.2
/// houdini-^0.2
/// nuke-0.1@2+<1.1.1@2
/// ```
pub fn parse_package_variants_range(input: &str)-> PNResult<&str, (&str, Range<Variant<SemanticVersion>>)> {
    separated_pair(
        alphaword_many0_underscore_word, 
        tag("-"), 
        alt(
            (
                parse_semver_variants_between,
                parse_caret_variant_semver_range
            )
        ))(input)
}

/// Wraps `parse_package_variants_range` ensuring that it consumes its input completely.
///
/// # Example
/// ```
/// # use pes_core::parser::parse_consuming_package_variants_range;
/// # use pes_core::{constants, SemanticVersion, Variant, ReleaseType, range::Range};
/// # fn main()  {
/// let variant = parse_consuming_package_variants_range("maya-^1.2.3-beta");
/// assert_eq!(
///    variant.unwrap(), 
///    ("maya", Range::between(
///               Variant::new(SemanticVersion::new(1, 2, 3, ReleaseType::Beta), 0),
///               Variant::new(SemanticVersion::new(2, 0, 0, ReleaseType::Beta), constants::MAX_VARIANTS),
///             )
///    ));
///
/// // note - just because you can doesnt mean you should. One should not normally refer to a specific
/// // variant index in a package range. But it does work technically...
/// let variant = parse_consuming_package_variants_range("maya-1.2.3-beta@2+<4");
/// assert_eq!(
///    variant.unwrap(), 
///    ("maya", Range::between(
///               Variant::new(SemanticVersion::new(1, 2, 3, ReleaseType::Beta), 2),
///               Variant::new(SemanticVersion::new(4, 0, 0, ReleaseType::Release), constants::MAX_VARIANTS),
///             )
///    ));
///
/// # }
/// ```
pub fn parse_consuming_package_variants_range(input: &str) -> Result<(&str, Range<Variant<SemanticVersion>>), PesError> {
    let (_,result) = all_consuming(
        ws(parse_package_variants_range)
    )(input)
    .map_err(|_| 
        PesError::ParsingFailure(
            format!("parse_consuming_package_variants_range failed {}", input)
        )
    )?;

    Ok(result)
}

/// Given a distribution with explicit or implicit variant, return a tuple with the package name and a range over the semver variant
/// In otherwords, the package and SemanticVersion should be exact, while the variant index may either be exact or a range.
///
/// # Example
/// ```
/// # use pes_core::parser::parse_package_variants;
/// # use pes_core::{SemanticVersion, Variant, ReleaseType, range::Range};
/// # fn main()  {
/// let range = parse_package_variants("maya-1.2.3@0");
/// assert_eq!(
///    range, 
///    Ok(
///         ("", ("maya", Range::exact(Variant::new(SemanticVersion::new(1,2,3,ReleaseType::Release), 0))))
///      )
/// );
/// # }
/// ```
pub fn parse_package_variants(input: &str) -> PNResult<&str, (&str, Range<Variant<SemanticVersion>>)> {
    separated_pair(
        alphaword_many0_underscore_word, 
        tag("-"), 
        alt(
            (
                parse_variant_semver_exact_range,
                parse_semver_with_implicit_variant_range
            )
        ))(input)
}

/// Wraps ```parse_package_variants```, ensuring that the full input is consumed
///
/// # Example
/// ```
/// # use pes_core::parser::parse_consuming_package_variants;
/// # use pes_core::{constants, SemanticVersion, Variant, ReleaseType, range::Range};
/// # fn main()  {
/// let variant = parse_consuming_package_variants("maya-1.2.3@0");
/// assert_eq!(
///    variant.unwrap(), 
///    ("maya", Range::exact(Variant::new(SemanticVersion::new(1,2,3,ReleaseType::Release), 0)))
/// );
///
/// let variant = parse_consuming_package_variants("maya-1.2.3");
/// assert_eq!(
///    variant.unwrap(), 
///    ("maya", Range::between(
///               Variant::new(SemanticVersion::new(1, 2, 3, ReleaseType::Release), 0),
///               Variant::new(SemanticVersion::new(1, 2, 3, ReleaseType::Release), constants::MAX_VARIANTS),
///             )
///    ));
/// # }
/// ```
pub fn parse_consuming_package_variants(input: &str) -> Result<(&str, Range<Variant<SemanticVersion>>), PesError> {
    let (_,result) = all_consuming(
        ws(parse_package_variants)
    )(input)
    .map_err(|_| 
        PesError::ParsingFailure(
            format!("parse_consuming_package_variants failed {}", input)
        )
    )?;

    Ok(result)
}

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
///
/// # Example
/// ```
/// # use pes_core::parser::parse_variant_semver;
/// # use pes_core::{SemanticVersion, Variant, ReleaseType};
/// # fn main()  {
/// let value = parse_variant_semver("1.2.3@0");
/// assert_eq!(
///    value, 
///    Ok(
///         ("", Variant::new(SemanticVersion::new(1,2,3,ReleaseType::Release), 0))
///      )
/// );
/// # }
/// ```
pub fn parse_variant_semver(s: &str) -> PNResult<&str, Variant<SemanticVersion>> {
    let (rest, (semver, version)) = separated_pair(parse_semver, tag("@"), digit1)(s)?;
    let version = version.parse::<u8>().unwrap();
    let variant = Variant::new(semver, version);
    Ok((rest, variant))
}

/// Wraps ```parse_variant_semver```, ensuring that it completely consumes the input
///
/// # Example
/// ```
/// # use pes_core::parser::parse_consuming_variant_semver;
/// # use pes_core::{SemanticVersion, Variant, ReleaseType};
/// # fn main()  {
/// let value = parse_consuming_variant_semver("1.2.3@0");
/// assert_eq!(
///    value.unwrap(), 
///        Variant::new(SemanticVersion::new(1,2,3,ReleaseType::Release), 0)
/// );
/// # }
/// ```
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
///
/// # Example
/// ```
/// # use pes_core::parser::parse_variant_semver_exact_range;
/// # use pes_core::{SemanticVersion, Variant, ReleaseType, range::Range};
/// # fn main()  {
/// let value = parse_variant_semver_exact_range("1.2.3@0");
/// assert_eq!(
///    value.unwrap(), 
///        ("", Range::exact(Variant::new(SemanticVersion::new(1,2,3,ReleaseType::Release), 0)))
/// );
/// # }
/// ```
pub fn parse_variant_semver_exact_range(s: &str) -> PNResult<&str, Range<Variant<SemanticVersion>>> {
    let (rest, (semver, version)) = separated_pair(parse_semver, tag("@"), digit1)(s)?;
    let version = version.parse::<u8>().unwrap();
    let variant = Range::exact(Variant::new(semver, version));
    Ok((rest, variant))
}

/// Wraps ```parse_variant_semver_range```, ensuring that it completely consumes the input
///
/// # Example
/// ```
/// # use pes_core::parser::parse_variant_semver_exact_range;
/// # use pes_core::{SemanticVersion, Variant, ReleaseType, range::Range};
/// # fn main()  {
/// let value = parse_variant_semver_exact_range("1.2.3@0");
/// assert_eq!(
///    value.unwrap(), 
///        ("", Range::exact(Variant::new(SemanticVersion::new(1,2,3,ReleaseType::Release), 0)))
/// );
/// # }
/// ```
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


/// Parse an SemVer with a variant range implied by the lack of an explicit variant. in other words
/// given an input like this `1.3.4` produce a `Range<Variant<SemanticVersion>>`, where the Range
/// is over the variant indexes from 0..constants::MAX_VARIANTS.
pub fn parse_semver_with_implicit_variant_range(s: &str) -> PNResult<&str, Range<Variant<SemanticVersion>>> {
    let (rest, semver) = parse_semver(s)?;
    let variant = Range::between(
        Variant::new(semver, 0), 
        Variant::new(semver, constants::MAX_VARIANTS)
    );
    Ok((rest, variant))
}

/// Wraps the ```parse_semver_with_implicit_variant_range``` parser, ensuring that it completely consumes the input
pub fn parse_consuming_semver_with_implicit_variant_range(input: &str) -> Result<Range<Variant<SemanticVersion>>, PesError> {
    let (_,result) = all_consuming(
        ws(
            parse_semver_with_implicit_variant_range
        )
    )(input)
    .map_err(|_| 
        PesError::ParsingFailure(format!("parse_consuming_semver_with_implicit_variant_range {}",input))
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
//     //alt((parse_caret_variant_semver_range, parse_semver_between, parse_semver_exact))(s)
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
pub(crate) fn parse_caret_variant_semver_range(s: &str) -> PNResult<&str, Range<Variant<SemanticVersion>>> {
    alt((parse_caret_explicit_variant_semver_range, parse_caret_implicit_variant_semver_range))(s)
}

// given an input with an explicit variant (eg ^1.2.3@0 or ^1.2@3 or ^1@1 ), return an exact range
pub(crate) fn parse_caret_explicit_variant_semver_range(s: &str) -> PNResult<&str, Range<Variant<SemanticVersion>>> {
    fn _parse_variant_prerelease(s: &str) -> PNResult<&str, (Variant<SemanticVersion>, usize)> {
        let (leftover,(first, rest, release_type, index)) = tuple((
            digit1, 
            many_m_n(0, 2, preceded(tag("."), digit1)),
            preceded(tag("-"), parse_prerelease),
            preceded(tag("@"),digit1)
        ))(s)?;

        let variant = Variant::new(
            SemanticVersion::new(
                first.parse::<u32>().unwrap(),
                rest.get(0).unwrap_or(&"0").parse::<u32>().unwrap(),
                rest.get(1).unwrap_or(&"0").parse::<u32>().unwrap(),
                release_type
            ), 
            index.parse::<u8>().unwrap()
        );
    
        Ok((leftover,(variant, rest.len())))
    }

    fn _parse_variant_release(s: &str) -> PNResult<&str, (Variant<SemanticVersion>, usize)> {
        let (leftover,(first, rest, index)) = tuple((
            digit1, 
            many_m_n(0, 2, preceded(tag("."), digit1)),
            preceded(tag("@"),digit1)
        ))(s)?;

        let variant = Variant::new(
            SemanticVersion::new(
                first.parse::<u32>().unwrap(),
                rest.get(0).unwrap_or(&"0").parse::<u32>().unwrap(),
                rest.get(1).unwrap_or(&"0").parse::<u32>().unwrap(),
                ReleaseType::Release
            ), 
            index.parse::<u8>().unwrap()
        );
    
        Ok((leftover,(variant, rest.len())))
    }

    let (leftover, (variant, rest_len)) = preceded(tag("^"),alt((_parse_variant_prerelease, _parse_variant_release)))(s)?;
 
    let semver2 = match rest_len {
        0 => SemanticVersion::new(variant.major()+1, 0, 0, variant.release_type()),
        1 => {
            if variant.major() >= 1 {
                SemanticVersion::new(variant.major()+1, 0, 0, variant.release_type())
            } else {
                SemanticVersion::new(variant.major(), variant.minor()+1, 0, variant.release_type())
            }
        },
        2 => {
            if variant.major() >= 1 {
                // eg ^1.2.3  :=  >=1.2.3, <2.0.0
                SemanticVersion::new(variant.major()+1, 0, 0, variant.release_type())
            } else if variant.minor() == 0{
                // eg ^0.0.3  :=  >=0.0.3, <0.0.4
                SemanticVersion::new(variant.major(), variant.minor(), variant.patch()+1, variant.release_type())
            } else {
                // eg ^0.2.3  :=  >=0.2.3, <0.3.0
                SemanticVersion::new(variant.major(), variant.minor() + 1, 0, variant.release_type())
            }
        },
        _ => panic!("invalid semantic version")
    };

    let index = variant.index();
    Ok(
        (
            leftover, 
            Range::between(
                variant, 
                Variant::new(semver2,index)
            ) 
        )
    )
}

// given an input with an implicit variant (eg ^1.2.3 or ^1.2 or ^1), return an exact range
fn parse_caret_implicit_variant_semver_range(s: &str) -> PNResult<&str, Range<Variant<SemanticVersion>>> {
    fn _parse_semver_prerelease(s: &str) -> PNResult<&str, (SemanticVersion, usize)> {
        let (leftover,(first, rest, release_type)) = tuple((digit1, many_m_n(0, 2, preceded(tag("."), digit1)),preceded(tag("-"), parse_prerelease) ))(s)?;
        let semver = SemanticVersion::new(
            first.parse::<u32>().unwrap(),
            rest.get(0).unwrap_or(&"0").parse::<u32>().unwrap(),
            rest.get(1).unwrap_or(&"0").parse::<u32>().unwrap(),
           release_type
        );
    
        Ok((leftover,(semver, rest.len())))
    }

    fn _parse_semver_release(s: &str) -> PNResult<&str, (SemanticVersion, usize)> {
        let (leftover,(first, rest)) = tuple((digit1, many_m_n(0, 2, preceded(tag("."), digit1))))(s)?;
        let semver = SemanticVersion::new(
            first.parse::<u32>().unwrap(),
            rest.get(0).unwrap_or(&"0").parse::<u32>().unwrap(),
            rest.get(1).unwrap_or(&"0").parse::<u32>().unwrap(),
            ReleaseType::Release
        );
    
        Ok((leftover,(semver, rest.len())))
    }
    let (leftover, (semver, rest_len)) = preceded(tag("^"),alt((_parse_semver_prerelease, _parse_semver_release)))(s)?;
 
    let semver2 = match rest_len {
        0 => SemanticVersion::new(semver.major+1, 0, 0, semver.release_type.clone()),
        1 => {
            if semver.major >= 1 {
                SemanticVersion::new(semver.major+1, 0, 0, semver.release_type.clone())
            } else {
                SemanticVersion::new(semver.major, semver.minor+1, 0, semver.release_type.clone())
            }
        },
        2 => {
            if semver.major >= 1 {
                SemanticVersion::new(semver.major+1, 0, 0, semver.release_type.clone())
            } else if semver.minor == 0{
                // eg ^0.0.3  :=  >=0.0.3, <0.0.4
                SemanticVersion::new(semver.major, semver.minor, semver.patch+1, semver.release_type.clone())
            } else {
                // eg ^0.2.3  :=  >=0.2.3, <0.3.0
                SemanticVersion::new(semver.major, semver.minor + 1, 0, semver.release_type.clone())
            }
        },
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

// Given a string representing two semantic versions separated by '+<', return a Range::between the first and second
// Variant wrapped SemanticVersion instances
pub(crate) fn parse_semver_variants_between(s: &str) -> PNResult<&str, Range<Variant<SemanticVersion>>> {
   
    fn parse_semver_returning_min_variant(s: &str)-> PNResult<&str, Variant<SemanticVersion>> {
        let (leftover, semver) = parse_semver(s)?;
        Ok((leftover, Variant::new(semver, 0)))
    }

    fn parse_semver_returning_max_variant(s: &str)-> PNResult<&str, Variant<SemanticVersion>> {
        let (leftover, semver) = parse_semver(s)?;
        Ok((leftover, Variant::new(semver, constants::MAX_VARIANTS)))
    }

    let (leftover, (sm1,sm2)) = separated_pair(
        alt((parse_variant_semver, parse_semver_returning_min_variant)),
        delimited(space0,alt((tag(constants::HO_RANGE_0), tag(constants::HO_RANGE_1))), space0), 
        alt((parse_variant_semver, parse_semver_returning_max_variant))
    )(s)?;
    // todo check to see that sm2 >= sm1
    Ok(
        (
            leftover, 
            Range::between(
                sm1, 
                sm2
            )
        )
    )
}



#[cfg(test)]
#[path = "../unit_tests/parser_variant.rs"]
mod unit_tests;