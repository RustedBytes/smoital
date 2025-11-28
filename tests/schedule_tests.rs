use smoital::schedule::{EquatorialSchedule, HeuristicSchedule, SmonthSchedule};

fn assert_offset_seconds(schedule: &impl SmonthSchedule, day: u32, expected_secs: i32) {
    assert_eq!(
        schedule.get_timezone_offset(day).local_minus_utc(),
        expected_secs,
        "unexpected offset for day {day}"
    );
}

#[test]
fn equatorial_identifies_long_months() {
    let sched = EquatorialSchedule::new();
    let long_indices = [6, 7, 9, 10, 12, 14, 16];

    for idx in 0..=16 {
        let is_long = long_indices.contains(&idx);
        assert_eq!(
            sched.is_smol_smonth(idx),
            is_long,
            "Smonth {idx} should {}be 37 days",
            if is_long { "" } else { "not " }
        );
        assert_eq!(sched.get_smonth_length(idx), if is_long { 37 } else { 36 });
    }
}

#[test]
fn equatorial_offsets_reset_after_smol_day() {
    let sched = EquatorialSchedule::new();

    // Start of year.
    assert_offset_seconds(&sched, 0, 12 * 3600);

    // Start of the first long smonth (index 6).
    assert_offset_seconds(&sched, 216, 12 * 3600);

    // Day before the smol day within the long smonth.
    assert_offset_seconds(&sched, 251, -680 * 60);

    // Smol day itself is pinned to UTC-12:00.
    assert_offset_seconds(&sched, 252, -12 * 3600);

    // Day after the smol day should restart at +12:00.
    assert_offset_seconds(&sched, 253, 12 * 3600);
}

#[test]
fn heuristic_schedule_marks_smol_days_and_wraps_offsets() {
    // Natural timezone start chosen so the rounded offset begins at 0.
    let sched = HeuristicSchedule::new(2030, 0.0);
    let expected_smol_days = [216, 253, 326, 363, 436, 509];

    for day in expected_smol_days {
        assert_offset_seconds(&sched, day, -12 * 3600);
    }

    // One day before the first smol day should wrap back to +00:40.
    assert_offset_seconds(&sched, 215, 40 * 60);

    // Immediately after smol days the offset should re-align to +00:00.
    assert_offset_seconds(&sched, 217, 0);
    assert_offset_seconds(&sched, 254, 0);
}
