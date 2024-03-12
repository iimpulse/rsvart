use crate::{ConfidenceInterval, Contig, CoordinateSystem};
use crate::region::Region;
pub struct RegionImprecise {
    start: u32,
    end: u32,
    coordinate_system: CoordinateSystem,
    start_ci: ConfidenceInterval,
    end_ci: ConfidenceInterval,
}

impl RegionImprecise {
    pub fn of(coordinate_system: CoordinateSystem,
              start: u32,
              start_ci: ConfidenceInterval,
              end: u32,
              end_ci: ConfidenceInterval) -> Self {
        RegionImprecise { start, end, coordinate_system, start_ci, end_ci }
    }
}

impl Region for RegionImprecise {
    fn start(&self) -> u32 {
        self.start
    }

    fn end(&self) -> u32 {
        self.end
    }

    fn coordinate_system(&self) -> &CoordinateSystem {
        &self.coordinate_system
    }

    fn start_confidence_interval(&self) -> &ConfidenceInterval {
        &self.start_ci
    }

    fn end_confidence_interval(&self) -> &ConfidenceInterval {
        &self.end_ci
    }

    fn with_coordinate_system(&mut self, cs: &CoordinateSystem) {
        if !self.coordinate_system.eq(cs) {
            self.start = self.start.wrapping_add(self.coordinate_system.start_delta(cs) as u32);
            self.coordinate_system = CoordinateSystem::from(*cs);
        }
    }

    fn invert(&mut self, contig: &Contig) {
        // TODO - centralize the code for coordinate inversion
        let cache = self.start;
        self.start = contig.length().wrapping_add(CoordinateSystem::start_delta(&self.coordinate_system, &self.coordinate_system) as usize) as u32 - self.end;
        self.end = contig.length().wrapping_add(CoordinateSystem::end_delta(&self.coordinate_system, &self.coordinate_system)as usize) as u32 - cache;
        ConfidenceInterval::swap_and_invert(&mut self.start_ci, &mut self.end_ci)
    }
}


#[cfg(test)]
mod test {
    use rstest::rstest;

    use crate::{CoordinateSystem};
    use crate::confidence_interval::ConfidenceInterval;

    use super::*;

    #[rstest]
    #[case(10, 20, CoordinateSystem::zero_based(), CoordinateSystem::zero_based(), 10)]
    #[case(10, 20, CoordinateSystem::zero_based(), CoordinateSystem::zero_based(), 11)]
    #[case(11, 20, CoordinateSystem::one_based(), CoordinateSystem::zero_based(), 10)]
    #[case(11, 20, CoordinateSystem::one_based(), CoordinateSystem::one_based(), 11)]
    fn test_start_with_coordinate_system(#[case] start: u32,
                                         #[case] end: u32,
                                         #[case] cs: CoordinateSystem,
                                         #[case] target: CoordinateSystem,
                                         #[case] expected: u32) {
        let start_ci: ConfidenceInterval = ConfidenceInterval::imprecise(10, 5);
        let end_ci: ConfidenceInterval = ConfidenceInterval::imprecise(5, 50);
        let region = RegionImprecise::of(cs, start, start_ci, end, end_ci);
        assert_eq!(region.start_with_coordinate_system(&target), expected);
    }

    #[rstest]
    #[case(10, 20, CoordinateSystem::zero_based(), CoordinateSystem::zero_based(), 20)]
    #[case(10, 20, CoordinateSystem::zero_based(), CoordinateSystem::one_based(), 20)]
    #[case(11, 20, CoordinateSystem::one_based(), CoordinateSystem::zero_based(), 20)]
    #[case(11, 20, CoordinateSystem::one_based(), CoordinateSystem::one_based(), 20)]
    fn test_end_with_coordinate_system(#[case] start: u32,
                                       #[case] end: u32,
                                       #[case] cs: CoordinateSystem,
                                       #[case] target: CoordinateSystem,
                                       #[case] expected: u32) {
        let start_ci: ConfidenceInterval = ConfidenceInterval::imprecise(10, 5);
        let end_ci: ConfidenceInterval = ConfidenceInterval::imprecise(5, 50);
        let region = RegionImprecise::of(cs, start, start_ci, end, end_ci);
        assert_eq!(region.end_with_coordinate_system(&target), expected);
    }
}