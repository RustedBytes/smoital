use chrono::{DateTime, Timelike, Utc};

/// Display modes for the "Extended" 40 minutes of the Martian day.
pub enum DisplayMode {
    /// Standard ISO8601 (e.g., 23:20) - ambiguous logic.
    Unoptimized,
    /// Overflow logic (e.g., 24:00).
    Overflowed,
    /// Extended minutes (e.g., 23:60).
    ExtendedMinutes,
    /// XM notation (e.g., 12:00 XM).
    XM,
}

pub struct SmoitalClock;

impl SmoitalClock {
    /// Determines the optimized time string for a given UTC instant.
    ///
    /// This handles the "Gap" logic where 23:20 UTC to 23:59 UTC represents
    /// the extra 40 minutes added to a standard day.
    pub fn format(time: DateTime<Utc>, mode: DisplayMode) -> String {
        let h = time.hour();
        let m = time.minute();
        let s = time.second();

        // Check for the extended period signature (last 40 mins of Earth day)
        // In a real integration, this requires timezone context, but for the
        // library's display logic, we detect the standard "slide back" window.
        let is_extended = h == 23 && m >= 20;

        if !is_extended {
            return format!("{:02}:{:02}:{:02}", h, m, s);
        }

        match mode {
            DisplayMode::Unoptimized => format!("{:02}:{:02}:{:02}", h, m, s),
            DisplayMode::Overflowed => {
                // Map 23:20 -> 24:00
                format!("24:{:02}:{:02}", m - 20, s)
            }
            DisplayMode::ExtendedMinutes => {
                // Map 23:20 -> 23:60
                format!("23:{:02}:{:02}", m + 40, s)
            }
            DisplayMode::XM => {
                // Map 23:20 -> 12:00 XM
                format!("12:{:02}:{:02} XM", m - 20, s)
            }
        }
    }
}
