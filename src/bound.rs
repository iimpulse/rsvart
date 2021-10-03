#[derive(Debug,PartialEq)]
pub enum Bound {
    OPEN,
    CLOSED
}
impl Bound {
    pub fn is_open(&self) -> bool {
        match self {
            Bound::OPEN => {
                true
            },
            Bound::CLOSED => {
                false
            }
        }
    }

    pub fn is_closed(&self) -> bool {
        match self {
            Bound::OPEN => {
                false
            },
            Bound::CLOSED => {
                true
            }
        }
    }
}