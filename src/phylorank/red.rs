use derive_more::Sub;

/// A Relative Evolutionary Divergence ([RED]) score.
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Sub)]
pub struct RED(pub f64);


impl RED {
    /// Return the absolute value of the [RED] score.
    pub fn abs(&self) -> Self {
        RED(self.0.abs())
    }
}

