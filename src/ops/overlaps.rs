use crate::{Contiged, Located, Stranded, Unit};
use crate::ops::func::overlaps;
use crate::ops::transposable::Transposable;

pub trait Overlaps<C, O = Self> where O: ?Sized {
    fn overlaps(&self, other: &O) -> bool;
}

impl<C, T> Overlaps<C> for T where C: Unit, T: Located<C> {
    fn overlaps(&self, other: &Self) -> bool {
        overlaps(self.start(), self.end(), other.start(), other.end())
    }
}

pub trait GenomicallyOverlaps<C, O = Self> where O: ?Sized {
    fn overlaps(&self, other: &O) -> bool;
}

impl<C: Unit, T> GenomicallyOverlaps<C> for T where C: Unit, T: Located<C> + Stranded + Contiged<C> {
    fn overlaps(&self, other: &Self) -> bool {

        if self.contig().ne(other.contig()) {
            return false
        }

        if self.strand().eq(&other.strand()) {
           return overlaps(self.start(), self.end(), other.start(), other.end())
        }

        let other_start = other.start_on_strand(self.strand());
        let other_end = other.end_on_strand(self.strand());
        overlaps(self.start(), self.end(), &other_start, &other_end)
    }
}

#[cfg(test)]
mod test {
    use rstest::rstest;
    use crate::ops::located::Located;
    use crate::Overlaps;

    pub struct TestRegion {
        start: u8,
        end: u8
    }

    impl Located<u8> for TestRegion {
        fn start(&self) -> &u8 {
            &self.start
        }

        fn end(&self) -> &u8 { 
            &self.end
        }
    }

    #[rstest]
    #[case(TestRegion { start: 4, end: 20 }, TestRegion { start: 1, end: 5 }, true)]
    #[case(TestRegion { start: 4, end: 20 }, TestRegion { start: 20, end: 29 }, false)]
    fn test_overlaps(#[case] region_1: TestRegion,
                     #[case] region_2: TestRegion, #[case] expected: bool){
        assert_eq!(region_1.overlaps(&region_2), expected);
    }
}