use std::fmt::{Debug, Formatter};
use std::rc::Rc;

use crate::{ConfidenceInterval, Contig, CoordinateSystem, GenomicRegion, Region, Strand};

pub struct GenomicRegionDefault {
    contig: Rc<dyn Contig>,
    strand: Strand,
    region: Box<dyn Region>,
}

impl GenomicRegionDefault {
    pub fn new(contig: Rc<dyn Contig>, strand: Strand, region: Box<dyn Region>) -> GenomicRegionDefault {
        GenomicRegionDefault {
            contig,
            strand,
            region,
        }
    }
}

impl GenomicRegion for GenomicRegionDefault {
    fn contig(&self) -> &Rc<dyn Contig> {
        &self.contig
    }

    fn strand(&self) -> &Strand {
        &self.strand
    }
}

impl Region for GenomicRegionDefault {
    fn start(&self) -> u32 {
        self.region.start()
    }

    fn end(&self) -> u32 {
        self.region.end()
    }

    fn coordinate_system(&self) -> &CoordinateSystem {
        self.region.coordinate_system()
    }

    fn start_confidence_interval(&self) -> &ConfidenceInterval {
        self.region.start_confidence_interval()
    }

    fn end_confidence_interval(&self) -> &ConfidenceInterval {
        self.region.end_confidence_interval()
    }

    fn as_precise(self) -> Box<dyn Region> {
        self.region.as_precise()
    }
}

impl Debug for GenomicRegionDefault {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GenomicRegionDefault")
            .field("contig_id", &self.contig.id())
            .field("contig_name", &self.contig.name())
            .field("strand", &self.strand)
            .field("coordinate_system", &self.region.coordinate_system())
            .field("start", &self.region.start())
            .field("end", &self.region.end())
            .finish()
    }
}


// --------------------------------------- TESTS ---------------------------------------------------

#[cfg(test)]
mod genomic_region_tests {
    use std::rc::Rc;

    use crate::{AssignedMoleculeType, ConfidenceInterval, CoordinateSystem, precise_region, region, SequenceRole, Strand};
    use crate::default::ContigDefault;

    use super::GenomicRegionDefault;

    #[test]
    fn new() {
        let contig = Rc::new(ContigDefault::of(1, "1".to_string(), SequenceRole::AssembledMolecule, "1".to_string(), AssignedMoleculeType::Chromosome, 249_250_621, "CM000663.1".to_string(), "NC_000001.10".to_string(), "chr1".to_string()).unwrap());

        let strand = Strand::Positive;

        /* WORKS WITH PRECISE OR IMPRECISE */
        // let region = precise_region(CoordinateSystem::LeftOpen, 10, 20);
        let region = region(CoordinateSystem::LeftOpen,
                            10, ConfidenceInterval::imprecise(5, 10),
                            20, ConfidenceInterval::imprecise(15, 20));


        let genomic_region = GenomicRegionDefault::new(contig, strand, region);

        println!("GR: {:?}", genomic_region);
    }
}