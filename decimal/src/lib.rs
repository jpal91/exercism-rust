use std::ops::{Add, Mul, Sub};

/// Type implementing arbitrary-precision decimal arithmetic

#[derive(Debug)]
pub struct Decimal(f32);

impl Decimal {
    pub fn try_from(input: &str) -> Option<Decimal> {
        match input.parse::<f32>() {
            Ok(n) => Some(Decimal(n)),
            _ => None
        }
    }
}

impl PartialEq for Decimal {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Eq for Decimal {}

impl PartialOrd for Decimal {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Decimal {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.total_cmp(&other.0)
    }
}

impl Add for Decimal {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Decimal(self.0 + rhs.0)
    }
}

impl Sub for Decimal {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Decimal(self.0 - rhs.0)
    }
}

impl Mul for Decimal {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Decimal(self.0 * rhs.0)
    }
}