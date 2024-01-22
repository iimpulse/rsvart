use crate::Bound;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum CoordinateSystem {
    FullyClosed,
    LeftOpen,
    RightOpen,
    FullyOpen
}

impl CoordinateSystem {
    fn value(&self) -> (Bound, Bound) {
        match *self {
            CoordinateSystem::FullyClosed => (Bound::Closed, Bound::Closed),
            CoordinateSystem::LeftOpen => (Bound::Open, Bound::Closed),
            CoordinateSystem::RightOpen => (Bound::Closed, Bound::Open),
            CoordinateSystem::FullyOpen => (Bound::Open, Bound::Open),
        }
    }

    pub fn zero_based() -> CoordinateSystem {
        CoordinateSystem::LeftOpen
    }

    pub fn one_based() -> CoordinateSystem {
        CoordinateSystem::FullyClosed
    }

    pub fn is_one_based(&self) -> bool {
        *self == CoordinateSystem::FullyClosed
    }

    pub fn is_zero_based(&self) -> bool {
        *self == CoordinateSystem::LeftOpen
    }

    pub fn start_bound(&self) -> Bound {
        self.value().0
    }

    pub fn end_bound(&self) -> Bound {
        self.value().1
    }

    pub fn start_delta(&self, target: &CoordinateSystem) -> i8 {
        if target.start_bound() == self.start_bound() {
            return 0;
        }
        match self.start_bound() {
            Bound::Open => 1,
            _ => -1
        }
    }

    pub fn end_delta(&self, target: &CoordinateSystem) -> i8 {
        if target.end_bound() == self.end_bound() {
            0
        } else {
            match self.end_bound() {
                Bound::Open => -1,
                _ => 1
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::rstest;

    #[rstest]
    fn test_baseness(){
        assert_eq!(CoordinateSystem::LeftOpen, CoordinateSystem::zero_based());
        assert_eq!(CoordinateSystem::FullyClosed, CoordinateSystem::one_based());
    }

    #[rstest]
    #[case(CoordinateSystem::FullyClosed, true)]
    #[case(CoordinateSystem::LeftOpen, false)]
    fn test_is_one_based(#[case] input: CoordinateSystem, #[case] expected: bool) {
        assert_eq!(input.is_one_based(), expected);
    }

    #[rstest]
    #[case(CoordinateSystem::FullyClosed, false)]
    #[case(CoordinateSystem::LeftOpen, true)]
    fn test_is_zero_based(#[case] input: CoordinateSystem, #[case] expected: bool) {
        assert_eq!(input.is_zero_based(), expected);
    }

    #[rstest]
    #[case(CoordinateSystem::FullyClosed, Bound::Closed)]
    #[case(CoordinateSystem::LeftOpen, Bound::Open)]
    #[case(CoordinateSystem::FullyOpen, Bound::Open)]
    fn test_start_bound(#[case] input: CoordinateSystem, #[case] expected: Bound) {
        assert_eq!(input.start_bound(), expected)
    }

    #[rstest]
    #[case(CoordinateSystem::FullyClosed, Bound::Closed)]
    #[case(CoordinateSystem::LeftOpen, Bound::Closed)]
    #[case(CoordinateSystem::FullyOpen, Bound::Open)]
    fn test_end_bound(#[case] input: CoordinateSystem, #[case] expected: Bound) {
        assert_eq!(input.end_bound(), expected)
    }

    #[rstest]
    #[case(CoordinateSystem::FullyClosed, CoordinateSystem::FullyClosed, 0)]
    #[case(CoordinateSystem::FullyClosed, CoordinateSystem::LeftOpen, -1)]
    #[case(CoordinateSystem::FullyClosed, CoordinateSystem::FullyOpen, -1)]
    #[case(CoordinateSystem::LeftOpen, CoordinateSystem::LeftOpen, 0)]
    #[case(CoordinateSystem::LeftOpen, CoordinateSystem::FullyClosed, 1)]
    fn test_start_delta(#[case] current: CoordinateSystem, #[case] target: CoordinateSystem, #[case] expected: i8){
        assert_eq!(current.start_delta(&target), expected)
    }

    #[rstest]
    #[case(CoordinateSystem::FullyClosed, CoordinateSystem::FullyClosed, 0)]
    #[case(CoordinateSystem::FullyClosed, CoordinateSystem::LeftOpen, 0)]
    #[case(CoordinateSystem::FullyClosed, CoordinateSystem::FullyOpen, 1)]
    #[case(CoordinateSystem::FullyClosed, CoordinateSystem::RightOpen, 1)]
    #[case(CoordinateSystem::LeftOpen, CoordinateSystem::LeftOpen, 0)]
    #[case(CoordinateSystem::LeftOpen, CoordinateSystem::FullyClosed, 0)]
    #[case(CoordinateSystem::FullyOpen, CoordinateSystem::FullyClosed, -1)]
    fn test_end_delta(#[case] current: CoordinateSystem, #[case] target: CoordinateSystem, #[case] expected: i8) {
        assert_eq!(current.end_delta(&target), expected)
    }
}
