use crate::Contig;
use crate::default::RegionImprecise;
use super::confidence_interval::ConfidenceInterval;
use super::coordinate_system::CoordinateSystem;
use super::default::RegionPrecise;

pub fn region(coordinate_system: CoordinateSystem,
              start: u32,
              start_ci: ConfidenceInterval,
              end: u32,
              end_ci: ConfidenceInterval) -> Box<dyn Region> {
    if start_ci.is_precise() && end_ci.is_precise() {
        precise_region(coordinate_system, start, end)
    } else {
        Box::new(RegionImprecise::of(coordinate_system, start, start_ci, end, end_ci))
    }
}

pub fn precise_region(coordinate_system: CoordinateSystem,
                      start: u32,
                      end: u32) -> Box<dyn Region> {
    Box::new(RegionPrecise::of(coordinate_system, start, end))
}

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

    fn end_with_coordinate_system(&self, _cs: &CoordinateSystem) -> u32 {
        self.end()
    }

    fn with_coordinate_system(&mut self, cs: &CoordinateSystem);

    fn contains(&self, other: &dyn Region) -> bool {
        self.start() <= other.start_with_coordinate_system(self.coordinate_system())
            && other.end_with_coordinate_system(self.coordinate_system()) <= self.end()
    }

    fn invert(&mut self, contig: &dyn Contig);
}
