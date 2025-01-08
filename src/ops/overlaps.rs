pub trait Overlaps<O = Self> where O: ?Sized {
    fn overlaps(&self, other: &O) -> bool;
}

#[cfg(test)]
mod test {
    use rstest::rstest;
    use crate::ops::located::Located;
    use crate::ops::spanning::Spanning;
    use crate::Overlaps;

    pub struct TestRegion {
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

    impl Overlaps for TestRegion {
        fn overlaps(&self, other: &TestRegion) -> bool {
            if self.is_empty() && other.is_empty() {
                self.start() == other.end() && other.start() == self.end()
            } else {
                self.start() < other.end() && other.start() < self.end()
            }
        }
    }


    #[rstest]
    #[case(TestRegion { start: 4, end: 20 }, TestRegion { start: 1, end: 5 }, true)]
    #[case(TestRegion { start: 4, end: 20 }, TestRegion { start: 20, end: 29 }, false)]
    fn test_contains(#[case] region_1: TestRegion,
                     #[case] region_2: TestRegion, #[case] expected: bool){
        assert_eq!(region_1.overlaps(&region_2), expected);
    }
}