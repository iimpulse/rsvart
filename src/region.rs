use crate::{Contains, Located, Operations, Overlaps, Unit};
use crate::regioned::Regioned;
use crate::{ops::func::contains, ops::func::overlaps};


#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Region<C> {
    start: C,
    end: C,
}

impl<C> Region<C>
where
    C: PartialOrd,
{
    pub fn new(start: C, end: C) -> Option<Self> {
        if start > end {
            None
        } else {
            Some(Self { start, end })
        }
    }
}

impl<C> Located<C> for Region<C> {
    fn start(&self) -> &C {
        &self.start
    }

    fn end(&self) -> &C {
        &self.end
    }
}

impl<C> Overlaps for Region<C>
where
    C: Unit,
{
    fn overlaps(&self, other: &Self) -> bool {
        overlaps(&self.start, &self.end, &other.start, &other.end)
    }
}

impl<C> Contains for Region<C>
where
    C: Unit,
{
    fn contains(&self, other: &Self) -> bool {
        contains(self.start(), self.end(), other.start(), other.end())
    }
}

impl<C> Operations<C> for Region<C> where C: Unit {}
impl<C> Regioned<C> for Region<C> where C: Unit {}


#[cfg(test)]
mod test {
    use rstest::rstest;
    use crate::{Contains, Located, Region, Overlaps};

    #[rstest]
    #[case(1, 5)]
    fn test_region_methods(#[case] start: u8, #[case] end: u8) {
        let region = Region::new(start, end).unwrap();
        let region_other = Region::new(2, 6).unwrap();
        assert_eq!(*region.start(), 1);
        assert_eq!(*region.end(), 5);
        let (start_coor, end_coor) = region.coordinates();
        assert_eq!(*start_coor, 1);
        assert_eq!(*end_coor, 5);
        assert_eq!(region.contains(&region_other), false);
        assert_eq!(region.overlaps(&region_other), true);
    }
}