use std::fmt::{Debug, Formatter};
use std::rc::Rc;

use crate::{ConfidenceInterval, Contig, CoordinateSystem, Region, Strand};
use crate::stranded::Stranded;

// The ideal pattern for minimizing memory usage uses one `GenomicAssembly` (we don't have that yet) which has
// a collection of `Contig`s.
// To create a `GenomicRegion`, one must get ahold of the `GenomicAssembly`, and the contigs. `Contig` is chosen and
// used to instantiate the `GenomicRegion`. The `GenomicRegion` does not store an owned copy of the `Contig`, it only stores
// a reference to the `Contig`. This implies that `GenomicAssembly` lives longer than all `GenomicRegion`s that
// refer to `Contig`s of the `GenomicAssembly`.
//
// We should write `GenomicRegion` trait to describe that it gives some sort of reference (&, Box, or whatever)

// TODO - this is possible, but it introduces an array of lifetimes everywhere
pub struct GenomicRegion {
    contig: Rc<Contig>,
    strand: Strand,
    region: Box<dyn Region>
}

impl GenomicRegion {
    pub fn new(contig: Rc<Contig>, strand: Strand, region: Box<dyn Region>) -> GenomicRegion {
        GenomicRegion {
            contig,
            strand,
            region,
        }
    }
}

impl Stranded for GenomicRegion {
    fn strand(&self) -> Strand {
        self.strand
    }

    fn with_strand(&self, strand: Strand) -> GenomicRegion {
        todo!()
        // if !self.strand.eq(&strand) {
        //     GenomicRegion {
        //         region: self.region.invert(self.contig.as_ref());
        //         strand: self.strand.opposite(),
        //         contig: self.contig;
        //     }
        // }
    }

    fn with_opposite_strand(&self) -> Self where Self: Sized {
        todo!()
    }

    fn to_positive_strand(&self) -> Self where Self: Sized {
        todo!()
    }

    fn to_negative_strand(&self) -> Self where Self: Sized {
        todo!()
    }
}

impl Region for GenomicRegion {
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

    fn with_coordinate_system(&mut self, cs: &CoordinateSystem) {
        self.region.with_coordinate_system(cs);
    }

    fn invert(&mut self, contig: &Contig) {
        self.region.invert(contig)
    }
}

impl Debug for GenomicRegion {
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

    use crate::{AssignedMoleculeType, ConfidenceInterval, Contig, CoordinateSystem, GenomicRegion, region, SequenceRole, Strand};

    #[test]
    fn new() {
        let contig = Rc::new(Contig::of(1, "1".to_string(), SequenceRole::AssembledMolecule, "1".to_string(), AssignedMoleculeType::Chromosome, 249_250_621, "CM000663.1".to_string(), "NC_000001.10".to_string(), "chr1".to_string()).unwrap());

        let strand = Strand::Positive;

        /* WORKS WITH PRECISE OR IMPRECISE */
        // let region = precise_region(CoordinateSystem::LeftOpen, 10, 20);
        let region = region(CoordinateSystem::zero_based(),
                            10, ConfidenceInterval::imprecise(5, 10),
                            20, ConfidenceInterval::imprecise(15, 20));


        let genomic_region = GenomicRegion::new(contig, strand, region);

        println!("GR: {:?}", genomic_region);
    }
}