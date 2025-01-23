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
