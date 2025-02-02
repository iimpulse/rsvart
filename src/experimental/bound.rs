#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Bound {
    Open,
    Closed
}
impl Bound {
    pub fn is_open(&self) -> bool {
        *self == Bound::Open
    }

    pub fn is_closed(&self) -> bool {
        *self == Bound::Closed
    }
}

#[cfg(test)]
mod test {
    use rstest::rstest;
    use super::Bound;

    #[rstest]
    #[case(Bound::Open, true)]
    #[case(Bound::Closed, false)]
    fn test_is_open(#[case] input: Bound, #[case] expected: bool) {
        assert_eq!(input.is_open(), expected);
    }

    #[rstest]
    #[case(Bound::Open, false)]
    #[case(Bound::Closed, true)]
    fn test_is_closed(#[case] input: Bound, #[case] expected: bool) {
        assert_eq!(input.is_closed(), expected);
    }
}