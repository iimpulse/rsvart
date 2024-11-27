use crate::{Contig, CoordinateSystem, Region};
use crate::cordinate_systemed::CoordinateSystemed;

pub struct Coordinates {
    start: u32,
    end: u32,
    coordinate_system: CoordinateSystem
}
impl Coordinates {

    pub fn new(start: u32, end: u32, coordinate_system: CoordinateSystem) -> Self {
        Self { start, end, coordinate_system }
    }

    pub fn start(&self) -> u32 {
        self.start
    }

    pub fn end(&self) -> u32 {
        self.end
    }

    pub fn with_coordinate_system(&mut self, cs: CoordinateSystem) {
        self.start = self.start().wrapping_add(self.coordinate_system.start_delta(cs) as u32);
        self.coordinate_system = cs
    }

    pub fn start_with_coordinate_system(&self, cs: CoordinateSystem) -> u32 {
        let delta = self.coordinate_system().start_delta(cs);
        match delta {
            -1 => self.start() - 1,
            0 => self.start(),
            1 => self.start() + 1,
            _ => panic!("Whoa!")
        }
    }

    pub fn end_with_coordinate_system(&self, _cs: CoordinateSystem) -> u32 {
        self.end()
    }

    pub fn contains(&self, other: &Coordinates) -> bool {
        self.start() <= other.start_with_coordinate_system(self.coordinate_system())
            && other.end_with_coordinate_system(self.coordinate_system()) <= self.end()
    }

    pub fn overlaps(&self, other: &Region) -> bool {
        return false
    }

    pub fn invert(&self, contig: &Contig){

    }

    // fn invert_coordinates(coordinate_system: CoordinateSystem, contig: Contig, pos: u32) -> u32 {
    //     return contig.length() + Self::length_delta(coordinate_system) - pos;
    // }

    fn length_delta(coordinate_system: CoordinateSystem) -> i8 {
        match coordinate_system.is_zero_based() {
            true => 0,
            false => CoordinateSystem::zero_based().start_delta(coordinate_system),
            _ => panic!("Coordinate System is_zero_based is broken.")
        }
    }
}

impl CoordinateSystemed for Coordinates {
    fn coordinate_system(&self) -> CoordinateSystem {
        self.coordinate_system
    }
}

#[cfg(test)]
mod test {
    use rstest::rstest;
    use super::*;

    #[rstest]
    #[case(10, 20, CoordinateSystem::zero_based(), CoordinateSystem::one_based(), 11, 20)]
    #[case(10, 20, CoordinateSystem::zero_based(), CoordinateSystem::zero_based(), 10, 20)]
    #[case(11, 20, CoordinateSystem::one_based(), CoordinateSystem::one_based(), 11, 20)]
    #[case(11, 20, CoordinateSystem::one_based(), CoordinateSystem::zero_based(), 10, 20)]
    fn test_with_coordinate_system(#[case] start: u32,
                                   #[case] end: u32,
                                   #[case] cs: CoordinateSystem,
                                   #[case] target: CoordinateSystem,
                                   #[case] expected_start: u32,
                                   #[case] expected_end: u32, ) {
        let mut region = Coordinates::new(start, end, cs);
        region.with_coordinate_system(target);
        assert_eq!(region.start, expected_start);
        assert_eq!(region.end, expected_end);
    }
}