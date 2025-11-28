use crate::constants::MARS_TO_EARTH_RATIO;
use std::time::Duration;

/// Represents a duration measured in Martian time units.
///
/// Useful for precise conversion between Earth seconds and Mars seconds,
/// which is necessary because 1 Mars second = 1.02749125 Earth seconds.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct MarsDuration {
    mars_seconds: f64,
}

impl MarsDuration {
    /// Create from Martian seconds.
    pub fn from_mars_seconds(secs: f64) -> Self {
        Self { mars_seconds: secs }
    }

    /// Create from Earth seconds (converting via the exact ratio).
    pub fn from_earth_seconds(earth_secs: f64) -> Self {
        Self {
            mars_seconds: earth_secs / MARS_TO_EARTH_RATIO,
        }
    }

    /// Get the value in Martian seconds.
    pub fn as_mars_seconds(&self) -> f64 {
        self.mars_seconds
    }

    /// Get the value in Earth seconds.
    pub fn as_earth_seconds(&self) -> f64 {
        self.mars_seconds * MARS_TO_EARTH_RATIO
    }

    /// Convert to a standard Rust Duration (Earth time).
    pub fn to_earth_duration(&self) -> Duration {
        let earth_secs = self.as_earth_seconds();
        Duration::from_secs_f64(earth_secs)
    }
}
