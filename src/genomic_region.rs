use crate::{ConfidenceInterval, CoordinateSystem};
use super::contig::Contig;
use super::region::Region;
use super::strand::Strand;

pub trait GenomicRegion: Region {

    // The ideal pattern for minimizing memory usage uses one `GenomicAssembly` (we don't have that yet) which has
    // a collection of `Contig`s.
    // To create a `GenomicRegion`, one must get ahold of the `GenomicAssembly`, and the contigs. `Contig` is chosen and
    // used to instantiate the `GenomicRegion`. The `GenomicRegion` does not store an owned copy of the `Contig`, it only stores
    // a reference to the `Contig`. This implies that `GenomicAssembly` lives longer than all `GenomicRegion`s that
    // refer to `Contig`s of the `GenomicAssembly`.
    //
    // We should write `GenomicRegion` trait to describe that it gives some sort of reference (&, Box, or whatever)

    fn contig(&self) -> Box<dyn Contig>;

    fn strand(&self) -> &Strand;

    fn contig_id(&self) -> usize {
        self.contig().id()
    }

    fn contig_name(&self) -> &str {
        self.contig().name()
    }

}


struct GenomicRegionDefault {
    contig: Box<dyn Contig>,
    strand: Strand,
    coordinate_system: CoordinateSystem,
    start: u32,
    end: u32
}

impl Region for GenomicRegionDefault {
    fn start(&self) -> u32 {
        self.start
    }

    fn end(&self) -> u32 {
        self.end
    }

    fn coordinate_system(&self) -> &CoordinateSystem {
        &self.coordinate_system
    }

    fn start_confidence_interval(&self) -> &ConfidenceInterval {
        todo!()
    }

    fn end_confidence_interval(&self) -> &ConfidenceInterval {
        todo!()
    }

    fn as_precise(&self) -> Box<dyn GenomicRegion> {
        todo!()
    }
}

impl GenomicRegion for GenomicRegionDefault {
    fn contig(&self) -> Box<dyn Contig> {

    }

    fn strand(&self) -> &Strand {

    }
}

// impl Region for GenomicRegionDefault {
//
// }
