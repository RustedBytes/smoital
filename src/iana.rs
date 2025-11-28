use crate::schedule::HeuristicSchedule;
use crate::schedule::SmonthSchedule;

/// Generates the IANA Timezone Rules for a given year.
///
/// Corresponds to the format described in Section 18.
pub fn generate_year_rules(year: i32, schedule: &HeuristicSchedule) -> Vec<String> {
    let mut rules = Vec::new();
    let days_in_year = 668; // Approximation for demo

    for d in 0..days_in_year {
        let offset = schedule.get_timezone_offset(d);
        let off_min = offset.local_minus_utc() / 60;

        // IANA Format: Rule YEAR Smoital only MON DAY 24:00 OFFSET
        // Note: Mapping Day Index 'd' to Earth Gregorian Month/Day requires
        // the "Skipped Date" epoch logic (Earth-Date = S + D).

        // Simplified output format for verification:
        rules.push(format!(
            "Rule {} Smoital only Day{} 24:00 {}",
            year, d, off_min
        ));
    }
    rules
}
