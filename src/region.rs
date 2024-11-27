use crate::{CoordinateSystem};
use crate::coordinates::Coordinates;

pub struct Region {
    coordinates: Coordinates
}

impl Region {
    pub fn new(coordinates: Coordinates) -> Self {
        Self { coordinates }
    }
}


#[cfg(test)]
mod test {
    use rstest::rstest;
    use crate::{AssignedMoleculeType, Contig, SequenceRole};
    use super::*;   
    // TODO: implement imprecise trait
    // #[rstest]
    // #[case(10, 20, CoordinateSystem::zero_based(), CoordinateSystem::zero_based(), 10)]
    // #[case(10, 20, CoordinateSystem::zero_based(), CoordinateSystem::one_based(), 11)]
    // #[case(11, 20, CoordinateSystem::one_based(), CoordinateSystem::zero_based(), 10)]
    // #[case(11, 20, CoordinateSystem::one_based(), CoordinateSystem::one_based(), 11)]
    // fn test_start_with_coordinate_system(#[case] start: u32,
    //                                      #[case] end: u32,
    //                                      #[case] cs: CoordinateSystem,
    //                                      #[case] target: CoordinateSystem,
    //                                      #[case] expected: u32) {
    //     let start_ci: ConfidenceInterval = ConfidenceInterval::imprecise(10, 5);
    //     let end_ci: ConfidenceInterval = ConfidenceInterval::imprecise(5, 50);
    //     let mut region = Region::imprecise(cs, start, start_ci, end, end_ci);
    //     assert_eq!(region.start_with_coordinate_system(&target), expected);
    // }

    // #[rstest]
    // #[case(10, 20, CoordinateSystem::zero_based(), CoordinateSystem::zero_based(), 20)]
    // #[case(10, 20, CoordinateSystem::zero_based(), CoordinateSystem::one_based(), 20)]
    // #[case(11, 20, CoordinateSystem::one_based(), CoordinateSystem::zero_based(), 20)]
    // #[case(11, 20, CoordinateSystem::one_based(), CoordinateSystem::one_based(), 20)]
    // fn test_end_with_coordinate_system(#[case] start: u32,
    //                                    #[case] end: u32,
    //                                    #[case] cs: CoordinateSystem,
    //                                    #[case] target: CoordinateSystem,
    //                                    #[case] expected: u32) {
    //     let start_ci: ConfidenceInterval = ConfidenceInterval::imprecise(10, 5);
    //     let end_ci: ConfidenceInterval = ConfidenceInterval::imprecise(5, 50);
    //     let mut region = Region::imprecise(cs, start, start_ci, end, end_ci);
    //     assert_eq!(region.end_with_coordinate_system(&target), expected);
    // }



}