#[derive(Debug, PartialEq)]
pub enum Strand {
    Positive,
    Negative
}

impl Strand {
    fn value(&self) -> char {
        match self {
            Strand::Positive => '+',
            Strand::Negative => '-'
        }
    }

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

#[cfg(test)]
mod test {
    use super::*;
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
    #[case(Strand::Negative, '-')]
    #[case(Strand::Positive, '+')]
    fn test_correct_symbol(#[case] input: Strand, #[case] expected: char){
        assert_eq!(input.value(), expected)
    }

    #[rstest]
    #[case(Strand::Negative, Strand::Positive)]
    #[case(Strand::Positive, Strand::Negative)]
    fn test_opposite_strand(#[case] input: Strand, #[case] expected: Strand){
        assert_eq!(input.opposite(), expected)
    }
}