use chrono::FixedOffset;

/// Represents a Date in the Smoital System.
///
/// Defined by Year, Smonth (intercalary month), and Day-of-Smonth.
/// This structure simplifies the handling of the 36/37 day months.
#[derive(Debug, Clone, PartialEq)]
pub struct SmoitalDate {
    pub year: i32,
    pub smonth: u32,
    pub day: u32, // 1-37
}

impl SmoitalDate {
    /// Calculate the UTC offset for this date using the standard formula.
    /// UTC-Offset = 760 - 40*D.
    ///
    /// Note: This formula applies strictly to the position within the Smonth.
    pub fn calculate_offset(&self) -> FixedOffset {
        // D is day of smonth
        let d = self.day as i32;

        // D=37 (Smol Day) is fixed at 24:00 (which implies start at -12:00)
        // 760 - 40*37 = -720m = -12h. Formula holds.
        let offset_min = 760 - (40 * d);

        FixedOffset::east_opt(offset_min * 60).unwrap()
    }

    /// Helper to identify if this is a "Smol Day" (Shortened Day).
    /// In Smoital, Smol days are *always* the 37th day.
    pub fn is_smol_day(&self) -> bool {
        self.day == 37
    }
}
