use crate::{Contiged, GenomicRegion, Regioned};
use crate::stranded::Stranded;

pub trait StrandedContigedRegioned<T> {
    fn contains(&self, other: &U) -> bool;

    fn overlaps_with(&self, other: &U) -> bool;

    fn overlap_length(&self, other: &U) -> u32;

    fn distance_to(&self) -> u32;

    fn start_on_strand(&self) -> u32;
    
    fn end_on_strand(&self) -> u32;

}

impl <T: Regioned + Stranded + Contiged, U: Regioned + Stranded + Contiged> StrandedContigedRegioned<T, U> for T {
    fn contains(&self, other: &U) -> bool {
        
        if self.contig().id() != other.contig().id() {
            return false;
        }

        if self.strand() == other.strand() {
            return self.region().contains(other.region());
        }
        // TODO: If strands are not the same we need to do something here.
        return false;
    }

    //        if (this.contigId() != other.contigId()) {
    //             return false;
    //         }
    //         if (this.strand() == other.strand()) {
    //             return this.coordinates().overlaps(other.coordinates());
    //         }
    //         int otherStart = other.startOnStrand(this.strand());
    //         int otherEnd = other.endOnStrand(this.strand());
    //         return this.coordinates().overlaps(other.coordinateSystem(), otherStart, otherEnd);
    //     }
    fn overlaps_with(&self, other: &U) -> bool {
        if self.contig() != other.contig()  {
            return false;
        }
        
        if self.strand() == other.strand() {
            return self.region().overlaps()
        }
        
        todo!()
    }

    fn overlap_length(&self, other: &U) -> u32 {
        todo!()
    }

    fn distance_to(&self) -> u32 {
        todo!()
    }

    fn start_on_strand(&self) -> u32 {
        todo!()
    }

    fn end_on_strand(&self) -> u32 {
        todo!()
    }
}


#[cfg(test)]
mod test {
    use std::rc::Rc;
    use super::*;
    use crate::{AssignedMoleculeType, Contig, CoordinateSystem, SequenceRole, Strand};
    use crate::Region;
    use rstest::rstest;
    
    fn contigs() -> (Contig, Contig) {
        (
            Contig::new(1, "1".to_string(), SequenceRole::AssembledMolecule, "1".to_string(), AssignedMoleculeType::Chromosome, 249_250_621, "CM000663.1".to_string(), "NC_000001.10".to_string(), "chr1".to_string()).unwrap(),
            Contig::new(2, "2".to_string(), SequenceRole::AssembledMolecule, "1".to_string(), AssignedMoleculeType::Chromosome, 249_250_621, "CM000663.1".to_string(), "NC_000001.10".to_string(), "chr1".to_string()).unwrap()    
        )
    }
    
    fn region(index: u8) -> TestRegion {
        let (contig_1, contig_2) = contigs();
        match index {
            1 => TestRegion { strand: Strand::Positive,
                region: Region::new(0,20, CoordinateSystem::zero_based()), contig: Rc::new(contig_1.clone())
            },
            2 => TestRegion { strand: Strand::Positive,
                region: Region::new(13, 19, CoordinateSystem::zero_based()), contig: Rc::new(contig_1.clone())
            },
            3 => TestRegion { strand: Strand::Positive,
                region: Region::new(21, 36, CoordinateSystem::zero_based()), contig: Rc::new(contig_1.clone())
            },
            4 =>  TestRegion { strand: Strand::Negative,
                region: Region::new(21, 36, CoordinateSystem::zero_based()), contig: Rc::new(contig_1.clone())
            },
            5 => TestRegion { strand: Strand::Positive,
                region: Region::new(21, 36, CoordinateSystem::zero_based()), contig: Rc::new(contig_2.clone())
            },
            _ => panic!("Test setup not right")
        }
    }
    
    struct TestRegion {
        strand: Strand,
        region: Region,
        contig: Rc<Contig>
    }

    impl Stranded for TestRegion {
        fn strand(&self) -> Strand {
            return self.strand;
        }
    }

    impl Regioned for TestRegion {
        fn region(&self) -> &Region {
            &self.region
        }
    }

    impl Contiged for TestRegion {
        fn contig(&self) -> &Contig {
            &*self.contig
        }
    }

    #[rstest]
    #[case(region(1), region(2), true)]
    #[case(region(1), region(3), false)]
    #[case(region(1), region(4), false)]
    #[case(region(1), region(5), false)]
    fn test_contains(#[case] input: TestRegion, #[case] other: TestRegion, #[case] expected: bool) {
        assert_eq!(input.contains(&other), expected);
    }
}


