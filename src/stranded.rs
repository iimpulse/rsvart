use crate::Strand;


pub trait Stranded {
    fn strand(&self) -> Strand;

    fn with_strand(&self, other: Strand) -> Self;

    fn with_opposite_strand(&self) -> Self where Self: Sized {
        self.with_strand(self.strand().opposite())
    }

    fn to_positive_strand(&self) -> Self where Self: Sized {
        self.with_strand(Strand::Positive)
    }

    fn to_negative_strand(&self) -> Self where Self: Sized {
        return self.with_strand(Strand::Negative);
    }
}


#[cfg(test)]
mod test {
    use super::*;
    use rstest::rstest;

    struct TestStrand {
        id: u16,
        strand: Strand
    }
    impl Stranded for TestStrand {
        fn with_strand(&self, s: Strand) -> TestStrand {
            return TestStrand {
                strand: s,       
                ..*self
            }
        }

        fn strand(&self) -> Strand {
            return self.strand.clone();
        }
    }

    #[rstest]
    #[case(Strand::Positive)]
    fn test_with_strand(#[case] expected_strand: Strand){
        let test_strand = TestStrand { id: 32, strand: Strand::Negative};
        let new_test_strand = test_strand.with_strand(Strand::Positive);
        assert_eq!(new_test_strand.strand, expected_strand);
    }


    #[rstest]
    #[case(Strand::Positive)]
    fn test_with_opposite_strand(#[case] expected_strand: Strand){
        let test_strand = TestStrand { id: 32, strand: Strand::Negative};
        let new_test_strand = test_strand.with_opposite_strand();
        assert_eq!(new_test_strand.strand, expected_strand);
    }

    #[rstest]
    #[case(Strand::Positive)]
    fn test_with_positive_strand(#[case] expected_strand: Strand){
        let test_strand = TestStrand { id: 32, strand: Strand::Negative};
        let new_test_strand = test_strand.to_positive_strand();
        assert_eq!(new_test_strand.strand, expected_strand);
    }

    #[rstest]
    #[case(Strand::Negative)]
    fn test_with_negative_strand(#[case] expected_strand: Strand){
        let test_strand = TestStrand { id: 32, strand: Strand::Positive};
        let new_test_strand = test_strand.to_negative_strand();
        assert_eq!(new_test_strand.strand, expected_strand);
    }
}