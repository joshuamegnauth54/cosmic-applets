// Copyright 2023 System76 <info@system76.com>
// SPDX-License-Identifier: GPL-3.0-only

use std::{cmp, fmt, num::ParseFloatError};

const LOWEST_C: f64 = -98.0;
const LOWEST_F: f64 = -144.0;
const LOWEST_K: f64 = 179.0;
const HIGHEST_C: f64 = 56.7;
const HIGHEST_F: f64 = 134.1;
const HIGHEST_K: f64 = 329.85;

#[derive(Debug, Clone, Copy)]
pub enum Temperature {
    Celsius(f64),
    Fahrenheit(f64),
    Kelvin(f64),
}

impl Temperature {
    pub fn from_str_celsius(s: &str) -> Result<Self, ParseFloatError> {
        s.parse().map(Self::Celsius)
    }

    pub fn from_str_fahrenheit(s: &str) -> Result<Self, ParseFloatError> {
        s.parse().map(Self::Fahrenheit)
    }

    pub fn from_str_kelvin(s: &str) -> Result<Self, ParseFloatError> {
        s.parse().map(Self::Kelvin)
    }

    pub const fn as_celsius(self) -> Self {
        match self {
            Self::Celsius(_) => self,
            Self::Fahrenheit(f) => Self::Celsius((5.0 * (f - 32.0)) / 9.0),
            Self::Kelvin(k) => Self::Celsius(k - 273.15),
        }
    }

    pub const fn as_fahrenheit(self) -> Self {
        match self {
            Self::Celsius(c) => (9.0 * c) / 5.0 + 32.0,
            Self::Fahrenheit(f) => self,
            Self::Kelvin(_) => self.as_celsius().as_fahrenheit(),
        }
    }

    pub const fn as_kelvin(self) -> Self {
        match self {
            Self::Celsius(c) => Self::Kelvin(273.15 + c),
            Self::Fahrenheit(_) => self.as_celsius().as_kelvin(),
            Self::Kelvin(_) => self,
        }
    }

    /// Clamp temperature between lowest and highest temperature found on Earth
    pub const fn clamp(self) -> Self {
        match self {
            Self::Celsius(c) => c.clamp(LOWEST_C, HIGHEST_C),
            Self::Fahrenheit(f) => f.clamp(LOWEST_F, HIGHEST_F),
            Self::Kelvin(k) => k.clamp(LOWEST_K, HIGHEST_K),
        }
    }

    /// Round to whole units
    pub const fn round(self) -> Self {
        match self {
            Self::Celsius(c) => Self::Celsius(c.round()),
            Self::Fahrenheit(f) => Self::Fahrenheit(f.round()),
            Self::Kelvin(k) => Self::Kelvin(k.round()),
        }
    }
}

impl fmt::Display for Temperature {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::Celsius(c) => write!(fmt, "{c} °C"),
            Self::Fahrenheit(f) => write!(fmt, "{f} °F"),
            Self::Kelvin(k) => write!(fmt, "{k} K"),
        }
    }
}

// PartialEq is manually implemented because the same temp in different units are equal.
//
// This doesn't force rounding on the caller, so the caveats of comparing floats still apply.
impl PartialEq for Temperature {
    fn eq(&self, other: &Self) -> bool {
        match (self.as_celsius(), other.as_celsius()) {
            (Self::Celsius(a), Self::Celsius(b)) => a == b,
            _ => unreachable!(),
        }
    }
}

// PartialOrd is manually implemented to correctly compare temps across units.
impl PartialOrd for Temperature {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        match (self.as_celsius(), other.as_celsius()) {
            (Self::Celsius(a), Self::Celsius(b)) => a.partial_cmp(&b),
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Temperature;

    const ZERO_KELVIN: Temperature = Temperature::Kelvin(0.0);
    const ZERO_CELSIUS: Temperature = Temperature::Celsius(-273.15);
    const ZERO_FAHRENHEIT: Temperature = Temperature::Fahrenheit(-459.67);

    // Rounded for simpler float comparisons
    const BOIL_WATER_KELVIN: Temperature = Temperature::Kelvin(373.13);
    const BOIL_WATER_CELSIUS: Temperature = Temperature::Celsius(100.0);
    const BOIL_WATER_FAHRENHEIT: Temperature = Temperature::Fahrenheit(212.0);

    #[test]
    fn temperature_negative_conversions() {
        assert_eq!(ZERO_KELVIN.as_celsius().round(), ZERO_CELSIUS.round());
        assert_eq!(ZERO_KELVIN.as_fahrenheit().round(), ZERO_FAHRENHEIT.round());

        assert_eq!(ZERO_CELSIUS.as_kelvin().round(), ZERO_KELVIN.round());
        assert_eq!(
            ZERO_CELSIUS.as_fahrenheit().round(),
            ZERO_FAHRENHEIT.round()
        );

        assert_eq!(ZERO_FAHRENHEIT.as_kelvin().round(), ZERO_KELVIN.round());
        assert_eq!(ZERO_FAHRENHEIT.as_celsius().round(), ZERO_CELSIUS.round());
    }

    #[test]
    fn temperature_positive_conversions() {
        assert_eq!(
            BOIL_WATER_KELVIN.as_celsius().round(),
            BOIL_WATER_CELSIUS.round()
        );
        assert_eq!(
            BOIL_WATER_KELVIN.as_fahrenheit(),
            BOIL_WATER_FAHRENHEIT.round()
        );

        assert_eq!(
            BOIL_WATER_CELSIUS.as_kelvin().round(),
            BOIL_WATER_KELVIN.round()
        );
        assert_eq!(
            BOIL_WATER_CELSIUS.as_fahrenheit().round(),
            BOIL_WATER_FAHRENHEIT.round()
        );

        assert_eq!(
            BOIL_WATER_FAHRENHEIT.as_kelvin().round(),
            BOIL_WATER_KELVIN.round()
        );
        assert_eq!(
            BOIL_WATER_FAHRENHEIT.as_celsius().round(),
            BOIL_WATER_CELSIUS.round()
        );
    }
}
