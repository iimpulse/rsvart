use crate::{Contains, Contig, Contiged, GenomicRegioned };
use crate::{Located, Operations, Overlaps, Strand, Stranded, Unit};
use crate::{ops::func::contains, ops::func::overlaps};

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

impl<'g, C> Overlaps for GenomicRegion<'g, C>
where
    C: Unit,
{
    fn overlaps(&self, other: &Self) -> bool {
        if self.contig.eq(other.contig) {
            overlaps(&self.start, &self.end, &other.start, &other.end)
        } else {
            false
        }
    }
}

impl<'g, C> Contains for GenomicRegion<'g, C>
where
    C: Unit,
{
    fn contains(&self, other: &Self) -> bool {
        if self.contig.eq(other.contig) {
            contains(&self.start, &self.end, &other.start, &other.end)
        } else {
            false
        }
    }
}

impl<'g, C> Operations<C> for GenomicRegion<'g, C> where C: Unit, {}
impl<'g, C> GenomicRegioned<C> for GenomicRegion<'g, C> where C: Unit {}



#[cfg(test)]
mod test {
    use std::convert::TryFrom;
    use super::Strand;
    use rstest::rstest;
    use crate::SvartError;
    
    


}