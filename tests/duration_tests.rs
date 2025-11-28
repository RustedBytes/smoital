use smoital::duration::MarsDuration;

fn close_to(a: f64, b: f64) -> bool {
    let tolerance = 1e-9_f64.max(1e-9 * a.abs());
    (a - b).abs() < tolerance
}

#[test]
fn converts_between_mars_and_earth_seconds() {
    let earth_secs = 5_000.0;
    let duration = MarsDuration::from_earth_seconds(earth_secs);

    assert!(close_to(duration.as_earth_seconds(), earth_secs));

    let round_tripped = MarsDuration::from_mars_seconds(duration.as_mars_seconds());
    assert!(close_to(round_tripped.as_earth_seconds(), earth_secs));
}

#[test]
fn mars_seconds_are_preserved() {
    let mars_secs = 123.456;
    let duration = MarsDuration::from_mars_seconds(mars_secs);

    assert!(close_to(duration.as_mars_seconds(), mars_secs));
    assert!(close_to(
        duration.as_earth_seconds(),
        mars_secs * 1.02749125
    ));
}

#[test]
fn std_duration_matches_earth_seconds() {
    let duration = MarsDuration::from_mars_seconds(2.5);
    let earth_seconds = duration.as_earth_seconds();
    let std_dur = duration.to_earth_duration();

    assert!(close_to(std_dur.as_secs_f64(), earth_seconds));
}
