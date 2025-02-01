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

    pub fn to_precise(&mut self) {
        self.upper_bound = 0;
        self.lower_bound = 0;
    }

    /// Swap upper and lower CI bounds.
    fn swap(&mut self) {
        std::mem::swap(&mut self.upper_bound, &mut self.lower_bound)
    }

    /// Swap upper and lower CI bounds in provided CIs and swap the CIs themselves.
    pub fn swap_and_invert(left: &mut ConfidenceInterval, right: &mut ConfidenceInterval) {
        left.swap();
        right.swap();
        std::mem::swap(left, right);
    }
}
