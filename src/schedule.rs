use chrono::FixedOffset;

use crate::constants::*;

/// Defines the layout of a Martian Year (which Smonths are 37 days long).
pub trait SmonthSchedule {
    /// Returns true if the Smonth at the given index (0-based) is 37 days long.
    fn is_smol_smonth(&self, smonth_index: u32) -> bool;

    /// Returns the length of the Smonth in days (36 or 37).
    fn get_smonth_length(&self, smonth_index: u32) -> u32 {
        if self.is_smol_smonth(smonth_index) {
            37
        } else {
            36
        }
    }

    /// Calculates the UTC offset for a specific day of the year (0-667).
    /// This requires iterating through the Smonths to find which one the day falls into.
    fn get_timezone_offset(&self, day_of_year: u32) -> FixedOffset;
}

/// Implements the "Equatorial Smoital Schedule".
///
/// This schedule is optimized for equatorial regions and is designed to be used
/// consistently every year. It features a "Quiet Period" (Period 1) of
/// standard 36-day Smonths during the long-day season (Perihelion), followed
/// by a cluster of 37-day (Smol) Smonths.
///
/// Pattern:
/// - Period 1 (Perihelion): ~7 Smonths of 36 days.
/// - Period 2 & 3: Alternating 37-day Smonths to correct the equation of time.
pub struct EquatorialSchedule {
    /// The index of the first "Long" (37-day) Smonth.
    /// Based on the paper's heuristics, this is typically index 5 or 6.
    first_long_smonth_index: u32,
    /// The relative offsets of the 37-day months from the first long smonth.
    /// Corresponds to the pattern [1, 2, 4, 5, 7, 9, 11].
    long_smonth_offsets: [u32; 7],
}

impl Default for EquatorialSchedule {
    fn default() -> Self {
        Self {
            // A default start index of 6 aligns well with the "Period 1" duration (~220-250 days).
            first_long_smonth_index: 6,
            // The spacing pattern defined for the Equatorial schedule
            long_smonth_offsets: [0, 1, 3, 4, 6, 8, 10],
            // Note: The paper sometimes defines offsets as [1, 2, 4...] relative to a base.
            // Here we map them 0-based relative to `first_long_smonth_index`.
            // Pattern logic:
            // Index 6 (37), Index 7 (37), Index 8 (36), Index 9 (37), Index 10 (37)...
        }
    }
}

impl EquatorialSchedule {
    pub fn new() -> Self {
        Self::default()
    }
}

impl SmonthSchedule for EquatorialSchedule {
    fn is_smol_smonth(&self, smonth_index: u32) -> bool {
        // Before the first long smonth, all are 36 (Standard).
        if smonth_index < self.first_long_smonth_index {
            return false;
        }

        let relative_index = smonth_index - self.first_long_smonth_index;

        // Check if the relative index matches one of the "Long" offsets.
        self.long_smonth_offsets.contains(&relative_index)
    }

    fn get_timezone_offset(&self, day_of_year: u32) -> FixedOffset {
        let mut current_day_sum = 0;
        let mut smonth_idx = 0;

        // 1. Determine which Smonth we are in
        while current_day_sum + self.get_smonth_length(smonth_idx) <= day_of_year {
            current_day_sum += self.get_smonth_length(smonth_idx);
            smonth_idx += 1;
        }

        // 2. Determine Day-of-Smonth (1-based)
        let day_of_smonth = day_of_year - current_day_sum + 1;

        // 3. Apply Smoital Logic
        // If it is the 37th day of a 37-day month, it is a Smol Day (UTC-12:00)
        let is_long_month = self.get_smonth_length(smonth_idx) == 37;

        if is_long_month && day_of_smonth == 37 {
            return FixedOffset::west_opt(12 * 3600).unwrap();
        }

        // Otherwise, use the standard formula: Offset = 760 - 40 * D
        // This calculates the offset in minutes.
        let offset_minutes = 760 - (40 * day_of_smonth as i32);

        FixedOffset::east_opt(offset_minutes * 60).unwrap()
    }
}

