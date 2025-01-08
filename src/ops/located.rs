pub trait Located<U> {
    fn start(&self) -> &U;

    fn end(&self) -> &U;

    fn coordinates(&self) -> (&U, &U) {
        (self.start(), self.end())
    }
}

#[cfg(test)]
mod test {
    use crate::ops::located::Located;
    use rstest::rstest;

    struct TestRegion<C> {
        start: C,
        end: C
    }

    impl<C> Located<C> for TestRegion<C> {
        fn start(&self) -> &C {
            &self.start
        }

        fn end(&self) -> &C{
            &self.end
        }
    }


    #[rstest]
    #[case(4, 20)]
    #[case(10, 20)]
    fn test_with_coordinate_system(#[case] start: u8,
                                   #[case] end: u8){
        let region = TestRegion { start, end };
        assert_eq!(region.start, start);
        assert_eq!(region.end, end);
        assert_eq!(region.coordinates(), (&start, &end));
    }
}