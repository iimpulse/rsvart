use crate::{Located, Unit};

pub trait Spanning<C> {

    fn span(&self) -> C;

    fn is_empty(&self) -> bool;
}
impl<C, T> Spanning<C> for T
where
    C: Unit,
    T: Located<C>,
{
    fn span(&self) -> C {
        *self.end() - *self.start()
    }

    fn is_empty(&self) -> bool {
        self.span().is_zero()
    }
}


#[cfg(test)]
mod test {
    use crate::ops::located::Located;
    use crate::ops::spanning::Spanning;
    use rstest::rstest;


    struct TestRegion {
        start: u8,
        end: u8
    }

    impl Located<u8> for TestRegion {
        fn start(&self) -> &u8 {
            &self.start
        }

        fn end(&self) -> &u8{
            &self.end
        }
    }

    #[rstest]
    #[case(4, 20, 16)]
    #[case(10, 20, 10)]
    fn test_with_coordinate_system(#[case] start: u8,
                                   #[case] end: u8, #[case] expected: u8){
        let region = TestRegion { start, end };
        assert_eq!(region.start, start);
        assert_eq!(region.end, end);
        assert_eq!(region.span(), expected);
    }
}