use crate::{ConfidenceInterval, Contig, CoordinateSystem, SequenceRole, AssignedMoleculeType};
use crate::region::Region;
pub struct RegionPrecise {
    start: u32,
    end: u32,
    coordinate_system: CoordinateSystem,
}

impl RegionPrecise {
    pub fn of(coordinate_system: CoordinateSystem, start: u32, end: u32) -> RegionPrecise {
        RegionPrecise {
            start,
            end,
            coordinate_system,
        }
    }
}

impl Region for RegionPrecise {
    fn start(&self) -> u32 {
        self.start
    }

    fn end(&self) -> u32 {
        self.end
    }

    fn coordinate_system(&self) -> &CoordinateSystem {
        &self.coordinate_system
    }

    // The start of the coordinate system is always precise.
    // We are confident that the start is always the start.
    fn start_confidence_interval(&self) -> &ConfidenceInterval {
        ConfidenceInterval::precise()
    }

    fn end_confidence_interval(&self) -> &ConfidenceInterval {
        ConfidenceInterval::precise()
    }

    fn with_coordinate_system(&mut self, cs: &CoordinateSystem) {
        self.start = self.start.wrapping_add(self.coordinate_system.start_delta(cs) as u32);
        self.coordinate_system = CoordinateSystem::from(*cs);
    }

    fn invert(&mut self, contig: &Contig) {
        // TODO - centralize the code for coordinate inversion
        let cache = self.start;
        self.start = contig.length().wrapping_add(CoordinateSystem::start_delta(&self.coordinate_system, &self.coordinate_system) as usize) as u32 - self.end;
        self.end = contig.length().wrapping_add(CoordinateSystem::end_delta(&self.coordinate_system, &self.coordinate_system) as usize) as u32 - cache;
    }
}


#[cfg(test)]
mod test {
    use rstest::rstest;

    use crate::{CoordinateSystem};

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
        let mut region = RegionPrecise::of(cs, start, end);
        region.with_coordinate_system(&target);
        assert_eq!(region.start, expected_start);
        assert_eq!(region.end, expected_end);
    }

    #[rstest]
    #[case(CoordinateSystem::zero_based(), 10, 20, 80, 90)]
    #[case(CoordinateSystem::one_based(), 11, 20, 81, 90)]
    fn test_invert(#[case] cs: CoordinateSystem,
                   #[case] start: u32,
                   #[case] end: u32,
                   // ------------------------------------------------------------------------------
                   #[case] expected_start: u32,
                   #[case] expected_end: u32) {
        let contig = Contig::of(1, "1".to_string(),
                                       SequenceRole::AssembledMolecule, "1".to_string(),
                                       AssignedMoleculeType::Chromosome, 100,
                                       "".to_string(), "".to_string(),
                                       "chr1".to_string()).unwrap();
        let mut region = RegionPrecise::of(cs, start, end);
        region.invert(&contig);
        assert_eq!(region.start, expected_start);
        assert_eq!(region.end, expected_end);
    }
}

