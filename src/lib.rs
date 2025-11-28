//! # Smoital Timekeeping Library
//!
//! Implements the **Synodic Mars-Orbit Intercalary Timezone Alignment (Smoital)** system.
//! Based on the paper "The Smoital System" (Joffe, 2025).
//!
//! Features:
//! - Heuristic Smonth Schedule generation.
//! - Optimized Clock display logic (XM/Overflow).
//! - Precise Mars/Earth duration conversion.
//! - IANA Timezone Rule generation.

pub mod clock;
pub mod constants;
pub mod date;
pub mod duration;
pub mod iana;
pub mod schedule;
pub mod year;

// Re-exports for easier access
pub use clock::{DisplayMode, SmoitalClock};
pub use date::SmoitalDate;
pub use duration::MarsDuration;
pub use schedule::{HeuristicSchedule, SmonthSchedule};
pub use year::SmoitalYear;
