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
        return CoordinateSystem::FullyClosed;
    }

    pub fn zero_based() -> CoordinateSystem {
        return CoordinateSystem::LeftOpen;
    }

    pub fn is_one_based(&self) -> bool {
        match self {
            CoordinateSystem::FullyClosed => {
                true
            },
            _ => {
                false
            }
        }
    }

    pub fn is_zero_based(&self) -> bool {
        match self {
            CoordinateSystem::LeftOpen => {
                true
            },
            _ => {
                false
            }
        }
    }

    pub fn start_bound(&self) -> Bound {
        self.value().0
    }

    pub fn end_bound(&self) -> Bound {
        self.value().1
    }

    /*pub fn start_delta(target: CoordinateSystem) -> i32 {
        return Option.empty()
    }*/

    /*pub fn end_delta(target: CoordinateSystem) -> i32 {
        return Option.empty()
    }*/
}

mod test {
    use super::*;

    #[rstest]
    #[case(FullyClosed, true)]
    fn test_is_one_based(#[case] input: CoordinateSystem, #[case] expected: bool) {
        assert_eq!(input.is_one_based(), expected);
    }
}