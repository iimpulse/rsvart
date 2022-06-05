use crate::{ConfidenceInterval, Contig, CoordinateSystem};
use crate::CoordinateSystem::{OneBased, ZeroBased};
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
            self.coordinate_system = CoordinateSystem::value_of(cs);
        }
    }

    fn invert(&mut self, contig: &dyn Contig) {
        // TODO - centralize the code for coordinate inversion
        let cache = self.start;
        self.start = contig.length().wrapping_add(CoordinateSystem::length_delta(&self.coordinate_system) as usize) as u32 - self.end;
        self.end = contig.length().wrapping_add(CoordinateSystem::length_delta(&self.coordinate_system) as usize) as u32 - cache;
        ConfidenceInterval::swap_and_invert(&mut self.start_ci, &mut self.end_ci)
    }
}

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
        self.coordinate_system = CoordinateSystem::value_of(cs);
    }

    fn invert(&mut self, contig: &dyn Contig) {
        // TODO - centralize the code for coordinate inversion
        let cache = self.start;
        self.start = contig.length().wrapping_add(CoordinateSystem::length_delta(&self.coordinate_system) as usize) as u32 - self.end;
        self.end = contig.length().wrapping_add(CoordinateSystem::length_delta(&self.coordinate_system) as usize) as u32 - cache;
    }
}


#[cfg(test)]
mod test {
    use rstest::rstest;

    use crate::{AssignedMoleculeType, CoordinateSystem, SequenceRole};
    use crate::confidence_interval::ConfidenceInterval;
    use crate::default::ContigDefault;

    use super::*;

    #[rstest]
    #[case(10, 20, CoordinateSystem::ZeroBased, CoordinateSystem::ZeroBased, 10)]
    #[case(10, 20, CoordinateSystem::ZeroBased, CoordinateSystem::OneBased, 11)]
    #[case(11, 20, CoordinateSystem::OneBased, CoordinateSystem::ZeroBased, 10)]
    #[case(11, 20, CoordinateSystem::OneBased, CoordinateSystem::OneBased, 11)]
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
    #[case(10, 20, CoordinateSystem::ZeroBased, CoordinateSystem::ZeroBased, 20)]
    #[case(10, 20, CoordinateSystem::ZeroBased, CoordinateSystem::OneBased, 20)]
    #[case(11, 20, CoordinateSystem::OneBased, CoordinateSystem::ZeroBased, 20)]
    #[case(11, 20, CoordinateSystem::OneBased, CoordinateSystem::OneBased, 20)]
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

    #[rstest]
    #[case(10, 20, CoordinateSystem::ZeroBased, CoordinateSystem::OneBased, 11, 20)]
    #[case(10, 20, CoordinateSystem::ZeroBased, CoordinateSystem::ZeroBased, 10, 20)]
    #[case(11, 20, CoordinateSystem::OneBased, CoordinateSystem::OneBased, 11, 20)]
    #[case(11, 20, CoordinateSystem::OneBased, CoordinateSystem::ZeroBased, 10, 20)]
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
    #[case(CoordinateSystem::ZeroBased, 10, 20, 80, 90)]
    #[case(CoordinateSystem::OneBased, 11, 20, 81, 90)]
    fn test_invert(#[case] cs: CoordinateSystem,
                   #[case] start: u32,
                   #[case] end: u32,
                   // ------------------------------------------------------------------------------
                   #[case] expected_start: u32,
                   #[case] expected_end: u32, ) {
        let contig = ContigDefault::of(1, "1".to_string(),
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