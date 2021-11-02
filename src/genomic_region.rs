use super::coordinate_system::CoordinateSystem;
use super::confidence_interval::ConfidenceInterval;

pub trait Region {
    fn start(&self) -> u32;
    fn end(&self) -> u32;
    fn coordinate_system(&self) -> &CoordinateSystem;
    fn start_confidence_interval(&self) -> &ConfidenceInterval;
    fn end_confidence_interval(&self) -> &ConfidenceInterval;

    fn start_with_coordinate_system(&self, cs: &CoordinateSystem) -> u32 {
        let delta = self.coordinate_system().start_delta(cs);
        match delta {
            -1 => self.start() - 1,
            0 => self.start(),
            1 => self.start() + 1,
            _ => panic!("Whoa!")
        }
    }

    fn end_with_coordinate_system(&self, cs: &CoordinateSystem) -> u32 {
        let delta = self.coordinate_system().end_delta(cs);
        match delta {
            -1 => self.end() - 1,
            0 => self.end(),
            1 => self.end() + 1,
            _ => panic!("Whoa!")
        }
    }

    fn contains(&self, other: &dyn Region) -> bool {
        self.start() <= other.start_with_coordinate_system(self.coordinate_system())
            && other.end_with_coordinate_system(self.coordinate_system()) <= self.end()
    }

    // if precise - no-op
    // if imprecise - set CIs to precise
    //
    fn as_precise(&self) -> Box<dyn Region>;



}

#[cfg(test)]
mod test {
    use crate::CoordinateSystem;
    use super::*;
    use rstest::rstest;
    use crate::confidence_interval::ConfidenceInterval;
    use std::sync::Condvar;


    #[rstest]
    #[case(10, 20, CoordinateSystem::LeftOpen, CoordinateSystem::LeftOpen, 10)]
    #[case(10, 20, CoordinateSystem::LeftOpen, CoordinateSystem::FullyClosed, 11)]
    #[case(10, 20, CoordinateSystem::LeftOpen, CoordinateSystem::RightOpen, 11)]
    #[case(11, 20, CoordinateSystem::FullyClosed, CoordinateSystem::LeftOpen, 10)]
    #[case(11, 20, CoordinateSystem::FullyClosed, CoordinateSystem::FullyClosed, 11)]
    #[case(11, 20, CoordinateSystem::FullyClosed, CoordinateSystem::RightOpen, 11)]
    fn test_start_with_coordinate_system(#[case] start: u32,
                                         #[case] end: u32,
                                         #[case] cs: CoordinateSystem,
                                         #[case] target: CoordinateSystem,
                                         #[case] expected: u32) {
        let start_ci: ConfidenceInterval = ConfidenceInterval::imprecise(-10, 5);
        let end_ci: ConfidenceInterval = ConfidenceInterval::imprecise(-5, 50);
        let region = RegionImprecise::of(start, end, cs, start_ci, end_ci);
        assert_eq!(region.start_with_coordinate_system(&target), expected);
    }

    #[rstest]
    #[case(10, 20, CoordinateSystem::LeftOpen, CoordinateSystem::FullyClosed, 20)]
    #[case(10, 20, CoordinateSystem::FullyOpen, CoordinateSystem::FullyClosed, 19)]
    #[case(10, 20, CoordinateSystem::RightOpen, CoordinateSystem::FullyClosed, 19)]
    #[case(10, 20, CoordinateSystem::RightOpen, CoordinateSystem::FullyOpen, 20)]
    #[case(10, 20, CoordinateSystem::LeftOpen, CoordinateSystem::FullyOpen, 21)]
    fn test_end_with_coordinate_system(#[case] start: u32,
                                         #[case] end: u32,
                                         #[case] cs: CoordinateSystem,
                                         #[case] target: CoordinateSystem,
                                         #[case] expected: u32) {
        let start_ci: ConfidenceInterval = ConfidenceInterval::imprecise(-10, 5);
        let end_ci: ConfidenceInterval = ConfidenceInterval::imprecise(-5, 50);
        let region = RegionImprecise::of(start, end, cs, start_ci, end_ci);
        assert_eq!(region.end_with_coordinate_system(&target), expected);
    }

    struct RegionImprecise {
        start: u32,
        end: u32,
        coordinate_system: CoordinateSystem,
        start_ci: ConfidenceInterval,
        end_ci: ConfidenceInterval
    }

    impl RegionImprecise {
        fn of(start: u32, end: u32, coordinate_system: CoordinateSystem,
              start_ci: ConfidenceInterval, end_ci: ConfidenceInterval) -> Self {
            RegionImprecise {start, end, coordinate_system, start_ci, end_ci}
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
    }

    struct RegionPrecise {
        start: u32,
        end: u32,
        coordinate_system: CoordinateSystem
    }
    impl RegionPrecise {
        fn of(start: u32, end: u32, coordinate_system: CoordinateSystem) -> RegionPrecise {
            RegionPrecise {
                start,
                end,
                coordinate_system
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
    }

}
