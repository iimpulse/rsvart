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

    // TODO - this is possible, but it introduces an array of lifetimes everywhere
    fn contig(&self) -> &dyn Contig;

    fn strand(&self) -> &Strand;

    fn contig_id(&self) -> usize {
        self.contig().id()
    }

    fn contig_name(&self) -> &str {
        self.contig().name()
    }

    fn with_strand(&mut self, strand: &Strand);

    fn with_positive_strand(&mut self) {
        self.with_strand(&Strand::Positive)
    }

    fn with_negative_strand(&mut self) {
        self.with_strand(&Strand::Negative)
    }

}

