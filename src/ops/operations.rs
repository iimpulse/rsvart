use crate::{Overlaps, Contains};

pub trait Operations<C>: Overlaps + Contains {}