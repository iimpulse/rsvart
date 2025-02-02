use crate::{GenomicRegion, Regioned};
use crate::stranded::Stranded;

pub trait StrandedRegioned {
    
    fn start_on_strand_with_coordinate_system() -> u32;
}

impl <T: Regioned + Stranded> StrandedRegioned for T {

    fn start_on_strand_with_coordinate_system() -> u32 { todo!() }
}


#[cfg(test)]
mod test {
    use super::*;
    use rstest::rstest;
    use crate::{CoordinateSystem, Strand};
    use crate::Region;

    struct TestRegion {
        strand: Strand,
        region: Region
    }

    impl Stranded for TestRegion {
        fn strand(&self) -> Strand {
            return self.strand;
        }
    }

    impl Regioned for TestRegion {
        fn region(&self) -> &Region {
            &self.region
        }
    }

    // const REGION_1: TestRegion = TestRegion { strand: Strand::Positive,
    //     region: Region::new(0,20, CoordinateSystem::zero_based()) };
    // const REGION_2: TestRegion = TestRegion { strand: Strand::Negative,
    //     region: Region::new(19, 30, CoordinateSystem::zero_based()) };
    // const REGION_3: TestRegion = TestRegion { strand: Strand::Negative,
    //     region: Region::new(21, 36, CoordinateSystem::zero_based()) };
    
}


