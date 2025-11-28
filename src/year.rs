use chrono::FixedOffset;

use crate::{date::SmoitalDate, schedule::SmonthSchedule};

/// Convenience wrapper that lets you work with a specific Martian year.
///
/// It ties a `SmonthSchedule` to a calendar year so you can move between
/// day-of-year indices and `SmoitalDate` values, and ask for the appropriate
/// timezone offset.
pub struct SmoitalYear<S: SmonthSchedule> {
    pub year: i32,
    schedule: S,
}

impl<S: SmonthSchedule> SmoitalYear<S> {
    /// Create a new year bound to the provided schedule.
    pub fn new(year: i32, schedule: S) -> Self {
        Self { year, schedule }
    }

    /// Returns the timezone offset for a day-of-year (0-indexed).
    pub fn timezone_offset_for_day(&self, day_of_year: u32) -> FixedOffset {
        self.schedule.get_timezone_offset(day_of_year)
    }

    /// Convert a `SmoitalDate` to day-of-year and return the timezone offset.
    /// Invalid dates (wrong year or out-of-range day) return `None`.
    pub fn timezone_offset_for_date(&self, date: &SmoitalDate) -> Option<FixedOffset> {
        self.day_of_year(date)
            .map(|day| self.timezone_offset_for_day(day))
    }

    /// Convert a `SmoitalDate` (Smonth is 0-indexed, Day is 1-indexed) to a day-of-year.
    /// Returns `None` if the date does not belong to this year or the day exceeds the
    /// length of the Smonth in the attached schedule.
    pub fn day_of_year(&self, date: &SmoitalDate) -> Option<u32> {
        if date.year != self.year || date.day == 0 {
            return None;
        }

        let mut day_index: u32 = 0;
        for smonth_idx in 0..date.smonth {
            day_index += self.schedule.get_smonth_length(smonth_idx);
        }

        let smonth_len = self.schedule.get_smonth_length(date.smonth);
        if date.day > smonth_len {
            return None;
        }

        Some(day_index + (date.day - 1))
    }

    /// Convert a day-of-year (0-indexed) into a `SmoitalDate` using the attached schedule.
    pub fn date_from_day(&self, day_of_year: u32) -> SmoitalDate {
        let mut remaining = day_of_year;
        let mut smonth_idx = 0;

        loop {
            let smonth_len = self.schedule.get_smonth_length(smonth_idx);
            if remaining < smonth_len {
                return SmoitalDate {
                    year: self.year,
                    smonth: smonth_idx,
                    day: remaining + 1,
                };
            }

            remaining -= smonth_len;
            smonth_idx += 1;
        }
    }

    /// Access the underlying schedule.
    pub fn schedule(&self) -> &S {
        &self.schedule
    }
}
