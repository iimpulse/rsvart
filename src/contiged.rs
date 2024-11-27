use std::rc::Rc;
use crate::Contig;

pub trait Contiged {
    fn contig(&self) -> &Contig;
}