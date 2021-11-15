const PRECISE: ConfidenceInterval = ConfidenceInterval { lower_bound: 0, upper_bound: 0 };

pub struct ConfidenceInterval {
    // TODO - investigate if it is OK to use unsigned ints for upper/lower bounds
    upper_bound: u32,
    lower_bound: u32,
}

impl ConfidenceInterval {
    pub fn imprecise(upper_bound: u32, lower_bound: u32) -> Self {
        ConfidenceInterval { upper_bound, lower_bound }
    }

    pub fn precise() -> &'static ConfidenceInterval {
        &PRECISE
    }

    pub fn lower_bound(&self) -> u32 {
        self.lower_bound
    }
    pub fn upper_bound(&self) -> u32 {
        self.upper_bound
    }

    pub fn is_precise(&self) -> bool {
        self.upper_bound == 0 && self.lower_bound == 0
    }
}
