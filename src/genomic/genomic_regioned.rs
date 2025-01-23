use crate::{Contiged, Located, Stranded, Operations};

pub trait GenomicRegioned<C>: Contiged<C> + Stranded + Located<C> + Operations<C> {}