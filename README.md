# smoital

[![Test](https://github.com/RustedBytes/smoital/actions/workflows/test.yml/badge.svg)](https://github.com/RustedBytes/smoital/actions/workflows/test.yml)
[![Crates.io Version](https://img.shields.io/crates/v/smoital)](https://crates.io/crates/smoital)

A Rust library for practical Martian timekeeping compatible with Earth-based ISO8601 standards.

Based on the 2025 paper [*"The Smoital System"* by Ben Joffe](https://www.marspapers.org/paper/Joffe_2025contrib.pdf), this library implements a system that allows Martian colonies to track Mean Solar Time using standard Earth computing primitives (UTC offsets and IANA timezones).

## The Concept

Designing a timekeeping system for Mars balances familiarity with Earth conventions against the reality of a 24h 39m 35s Martian day (Sol).

The **Smoital System** solves this by defining nearly all Martian calendar days as exactly **24 hours and 40 minutes** long (Standard Days), compensated by inserting **6 or 7 "Smol Days"** (24 hours exactly) per Martian year.

Instead of redefining the second or minute (which breaks software compatibility), Smoital relies on strategic **UTC Timezone Offsets**:

  * **Standard Days:** The timezone shifts back by 40 minutes daily to accommodate the longer day.
  * **Smol Days:** Occur specifically on the $37^{th}$ day of a 37-day "Smonth" and are fixed at UTC-12:00.
  * **Interoperability:** Allows Mars time to be stored in standard SQL databases and displayed on legacy OS clocks without modification.

## Features

  * **Heuristic Smonth Scheduling:** Implements the algorithm to determine the 36/37-day month pattern for any Martian year.
  * **Equatorial Schedule:** Pre-configured schedule optimized for equatorial settlements (consistent 36/37 day pattern).
  * **Optimized Clock Display:** Logic to disambiguate the "Extra 40 Minutes" (XM) that occur when a timezone shifts back.
  * **Precise Durations:** `MarsDuration` types using the exact Mars/Earth ratio ($1.02749125$).
  * **IANA Rule Generation:** Utilities to generate standard IANA timezone file formats for OS integration.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
smoital = "0.1"
chrono = "0.4"
```

## Usage

### 1\. Determining Martian Timezone Offsets

Calculate the correct UTC offset for any day of the year using the **Equatorial Schedule**.

```rust
use smoital::{SmonthSchedule, EquatorialSchedule};

fn main() {
    let schedule = EquatorialSchedule::new();

    // Day 0 (Start of Year): Standard 36-day month
    let offset_start = schedule.get_timezone_offset(0);
    println!("Day 0 Offset: {}", offset_start); // +12:00

    // Day 252 (A "Smol" Day): 37th day of a long Smonth
    let offset_smol = schedule.get_timezone_offset(252);
    println!("Day 252 Offset: {}", offset_smol); // -12:00
}
```

### 2\. The "Optimized" Clock (Handling the 24:40 Day)

On Mars, a standard day is 24h 40m. Standard Earth clocks will repeat the time `23:20` to `23:59` when the timezone shifts back. The `SmoitalClock` provides unambiguous display formats for this "Gap".

```rust
use chrono::{Utc, TimeZone};
use smoital::{SmoitalClock, DisplayMode};

fn main() {
    // A time inside the "Extra 40 Minutes" (e.g., 23:30 UTC)
    // On a standard clock, this looks like 23:30. 
    // In Smoital, this is actually part of the extended day.
    let time = Utc.with_ymd_and_hms(2025, 1, 1, 23, 30, 0).unwrap();

    // 1. "Overflow" Mode (24:00 - 24:39)
    let overflow = SmoitalClock::format(time, DisplayMode::Overflowed);
    println!("Overflow: {}", overflow); 
    // Output: "24:10:00"

    // 2. "XM" (Extra Meridian) Mode (12:00 XM - 12:39 XM)
    let xm = SmoitalClock::format(time, DisplayMode::XM);
    println!("XM Mode:  {}", xm); 
    // Output: "12:10:00 XM"
}
```

### 3\. Precise Mars Duration

Handle scientific calculations using the exact Earth-to-Mars second ratio.

```rust
use smoital::MarsDuration;

fn main() {
    // Create a duration of 100 Martian seconds
    let mars_duration = MarsDuration::from_mars_seconds(100.0);
    
    // Convert to Earth seconds
    // Ratio: 1 Mars sec = 1.02749125 Earth sec
    println!("Earth Seconds: {:.8}", mars_duration.as_earth_seconds());
    // Output: 102.74912500
}
```

### 4\. Working with a Calendar Year

Tie a schedule to a specific year so you can navigate between `day_of_year` values and `SmoitalDate` instances.

```rust
use smoital::{EquatorialSchedule, SmoitalDate, SmoitalYear};

fn main() {
    let year = SmoitalYear::new(2030, EquatorialSchedule::new());
    let smol_day = SmoitalDate {
        year: 2030,
        smonth: 6, // 0-indexed Smonth
        day: 37,
    };

    let day_index = year.day_of_year(&smol_day).unwrap(); // 252
    let offset = year.timezone_offset_for_date(&smol_day).unwrap();

    println!("Day {day_index} offset: {offset}");
}
```

## Background

### The "Smol" Day

To keep the calendar synchronized with the actual solar day, Smoital introduces **Smol Days**. These are days where the "extra" 40 minutes are omitted.

  * **Frequency:** \~6 or 7 times per Martian Year.
  * **Identification:** Always the $37^{th}$ day of a Smonth.
  * **Result:** A built-in correction for Mars' highly variable Equation of Time.

### Smonths

A "Smonth" (Synodic Month) is a period of 36 or 37 days. It represents the time it takes for the timezone offset to cycle from UTC+12:00 to UTC-12:00.

  * **Standard Smonth:** 36 Days (Days 1-36).
  * **Long Smonth:** 37 Days (Days 1-37). The 37th day is a **Smol Day**.

## License

This project is licensed under the Apache-2.0 License.
