use smoital::date::SmoitalDate;
use smoital::schedule::EquatorialSchedule;
use smoital::year::SmoitalYear;

#[test]
fn converts_between_day_indices_and_dates() {
    let schedule = EquatorialSchedule::new();
    let year = SmoitalYear::new(2090, schedule);

    let first_day = year.date_from_day(0);
    assert_eq!(
        first_day,
        SmoitalDate {
            year: 2090,
            smonth: 0,
            day: 1
        }
    );

    let smol_date = SmoitalDate {
        year: 2090,
        smonth: 6,
        day: 37,
    };
    assert_eq!(year.day_of_year(&smol_date), Some(252));
    assert_eq!(year.date_from_day(252), smol_date);

    // Invalid day (beyond the Smonth length) should return None.
    let invalid_day = SmoitalDate {
        year: 2090,
        smonth: 0,
        day: 40,
    };
    assert_eq!(year.day_of_year(&invalid_day), None);
}

#[test]
fn timezone_offsets_can_be_queried_with_dates() {
    let year = SmoitalYear::new(2030, EquatorialSchedule::new());

    let start_of_year = SmoitalDate {
        year: 2030,
        smonth: 0,
        day: 1,
    };
    let smol_date = SmoitalDate {
        year: 2030,
        smonth: 6,
        day: 37,
    };

    assert_eq!(
        year.timezone_offset_for_date(&start_of_year)
            .unwrap()
            .local_minus_utc(),
        12 * 3600
    );
    assert_eq!(
        year.timezone_offset_for_date(&smol_date)
            .unwrap()
            .local_minus_utc(),
        -12 * 3600
    );

    // Date belonging to another year should be rejected.
    let wrong_year = SmoitalDate {
        year: 2029,
        smonth: 0,
        day: 1,
    };
    assert!(year.timezone_offset_for_date(&wrong_year).is_none());
}
