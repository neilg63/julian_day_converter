[![mirror](https://img.shields.io/badge/mirror-github-blue)](https://github.com/neilg63/julian_day_converter)
[![crates.io](https://img.shields.io/crates/v/julian_day_converter.svg)](https://crates.io/crates/julian_day_converter)
[![docs.rs](https://docs.rs/julian_day_converter/badge.svg)](https://docs.rs/julian_day_converter)

# Julian Day Compatibility methods for Chrono

This library provides compatibility with astronomical applications that use Julian Days as 64-bit floats. A *Julian day* represents the number of days since the beginning of the Julian period, which started at 12 noon on 24th November 4713 BCE (-4713-11-24T12:00:00 UTC). Julian days facilitate calculations when dealing with extended periods of time and should not be confused with the Julian Calendar, which affects leap year assignment rules.

This crate adds three traits and 6 utility methods to Rust's standard datetime crate, Chrono, as well as standalone functions to convert to and from Unix timestamps. All date-time values are UTC, but may be converted to a timezone-aware *chrono::DateTime*.

A similar [julianday](https://crates.io/crates/julianday) crate exists to handle Julian days as integers and converts them to *chrono::NaiveDate* only. I developed this crate primarily to ensure interoperability with an [Astrological API server](https://github.com/neilg63/astro-calc-api) that leverages the [Swiss Ephemeris](https://github.com/aloistr/swisseph) calculation engine.

## Direct functions

### unixtime_to_julian_day(ts: i64) -> f64
Converts a unix timestamp directly to Julian days as a 64-bit float, compatibile with many astronomical applications.

### julian_day_to_unixtime(jd: f64) -> i64
Converts a Julian Day as a signed 64-bit integer. If the timestamp has to be cast to a 32-bit integers, dates before 1902 and after 2038 will be out of range.

### julian_day_to_weekday_index(jd: f64, offset_secs: i32) -> u8
Calculates the weekday index, where Sunday = 0, Monday = 1 and Saturday = 6. This will work for any historical or future Julian Day, whether or not it can be converted to a NaiveDateTime object.

### julian_day_to_datetime(jd: f64) -> Result<NaiveDateTime, ParsedError>
This returns a result type consistent with other Rust parsers, while its implementation for chrono::NaiveDateTime returns an option in keeping with other parser methods in the same library.

### datetime_to_julian_day(dt_str: &str) -> Result<f64, ParsedError>
Convert a fuzzy ISO-8601-like string to a Julian day value. This returns a result type consistent with other Rust parsers. The approximate **YYYY-mm-dd HH:MM:SS** date-time string is corrected to a plain ISO-8601 format without milliseconds or timezone suffixes. This is equivalent to instantiating a NaiveDateTime object from *NaiveDateTime::from_fuzzy_iso_string()* and then using the *date_time.to_jd()* method;

### iso_fuzzy_string_to_datetime(dt: &str) -> Result<NaiveDateTime, ParsedError>
Convert a fuzzy ISO-8601-like string to a NaiveDateTime. This returns a result type consistent with other Rust parsers, while its implementation for chrono::NaiveDateTime returns an option in keeping with other constructors in the same library. NB: Before version 0.3 this return an option

## Traits

## JulianDay
must implement:
- ```to_jd(&self) -> f64```
- ```from_jd(jd: f64) -> Option<Self>```

## FromFuzzyISOString
must implement:
- ```from_fuzzy_iso_string(&self, dt_str: &str) -> Option<Self>```

## WeekdayIndex
must implement:
- ```weekday_index(&self, offset_secs: i32) -> u8```

If the solar or standard local timezone offset is known, this calculates the weekday index (Sunday = 0, Monday = 1 ... Saturday = 6) for timezone-neutral DateTime objects. The solar timezone offset in seconds can be calculated from the longitude as 1º = 240 seconds, e.g. -3º (or 3ºW) would be -720.

## Usage

```rust
use chrono::NaiveDateTime;
use julian_day_converter::*;

fn main() {
    // Convert a sample Julian Day value to a valid NaiveDateTime object and then use to_jd() for interoperability
    // with astronomical applications
    // The return value is a result consistent with other parse functions
    // julian_day_to_datetime(jd_value) is equivalent to NaiveDateTime::from_jd(jd_value)
    let sample_julian_day = 2459827.25;
    if let Ok(date_time) = julian_day_to_datetime(sample_julian_day) {
      let formatted_date_time_string = date_time.format("%Y-%m-%dT%H:%M:%S").to_string();
      println!("The Julian day {} is {} as ISO date-time", sample_julian_day, formatted_date_time_string);
    }
    
    // Convert an approximate date-time string to a valid NaiveDateTime object and then use to_jd() for interoperability
    // with astronomical applications
    // The return value is a result consistent with other parse functions
    let approx_date_time = "2023-11-09 15"; // 3pm 
    if let Some(date_time) = NaiveDateTime::from_fuzzy_iso_string(approx_date_time) {
      let formatted_date_time_string = date_time.format("%Y-%m-%dT%H:%M:%S").to_string();
      println!("The input time of `{}` is assumed to be {} UTC and in Julian Days is {}", approx_date_time, formatted_date_time_string, date_time.to_jd());
    }
  
    let historical_jd = 2334317.39336563;
    // Convert to a NaiveDateTime object and then apply its format method
    // The return value is an options consistent with other chrono::NaiveDateTime constructors
    if let Some(date_time) = NaiveDateTime::from_jd(historical_jd) {
      let formatted_date_time_string = date_time.format("%Y-%m-%dT%H:%M:%S").to_string();
      println!("The meteorite landed on Earth at {} Julian days or {}", historical_jd, formatted_date_time_string);
    }

    let prehistoric_julian_day = -190338.875;
    // Convert to a NaiveDateTime object and then apply its format method
    // The return value is a result consistent with other parse functions
    if let Some(date_time) = NaiveDateTime::from_jd(prehistoric_julian_day) {
      let formatted_date_time_string = date_time.format("%Y-%m-%dT%H:%M:%S").to_string();
      println!("An asteroid hit the Earth at {} Julian days or {}", historical_jd, formatted_date_time_string);
    }

    let unix_timestamp = 169938309;
    // does not require conversion to a NaiveDateTime object
    let jd = unixtime_to_julian_day(unix_timestamp);
    println!("All valid 64 bit integers can be converted to Julian days, e.g. {} is {} Julian days", unix_timestamp, jd);
    
    let julian_day = 2134937.3937f64;
    // does not require conversion to a NaiveDateTime object
    let ts = julian_day_to_unixtime(julian_day);
    println!("Some historical and far-future Julian days may yield very big timestamp values e.g. {} is {} as a unix timestamp", julian_day, ts);
  
  }

```
