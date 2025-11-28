/// The exact ratio of Mars Seconds to Earth Seconds.
/// Derived from 24h 39m 35s 244ms / 86400s.
/// Ratio = 1.02749125.
pub const MARS_TO_EARTH_RATIO: f64 = 1.02749125;

/// Length of a Martian Sol in Earth seconds (88,775.244 seconds).
pub const SOL_LENGTH_SECONDS: f64 = 88_775.244;

/// Standard Day length in minutes (24h 40m).
pub const STANDARD_DAY_MINS: i64 = 24 * 60 + 40;

/// Smol Day length in minutes (24h 00m).
pub const SMOL_DAY_MINS: i64 = 24 * 60;

// Constants for Heuristic Algorithm
pub const C1_SECONDS: f64 = 85.0;
pub const C2: f64 = 4.51;
pub const C3: f64 = 1.54;
pub const C4: f64 = 35.8;
