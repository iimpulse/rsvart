#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Bound {
    OPEN,
    CLOSED
}
impl Bound {
    pub fn is_open(&self) -> bool {
        *self == Bound::OPEN
    }

    pub fn is_closed(&self) -> bool {
        *self == Bound::CLOSED
    }
}