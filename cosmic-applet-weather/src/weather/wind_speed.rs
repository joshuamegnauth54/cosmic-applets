// Copyright 2023 System76 <info@system76.com>
// SPDX-License-Identifier: GPL-3.0-only

use std::{cmp, fmt};

#[derive(Debug, Clone, Copy)]
pub enum WindSpeed {
    /// Kilometers per hour
    Kmh(f64),
    /// Knots
    Knots(f64),
    // Miles per hour
    Mph(f64),
    /// Meters per second
    Ms(f64),
}

impl WindSpeed {
    pub const fn as_kmh(self) -> Self {}

    pub const fn as_knots(self) -> Self {}

    pub const fn as_mph(self) -> Self {}

    pub const fn as_ms(self) -> Self {}
}

impl fmt::Display for WindSpeed {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::Kmh(kmh) => write!(f, "{kmh} km/h"),
            Self::Knots(kn) => write!(f, "{kn} kn"),
            Self::Mph(mph) => write!(f, "{mph} mph"),
            Self::Ms(ms) => write!(f, "{ms} m/s"),
        }
    }
}

// PartialEq is manually implemented because the same speed in different units are equal.
//
// This doesn't force rounding on the caller, so the caveats of comparing floats still apply.
impl PartialEq for WindSpeed {
    fn eq(&self, other: &Self) -> bool {
        match (self.as_kmh(), other.as_kmh()) {
            (Self::Kmh(a), Self::Kmh(b)) => a == b,
            _ => unreachable!(),
        }
    }
}

// PartialOrd is manually implemented to correctly compare speeds across units.
impl PartialOrd for WindSpeed {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        match (self.as_kmh(), other.as_kmh()) {
            (Self::Kmh(a), Self::Kmh(b)) => a.partial_cmp(&b),
            _ => unreachable!(),
        }
    }
}
