pub struct ConfidenceInterval {
    lower_bound: i64,
    upper_bound: u64
}

impl ConfidenceInterval {
    pub fn new(lower_bound: i64, upper_bound: u64) -> Self {
        if lower_bound == 0 && upper_bound == 0 {
            &ConfidenceInterval {
                lower_bound: 0,
                upper_bound: 0
            }
        } else {
            ConfidenceInterval { lower_bound, upper_bound }
        }

    }

    fn precise() -> &ConfidenceInterval {
        &ConfidenceInterval {
            lower_bound: 0,
            upper_bound: 0
        }
    }

    fn lower_bound(&self) -> i64 {
        self.lower_bound
    }
    fn upper_bound(&self) -> u64 {
        self.upper_bound
    }
}