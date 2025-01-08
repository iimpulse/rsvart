use std::ops::Sub;

pub trait Unit: Sub<Output = Self> + Ord + Copy {
    fn is_zero(&self) -> bool;

    fn zero() -> Self;

    fn one() -> Self;
}

macro_rules! impl_unit {
    ($TYPE:ty) => {
        impl Unit for $TYPE {
            fn is_zero(&self) -> bool {
                *self == 0
            }

            fn zero() -> Self {
                0
            }

            fn one() -> Self {
                1
            }
        }
    };
}

impl_unit!(u8);
impl_unit!(u16);
impl_unit!(u32);
impl_unit!(u64);
impl_unit!(u128);
impl_unit!(usize);


#[cfg(test)]
mod test {
    use rstest::rstest;
    use crate::ops::unit::Unit;

    #[rstest]
    #[case(4, 0)]
    #[case(10, 0)]
    fn test_with_coordinate_system(#[case] start: u8,
                                   #[case] end: u8){
        assert_eq!(start.is_zero(), false);
        assert_eq!(end.is_zero(), true);
        assert_eq!(u8::zero(), 0);
        assert_eq!(u8::one(), 1);
    }
}