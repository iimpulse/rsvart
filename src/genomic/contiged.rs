use crate::ops::Located;

pub trait Contiged<C> {
    type Contig: Located<C> + Eq;

    fn contig(&self) -> &Self::Contig;
}
