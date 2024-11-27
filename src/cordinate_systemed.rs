use crate::CoordinateSystem;

pub trait CoordinateSystemed {
    fn coordinate_system(&self) -> CoordinateSystem;
}