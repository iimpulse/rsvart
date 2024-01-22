#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Strand {
    Positive,
    Negative,
    Unknown
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
            Strand::Positive => Strand::Negative,
            Strand::Unknown => Strand::Unknown
        }
    }
}
impl From<char> for Strand {
    fn from(value: char) -> Strand {
        match value {
            '+' => Strand::Positive,
            '-' => Strand::Negative,
            _ => Strand::Unknown
        }
    }
}

impl From<&str> for Strand {
    fn from(value: &str) -> Strand {
        match value.to_uppercase().as_str() {
            "POS" | "POSITIVE" => Strand::Positive,
            "NEG" | "NEGATIVE" => Strand::Negative,
            _ => Strand::Unknown
        }
    }
}

impl std::fmt::Display for Strand {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let sign: char = match &self.is_positive() { true => '+', _ => '-' };
        write!(f, "{}", sign)
    }
}

#[cfg(test)]
mod test {
    use super::Strand;
    use rstest::rstest;

    #[rstest]
    #[case(Strand::Positive, true)]
    #[case(Strand::Negative, false)]
    fn test_is_positive(#[case] input: Strand, #[case] expected: bool){
        assert_eq!(input.is_positive(), expected);
    }

    #[rstest]
    #[case(Strand::Negative, true)]
    #[case(Strand::Positive, false)]
    fn test_is_negative(#[case] input: Strand, #[case] expected: bool){
        assert_eq!(input.is_negative(), expected);
    }

    #[rstest]
    #[case(Strand::Negative, "-")]
    #[case(Strand::Positive, "+")]
    fn test_correct_symbol(#[case] input: Strand, #[case] expected: String){
        assert_eq!(format!("{}", input), expected)
    }

    #[rstest]
    #[case('-', Strand::Negative)]
    #[case('+', Strand::Positive)]
    #[case(':', Strand::Unknown)]
    fn test_correct_strand_char(#[case] input: char, #[case] expected: Strand){
        assert_eq!(Strand::from(input), expected);
    }

    #[rstest]
    #[case("pos", Strand::Positive)]
    #[case("neg", Strand::Negative)]
    #[case("positive", Strand::Positive)]
    #[case("negative", Strand::Negative)]
    #[case("notaastrand", Strand::Unknown)]
    fn test_correct_strand_string(#[case] input: &str, #[case] expected: Strand){
        assert_eq!(Strand::from(input), expected);
    }

    #[rstest]
    #[case(Strand::Negative, Strand::Positive)]
    #[case(Strand::Positive, Strand::Negative)]
    #[case(Strand::Unknown, Strand::Unknown)]
    fn test_opposite_strand(#[case] input: Strand, #[case] expected: Strand){
        assert_eq!(input.opposite(), expected)
    }
}