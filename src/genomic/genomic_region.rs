use crate::{Contig, Contiged};
use crate::{Located, Strand, Stranded, Unit};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct GenomicRegion<'g, C> {
    contig: &'g Contig<C>,
    start: C,
    end: C,
    strand: Strand,
}

impl<'g, C> GenomicRegion<'g, C>
where
    C: Unit,
{
    pub fn new(contig: &'g Contig<C>, start: C, end: C, strand: Strand) -> Option<Self> {
        if start > end || contig.end() < &end {
            None
        } else {
            Some(GenomicRegion {
                contig,
                start,
                end,
                strand,
            })
        }
    }
}

impl<'g, C> Contiged<C> for GenomicRegion<'g, C>
where
    C: Unit,
{
    type Contig = Contig<C>;
    fn contig(&self) -> &Contig<C> {
        self.contig
    }
}

impl<'g, C> Located<C> for GenomicRegion<'g, C> {
    fn start(&self) -> &C {
        &self.start
    }

    fn end(&self) -> &C {
        &self.end
    }
}

impl<'g, C> Stranded for GenomicRegion<'g, C> {
    fn strand(&self) -> Strand { 
        self.strand
    }
}

#[cfg(test)]
mod test {
    use super::Strand;
    use rstest::rstest;
    use crate::{AssignedMoleculeType, Contig, GenomicRegion, GenomicallyContains, GenomicallyOverlaps, SequenceRole};

    #[rstest]
    fn test_genomic_region(){
        let length: u32 = 10000;
        let contig: Contig<u32> = Contig::new("chr1".to_string(), SequenceRole::AssembledMolecule, "something".to_string(), AssignedMoleculeType::Chromosome, length,"CM061752.1".to_string(), "NM_1234".to_string(), "chr6".to_string()).unwrap();
        let genomic_region: GenomicRegion<u32> = GenomicRegion::new(&contig, 10, 20, Strand::Forward).unwrap();
        let genomic_region_1: GenomicRegion<u32> = GenomicRegion::new(&contig, 15, 20, Strand::Forward).unwrap();
        assert_eq!(genomic_region.contains(&genomic_region_1), true);
        assert_eq!(genomic_region.overlaps(&genomic_region_1), true);
    }
}