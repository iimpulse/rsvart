use crate::{Located, Operations};

pub trait Regioned<C>: Located<C> + Operations<C>  {}