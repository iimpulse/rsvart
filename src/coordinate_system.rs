use crate::Bound;
use crate::CoordinateSystem::{OneBased, ZeroBased};

const LEFT_OPEN_CS: CoordinateSystem = ZeroBased;
const FULLY_CLOSED_CS: CoordinateSystem = OneBased;

#[derive(Debug, PartialEq)]
pub enum CoordinateSystem {
    OneBased,
    ZeroBased
}

impl CoordinateSystem {
    fn value(&self) -> &(Bound, Bound) {
        match self {
            OneBased => &(Bound::CLOSED, Bound::CLOSED),
            ZeroBased => &(Bound::OPEN, Bound::CLOSED)
        }
    }

    pub fn zero_based() -> &'static CoordinateSystem {
        &LEFT_OPEN_CS
    }

    pub fn one_based() -> &'static CoordinateSystem {
        &FULLY_CLOSED_CS
    }

    pub fn is_one_based(&self) -> bool {
        self.eq(&OneBased)
    }

    pub fn is_zero_based(&self) -> bool {
        self.eq(&ZeroBased)
    }

    pub fn start_bound(&self) -> &Bound {
        &self.value().0
    }

    pub fn end_bound(&self) -> &Bound {
        &self.value().1
    }

    pub fn start_delta(&self, target: &CoordinateSystem) -> i8 {
        if self.eq(target) {
            return 0;
        }

        match *self {
            ZeroBased => 1,
            _ => -1
        }
    }

    pub fn length_delta(cs: &CoordinateSystem) -> i8 {
        ZeroBased.start_delta(cs)
    }

    pub fn value_of(cs: &CoordinateSystem) -> Self {
        match cs {
            OneBased => OneBased,
            ZeroBased => ZeroBased
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(CoordinateSystem::OneBased, true)]
    #[case(CoordinateSystem::ZeroBased, false)]
    fn test_is_one_based(#[case] input: CoordinateSystem,
                         #[case] expected: bool) {
        assert_eq!(input.is_one_based(), expected);
    }

    #[rstest]
    #[case(CoordinateSystem::OneBased, false)]
    #[case(CoordinateSystem::ZeroBased, true)]
    fn test_is_zero_based(#[case] input: CoordinateSystem,
                          #[case] expected: bool) {
        assert_eq!(input.is_zero_based(), expected);
    }

    #[rstest]
    #[case(CoordinateSystem::OneBased, & Bound::CLOSED)]
    #[case(CoordinateSystem::ZeroBased, & Bound::OPEN)]
    fn test_start_bound(#[case] input: CoordinateSystem,
                        #[case] expected: &Bound) {
        assert_eq!(input.start_bound(), expected)
    }

    #[rstest]
    #[case(CoordinateSystem::OneBased, & Bound::CLOSED)]
    #[case(CoordinateSystem::ZeroBased, & Bound::CLOSED)]
    fn test_end_bound(#[case] input: CoordinateSystem,
                      #[case] expected: &Bound) {
        assert_eq!(input.end_bound(), expected)
    }

    #[rstest]
    #[case(CoordinateSystem::OneBased, CoordinateSystem::OneBased, 0)]
    #[case(CoordinateSystem::OneBased, CoordinateSystem::ZeroBased, -1)]
    #[case(CoordinateSystem::ZeroBased, CoordinateSystem::ZeroBased, 0)]
    #[case(CoordinateSystem::ZeroBased, CoordinateSystem::OneBased, 1)]
    fn test_start_delta(#[case] current: CoordinateSystem,
                        #[case] target: CoordinateSystem,
                        #[case] expected: i8){
        assert_eq!(current.start_delta(&target), expected)
    }

}