/// Implements the Heuristic Algorithm.
///
/// This calculates the precise timezone schedule for any year based on a
/// reference "Natural Timezone" (Mean Solar Time offset).
pub struct HeuristicSchedule {
    natural_tz_start: f64,
    smol_dates: Vec<u32>, // Day indices (0-indexed) that are Smol
}

impl HeuristicSchedule {
    pub fn new(_year: i32, natural_tz_min: f64) -> Self {
        // SmoitalTZ_{y,0} calculation
        let raw_start = natural_tz_min + (C1_SECONDS / 60.0);
        let _start_offset = Self::wrap_24hr(Self::round_40min(raw_start));

        // Smoitus Factor
        let smoitus_factor = ((raw_start / 40.0) + 0.5).fract();

        // SmonthStart approximation
        let smonth_start_y0 = 0.0;

        // FirstLongSmonth
        let c2 = C2;
        let c3 = C3;
        let c4 = C4;
        let fls_val = c2 + (smoitus_factor * c3) + (smonth_start_y0 / c4);
        let first_long_smonth = fls_val.floor() as i32;

        let smol_days_in_year = 6;

        // Smol Date Generation using the heuristic spacing pattern
        // Pattern of spacing relative to first_long_smonth: [1, 2, 4, 5, 7, 9, 11]
        // Note: The previous struct used 0-based local offsets, this uses the paper's
        // 1-based indexing logic for the calculation loop.
        let spacings = [1, 2, 4, 5, 7, 9, 11];
        let mut smol_dates = Vec::new();

        for n in 0..smol_days_in_year {
            let spacing_idx = n as usize;
            if spacing_idx < spacings.len() {
                let s_month_offset = spacings[spacing_idx];
                // Formula approx: 36 * (FirstLongSmonth + Spacing) + n
                // We clamp to ensure valid day indices.
                let date_idx = 36 * (first_long_smonth + s_month_offset) + (n as i32);
                if date_idx >= 0 {
                    smol_dates.push(date_idx as u32);
                }
            }
        }

        HeuristicSchedule {
            natural_tz_start: natural_tz_min,
            smol_dates,
        }
    }

    fn round_40min(tz: f64) -> f64 {
        (tz / 40.0).round() * 40.0
    }

    fn wrap_24hr(tz: f64) -> f64 {
        let mut t = tz;
        while t <= -720.0 {
            t += 1440.0;
        }
        while t > 720.0 {
            t -= 1440.0;
        }
        t
    }
}

impl SmonthSchedule for HeuristicSchedule {
    fn is_smol_smonth(&self, _smonth_index: u32) -> bool {
        // The heuristic struct calculates Smol Dates directly rather than Smonth lengths,
        // so this trait method is less applicable, but we can infer it.
        // For simplicity in this library context, we rely on get_timezone_offset.
        false
    }

    fn get_timezone_offset(&self, day_of_year: u32) -> FixedOffset {
        // Check if Smol Day (UTC-12:00)
        if self.smol_dates.contains(&day_of_year) {
            return FixedOffset::west_opt(12 * 3600).unwrap();
        }

        // Calculate count of Smol days up to this day
        let smol_count = self.smol_dates.iter().filter(|&&d| d < day_of_year).count() as i32;

        // Formula: SmoitalTZ_{y,0} - 40 * (d - smol_count)
        let start_offset = Self::wrap_24hr(Self::round_40min(
            self.natural_tz_start + (C1_SECONDS / 60.0),
        ));
        let adjustment = 40.0 * (day_of_year as i32 - smol_count) as f64;

        let offset = Self::wrap_24hr(start_offset - adjustment);

        FixedOffset::east_opt((offset * 60.0) as i32).unwrap()
    }
}
