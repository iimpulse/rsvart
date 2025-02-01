use crate::{Contiged, Located, Stranded, Unit};
use crate::ops::func::{contains};
use crate::ops::transposable::Transposable;

pub trait Contains<C, O = Self> where O: ?Sized,
{
    fn contains(&self, other: &O) -> bool;
}

impl<C, T> Contains<C> for T where C: Unit, T: Located<C> {
    fn contains(&self, other: &Self) -> bool {
        contains(self.start(), self.end(), other.start(), other.end())
    }
}


pub trait GenomicallyContains<C, O = Self> where O: ?Sized,
{
    fn contains(&self, other: &O) -> bool;
}

impl<C, T> GenomicallyContains<C> for T where C: Unit, T: Located<C> + Stranded + Contiged<C> {
    fn contains(&self, other: &Self) -> bool {
        if self.contig().ne(other.contig()) {
            return false
        }

        if self.strand().eq(&other.strand()) {
            return contains(self.start(), self.end(), other.start(), other.end())
        }

        let other_start = other.start_on_strand(self.strand());
        let other_end = other.end_on_strand(self.strand());
        contains(self.start(), self.end(), &other_start, &other_end)
    }
}

#[cfg(test)]
mod test {
    use rstest::rstest;
    use crate::{Contains, Located};

    struct TestRegion {
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
    #[case(TestRegion { start: 4, end: 20 }, TestRegion { start: 5, end: 16 }, true)]
    #[case(TestRegion { start: 4, end: 20 }, TestRegion { start: 20, end: 29 }, false)]
    fn test_contains(#[case] region_1: TestRegion,
                     #[case] region_2: TestRegion, #[case] expected: bool){
        assert_eq!(region_1.contains(&region_2), expected);
    }
}