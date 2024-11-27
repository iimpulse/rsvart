use crate::{Region};

pub trait Regioned {
    fn region(&self) -> &Region;
}