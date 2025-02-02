use crate::genomic::{Contiged, Strand, Stranded};
use crate::ops::{Located, Unit};

pub trait Transposable<C> {
    fn start_on_strand(&self, strand: Strand) -> C;

    fn end_on_strand(&self, strand: Strand) -> C;
}

impl<C, T> Transposable<C> for T
where
    C: Unit,
    T: Contiged<C> + Located<C> + Stranded,
{
    fn start_on_strand(&self, strand: Strand) -> C {
        match self.strand().eq(&strand) {
            true => *self.start(),
            false => *self.contig().end() - *self.end(),
        }
    }

    fn end_on_strand(&self, strand: Strand) -> C {
        match self.strand().eq(&strand) {
            true => *self.end(),
            false => *self.contig().end() - *self.start(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::genomic::{AssignedMoleculeType, Contig, SequenceRole};
    use rstest::rstest;

    #[derive(PartialEq, Eq)]
    struct TestRegion {
        strand: Strand,
        start: u8,
        end: u8,
        contig: Contig<u8>,
    }

    impl Stranded for TestRegion {
        fn strand(&self) -> Strand {
            self.strand
        }
    }

    impl Located<u8> for TestRegion {
        fn start(&self) -> &u8 {
            &self.start
        }

        fn end(&self) -> &u8 {
            &self.end
        }
    }

    impl Contiged<u8> for TestRegion {
        type Contig = Contig<u8>;

        fn contig(&self) -> &Self::Contig {
            &self.contig
        }
    }
    fn get_test_contig() -> Contig<u8> {
        Contig::new(
            "1".to_string(),
            SequenceRole::AssembledMolecule,
            "1".to_string(),
            AssignedMoleculeType::Chromosome,
            249,
            "CM000663.1".to_string(),
            "NC_000001.10".to_string(),
            "chr1".to_string(),
        )
        .unwrap()
    }
    #[rstest]
    fn test_start_on_strand() {
        let contig = get_test_contig();
        let test_region = TestRegion {
            strand: Strand::Reverse,
            start: 32,
            end: 200,
            contig,
        };
        assert_eq!(
            test_region.start_on_strand(Strand::Reverse),
            *test_region.start()
        );
        assert_eq!(test_region.start_on_strand(Strand::Forward), 49);
    }

    #[rstest]
    fn test_end_on_strand() {
        let contig = get_test_contig();
        let test_region = TestRegion {
            strand: Strand::Reverse,
            start: 32,
            end: 200,
            contig,
        };
        assert_eq!(
            test_region.end_on_strand(Strand::Reverse),
            *test_region.end()
        );
        assert_eq!(test_region.end_on_strand(Strand::Forward), 217);
    }
}
