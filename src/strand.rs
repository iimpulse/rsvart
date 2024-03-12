use std::convert::TryFrom;
use crate::SvartError;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Strand {
    Positive,
    Negative,
}

impl Strand {
    pub fn is_positive(&self) -> bool {
        *self == Strand::Positive
    }

    pub fn is_negative(&self) -> bool {
        *self == Strand::Negative
    }

    pub fn opposite(&self) -> Strand {
        match *self {
            Strand::Negative => Strand::Positive,
            Strand::Positive => Strand::Negative
        }
    }
}

impl TryFrom<char> for Strand {
    type Error = SvartError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '+' => Ok(Strand::Positive),
            '-' => Ok(Strand::Negative),
            _ => Err(SvartError::IllegalValueError("Could not parse value for strand."))
        }
    }
}

impl TryFrom<&str> for Strand {
    type Error = SvartError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.to_uppercase().as_str() {
            "POS" | "POSITIVE" => Ok(Strand::Positive),
            "NEG" | "NEGATIVE" => Ok(Strand::Negative),
            _ => Err(SvartError::IllegalValueError("Could not parse value for strand."))
        }
    }
}

impl std::fmt::Display for Strand {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let sign: char = match &self.is_positive() {
            true => '+',
            _ => '-'
        };
        write!(f, "{}", sign)
    }
}

#[cfg(test)]
mod test {
    use std::convert::TryFrom;
    use super::Strand;
    use rstest::rstest;
    use crate::SvartError;

    #[rstest]
    #[case(Strand::Positive, true)]
    #[case(Strand::Negative, false)]
    fn test_is_positive(#[case] input: Strand, #[case] expected: bool) {
        assert_eq!(input.is_positive(), expected);
    }

    #[rstest]
    #[case(Strand::Negative, true)]
    #[case(Strand::Positive, false)]
    fn test_is_negative(#[case] input: Strand, #[case] expected: bool) {
        assert_eq!(input.is_negative(), expected);
    }

    #[rstest]
    #[case(Strand::Negative, "-")]
    #[case(Strand::Positive, "+")]
    fn test_correct_symbol(#[case] input: Strand, #[case] expected: String) {
        assert_eq!(format!("{}", input), expected)
    }

    #[rstest]
    #[case('-', Strand::Negative)]
    #[case('+', Strand::Positive)]
    fn test_correct_strand_char_pass(#[case] input: char, #[case] expected: Strand) {
        assert_eq!(Strand::try_from(input).unwrap(), expected);
    }

    #[rstest]
    #[case(":", SvartError::IllegalValueError("Could not parse value for strand."))]
    fn test_correct_strand_char_fail(#[case] input: char, #[case] expected: SvartError) {
        assert_eq!(Strand::try_from(input).unwrap_err(), expected);
    }

    #[rstest]
    #[case("pos", Strand::Positive)]
    #[case("neg", Strand::Negative)]
    #[case("positive", Strand::Positive)]
    #[case("negative", Strand::Negative)]
    fn test_correct_strand_string_pass(#[case] input: &str, #[case] expected: Strand) {
        assert_eq!(Strand::try_from(input).unwrap(), expected);
    }

    #[rstest]
    #[case("notaastrand", SvartError::IllegalValueError("Could not parse value for strand."))]
    fn test_correct_strand_string_fail(#[case] input: &str, #[case] expected: SvartError) {
        assert_eq!(Strand::try_from(input).unwrap_err(), expected);
    }

    #[rstest]
    #[case(Strand::Negative, Strand::Positive)]
    #[case(Strand::Positive, Strand::Negative)]
    fn test_opposite_strand(#[case] input: Strand, #[case] expected: Strand) {
        assert_eq!(input.opposite(), expected)
    }
}