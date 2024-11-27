use crate::Strand;
use crate::stranded::Stranded;

pub trait Transposable: Stranded {

    fn with_strand(&mut self, other: Strand);

    fn with_opposite_strand(&mut self) {
        self.with_strand(self.strand().opposite())
    }
    
    fn with_positive_strand(&mut self) {
        self.with_strand(Strand::Positive)
    }

    fn with_negative_strand(&mut self) {
        self.with_strand(Strand::Negative)
    }
}


#[cfg(test)]
mod test {
    use rstest::rstest;
    use super::*;

    struct TestStrand {
        id: u16,
        strand: Strand
    }
    impl Stranded for TestStrand {
        fn strand(&self) -> Strand {
            self.strand
        }
    }
    
    impl Transposable for TestStrand {
        fn with_strand(&mut self, other: Strand) {
            self.strand = other;
        }

        fn with_opposite_strand(&mut self) {
            self.strand = self.strand.opposite();
        }

        fn with_positive_strand(&mut self) {
            self.strand = Strand::Positive;
        }

        fn with_negative_strand(&mut self) {
            self.strand = Strand::Negative
        }
    }

    #[rstest]
    #[case(Strand::Positive)]
    fn test_with_strand(#[case] expected_strand: Strand){
        let mut test_strand = TestStrand { id: 32, strand: Strand::Negative};
        test_strand.with_strand(Strand::Positive);
        assert_eq!(test_strand.strand(), expected_strand);
    }

    #[rstest]
    #[case(Strand::Positive)]
    fn test_with_opposite_strand(#[case] expected_strand: Strand){
        let mut test_strand = TestStrand { id: 32, strand: Strand::Negative};
        test_strand.with_opposite_strand();
        assert_eq!(test_strand.strand(), expected_strand);
    }

    #[rstest]
    #[case(Strand::Positive)]
    fn test_with_positive_strand(#[case] expected_strand: Strand){
        let mut test_strand = TestStrand { id: 32, strand: Strand::Negative};
        test_strand.with_positive_strand();
        assert_eq!(test_strand.strand(), expected_strand);
    }

    #[rstest]
    #[case(Strand::Negative)]
    fn test_with_negative_strand(#[case] expected_strand: Strand){
        let mut test_strand = TestStrand { id: 32, strand: Strand::Positive};
        test_strand.with_negative_strand();
        assert_eq!(test_strand.strand(), expected_strand);
    }
    
}
    
