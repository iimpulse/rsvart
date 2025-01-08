use std::convert::TryFrom;
use crate::SvartError;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Strand {
    Forward,
    Reverse
}

impl Strand {
    pub fn is_forward(&self) -> bool {
        *self == Strand::Forward
    }

    pub fn is_reverse(&self) -> bool {
        *self == Strand::Reverse
    }

    pub fn opposite(&self) -> Strand {
        match *self {
            Strand::Reverse => Strand::Forward,
            Strand::Forward => Strand::Reverse
        }
    }
}

impl TryFrom<char> for Strand {
    type Error = SvartError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '+' => Ok(Strand::Forward),
            '-' => Ok(Strand::Reverse),
            _ => Err(SvartError::IllegalValueError("Could not parse value for strand."))
        }
    }
}

impl TryFrom<&str> for Strand {
    type Error = SvartError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.to_uppercase().as_str() {
            "POS" | "POSITIVE" | "FWD" | "FORWARD" => Ok(Strand::Forward),
            "NEG" | "NEGATIVE" | "REV" | "REVERSE" => Ok(Strand::Reverse),
            _ => Err(SvartError::IllegalValueError("Could not parse value for strand."))
        }
    }
}

impl std::fmt::Display for Strand {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let sign: char = match &self.is_forward() {
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
    #[case(Strand::Forward, true)]
    #[case(Strand::Reverse, false)]
    fn test_is_positive(#[case] input: Strand, #[case] expected: bool) {
        assert_eq!(input.is_forward(), expected);
    }

    #[rstest]
    #[case(Strand::Reverse, true)]
    #[case(Strand::Forward, false)]
    fn test_is_negative(#[case] input: Strand, #[case] expected: bool) {
        assert_eq!(input.is_reverse(), expected);
    }

    #[rstest]
    #[case(Strand::Reverse, "-")]
    #[case(Strand::Forward, "+")]
    fn test_correct_symbol(#[case] input: Strand, #[case] expected: String) {
        assert_eq!(format!("{}", input), expected)
    }

    #[rstest]
    #[case('-', Strand::Reverse)]
    #[case('+', Strand::Forward)]
    fn test_correct_strand_char_pass(#[case] input: char, #[case] expected: Strand) {
        assert_eq!(Strand::try_from(input).unwrap(), expected);
    }

    #[rstest]
    #[case(":", SvartError::IllegalValueError("Could not parse value for strand."))]
    fn test_correct_strand_char_fail(#[case] input: char, #[case] expected: SvartError) {
        assert_eq!(Strand::try_from(input).unwrap_err(), expected);
    }

    #[rstest]
    #[case("pos", Strand::Forward)]
    #[case("neg", Strand::Reverse)]
    #[case("positive", Strand::Forward)]
    #[case("negative", Strand::Reverse)]
    fn test_correct_strand_string_pass(#[case] input: &str, #[case] expected: Strand) {
        assert_eq!(Strand::try_from(input).unwrap(), expected);
    }

    #[rstest]
    #[case("notaastrand", SvartError::IllegalValueError("Could not parse value for strand."))]
    fn test_correct_strand_string_fail(#[case] input: &str, #[case] expected: SvartError) {
        assert_eq!(Strand::try_from(input).unwrap_err(), expected);
    }

    #[rstest]
    #[case(Strand::Reverse, Strand::Forward)]
    #[case(Strand::Forward, Strand::Reverse)]
    fn test_opposite_strand(#[case] input: Strand, #[case] expected: Strand) {
        assert_eq!(input.opposite(), expected)
    }
}