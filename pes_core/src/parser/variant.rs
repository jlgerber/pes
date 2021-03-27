use super::*;


/// variant wraps a semver adding the notion of a "you guessed it" variant
pub fn parse_variant_semver(s: &str) -> PNResult<&str, Variant<SemanticVersion>> {
    let (rest, (semver, version)) = separated_pair(parse_semver, tag("@"), digit1)(s)?;
    let version = version.parse::<u8>().unwrap();
    let variant = Variant::new(semver, version);
    Ok((rest, variant))
}

/// Wraps ```parse_variant_semver```, ensuring that it completely consumes the input
pub fn parse_consuming_variant_semver(input: &str) -> Result<Variant<SemanticVersion>, PesError> {
    let result = all_consuming(ws(parse_variant_semver))(input).map_err(|_| PesError::ParsingFailure(format!("parse_consuming_variant_semver {}",input)))?;
    let (_, result) = result;
    Ok(result)
}