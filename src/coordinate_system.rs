use crate::Bound;

pub enum CoordinateSystem {
    FullyClosed,
    LeftOpen,
    RightOpen,
    FullyOpen
}

impl CoordinateSystem {
    fn value(&self) -> (Bound, Bound) {
        match self {
            CoordinateSystem::FullyClosed => (Bound::CLOSED, Bound::CLOSED),
            CoordinateSystem::LeftOpen => (Bound::OPEN, Bound::CLOSED),
            CoordinateSystem::RightOpen => (Bound::CLOSED, Bound::OPEN),
            CoordinateSystem::FullyOpen => (Bound::OPEN, Bound::OPEN)
        }
    }
 
    pub fn one_based() -> CoordinateSystem {
        CoordinateSystem::FullyClosed
    }

    pub fn zero_based() -> CoordinateSystem {
        CoordinateSystem::LeftOpen
    }

    pub fn is_one_based(&self) -> bool {
        match self {
            CoordinateSystem::FullyClosed => true,
            _ => false
        }
    }

    pub fn is_zero_based(&self) -> bool {
        match self {
            CoordinateSystem::LeftOpen => true,
            _ =>  false
        }
    }

    pub fn start_bound(&self) -> Bound {
        self.value().0
    }

    pub fn end_bound(&self) -> Bound {
        self.value().1
    }

    pub fn start_delta(&self, target: CoordinateSystem) -> i8 {
        if target.start_bound() == self.start_bound(){
            return 0;
        }
        match self.start_bound() {
            Bound::OPEN => 1,
            _ => -1
        }
    }

    pub fn end_delta(&self, target: CoordinateSystem) -> i8 {
        if target.start_bound() == self.start_bound(){
            return 0;
        }
        match self.start_bound() {
            Bound::OPEN => 1,
            _ => {
                -1
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(CoordinateSystem::FullyClosed, true)]
    #[case(CoordinateSystem::LeftOpen, false)]
    fn test_is_one_based(#[case] input: CoordinateSystem, #[case] expected: bool) {
        assert_eq!(input.is_one_based(), expected);
    }
    #[rstest]
    #[case(CoordinateSystem::FullyClosed, false)]
    #[case(CoordinateSystem::LeftOpen, true)]
    fn test_is_zero_base(#[case] input: CoordinateSystem, #[case] expected: bool) {
        assert_eq!(input.is_zero_based(), expected);
    }

    #[rstest]
    #[case(CoordinateSystem::FullyClosed, Bound::CLOSED)]
    #[case(CoordinateSystem::LeftOpen, Bound::OPEN)]
    #[case(CoordinateSystem::FullyOpen, Bound::OPEN)]
    fn test_start_bound(#[case] input: CoordinateSystem, #[case] expected: Bound){
        assert_eq!(input.start_bound(), expected)
    }

    #[rstest]
    #[case(CoordinateSystem::FullyClosed, Bound::CLOSED)]
    #[case(CoordinateSystem::LeftOpen, Bound::CLOSED)]
    #[case(CoordinateSystem::FullyOpen, Bound::OPEN)]
    fn test_end_bound(#[case] input: CoordinateSystem, #[case] expected: Bound){
        assert_eq!(input.end_bound(), expected)
    }

    #[rstest]
    #[case(CoordinateSystem::FullyClosed, CoordinateSystem::FullyClosed, 0)]
    #[case(CoordinateSystem::FullyClosed, CoordinateSystem::LeftOpen, -1)]
    #[case(CoordinateSystem::FullyClosed, CoordinateSystem::FullyOpen, -1)]
    #[case(CoordinateSystem::LeftOpen, CoordinateSystem::LeftOpen, 0)]
    #[case(CoordinateSystem::LeftOpen, CoordinateSystem::FullyClosed, 1)]
    fn test_start_delta(#[case] current: CoordinateSystem, #[case] target: CoordinateSystem, #[case] expected: i8){
        assert_eq!(current.start_delta(target), expected)
    }


}