pub trait Contains<O = Self> where O: ?Sized,
{
    fn contains(&self, other: &O) -> bool;
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

    impl Contains for TestRegion {
        fn contains(&self, other: &TestRegion) -> bool {
            self.start() <= other.start() && other.end() <= self.end()
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