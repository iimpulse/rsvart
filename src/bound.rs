#[derive(Debug, PartialEq)]
pub enum Bound {
    OPEN,
    CLOSED
}
impl Bound {
    pub fn is_open(&self) -> bool {
        // TODO - simple equals?
        self == &Bound::OPEN
        // self.eq(&Bound::OPEN)
        // match self {
        //     Bound::OPEN => {
        //         true
        //     },
        //     Bound::CLOSED => {
        //         false
        //     }
        // }
    }

    pub fn is_closed(&self) -> bool {
        // TODO - simple equals?
        self == &Bound::CLOSED
        // match self {
        //     Bound::OPEN => {
        //         false
        //     },
        //     Bound::CLOSED => {
        //         true
        //     }
        // }
    }
}