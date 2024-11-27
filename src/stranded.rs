use crate::Strand;


pub trait Stranded {
    fn strand(&self) -> Strand;
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

        fn strand(&self) -> Strand {
            return self.strand;
        }
        
    }

    #[rstest]
    #[case(TestStrand { id: 1, strand: Strand::Positive}, Strand::Positive)]
    #[case(TestStrand { id: 1, strand: Strand::Negative}, Strand::Negative)]
    fn test_is_positive(#[case] input: TestStrand, #[case] expected: Strand) {
        assert_eq!(input.strand(), expected);
    }
}