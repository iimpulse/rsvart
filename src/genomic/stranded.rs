use super::Strand;

pub trait Stranded {
    fn strand(&self) -> Strand;
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::rstest;

    struct TestStrand {
        id: u16,
        strand: Strand,
    }
    impl Stranded for TestStrand {
        fn strand(&self) -> Strand {
            self.strand
        }
    }

    #[rstest]
    #[case(TestStrand { id: 1, strand: Strand::Forward}, Strand::Forward)]
    #[case(TestStrand { id: 1, strand: Strand::Reverse}, Strand::Reverse)]
    fn test_is_positive(#[case] input: TestStrand, #[case] expected: Strand) {
        assert_eq!(input.strand(), expected);
        assert_eq!(input.id, 1)
    }
}
