[![mirror](https://img.shields.io/badge/mirror-github-blue)](https://github.com/neilg63/julian_day_converter)
[![crates.io](https://img.shields.io/crates/v/julian_day_converter.svg)](https://crates.io/crates/julian_day_converter)
[![docs.rs](https://docs.rs/julian_day_converter/badge.svg)](https://docs.rs/julian_day_converter)


# Julian Day Compatibility Methods for Chrono

This library provides compatibility with astronomical applications that use Julian Days as 64-bit floats. A *Julian Day* represents the number of days since the beginning of the Julian period, which started at 12 noon on November 24, 4713 BCE (-4713-11-24T12:00:00 UTC). Julian Days facilitate calculations over extended periods and should not be confused with the Julian Calendar, which affects leap year rules.

This crate adds two traits to supplement Rust's standard datetime crate, Chrono, as well as four standalone functions to convert directly to and from Unix timestamps. All date-time values are in UTC but can be converted to a timezone-aware *chrono::DateTime*.

Please note that Julian Day values as 64-bit floats are always rounded to the nearest millisecond with *chrono::NaiveDateTime*. Prior to version 0.4.3, they were rounded to the nearest second. 

## Direct Functions

- `unixtime_to_julian_day(ts: i64) -> f64`
- `unix_millis_to_julian_day(ts: i64) -> f64`

These functions convert Unix timestamps directly to Julian days as a 64-bit float, compatible with many astronomical applications. The first converts Unix timestamps as seconds and the latter as milliseconds.

- `julian_day_to_unixtime(jd: f64) -> i64`
- `julian_day_to_unix_millis(jd: f64) -> i64`

These functions convert a Julian Day to a signed 64-bit integer that represents the time elapsed either in seconds or milliseconds since the start of 1970 UTC.

If the second timestamp has to be cast to a 32-bit integer, dates before 1902 and after 2038 will be out of range. With 64-bit integers, the range is approximately ± 292 million years with milliseconds and 292 billion years with seconds. 

- `julian_day_to_weekday_index(jd: f64, offset_secs: i32) -> u8`

Calculates the weekday index, where Sunday = 0, Monday = 1, and Saturday = 6. This will work for any historical or future Julian Day, whether or not it can be converted to a NaiveDateTime object.

- `julian_day_to_datetime(jd: f64) -> Result<NaiveDateTime, ParsedError>`

Converts a valid Julian Day within a range of -9999-01-01 to 9999-12-31 to NaiveDateTime, assumed to be UTC. This is the same as `NaiveDateTime::from_jd()`, except it returns a `Result` rather than an `Option`.

## Traits

### JulianDay
- `to_jd(&self) -> f64`
- `from_jd(jd: f64) -> Option<Self>`

These methods convert to and from Julian day values to millisecond precision.

NB: Prior to version 0.4.3, all values were rounded to exact seconds.

### WeekdayIndex
- `weekday_index(&self, offset_secs: i32) -> u8`
- `weekday_number(&self, offset_secs: i32) -> u8`

If the solar or standard local timezone offset is known, this calculates the weekday index (Sunday = 0, Monday = 1 ... Saturday = 6) for timezone-neutral DateTime objects. The solar timezone offset in seconds can be calculated from the longitude as 1º = 240 seconds, e.g., -3º (or 3ºW) would be -720.
This is consistent with Chrono's [`%w` format specifier](https://docs.rs/chrono/latest/chrono/format/strftime/index.html) and with JavaScript's `Date.getDay()` method.
The alternative `weekday_number(offset_secs: i32)` method returns a number from Monday = 1 to Sunday = 7, consistent with like-named methods in Java, C#, and ISO 8601. However, Python's `datetime.weekday()` is zero-indexed from Monday.

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
      let formatted_date_time_string = date_time.format("%Y-%m-%dT%H:%M:%S3fZ").to_string();
      println!("The Julian day {} is {} as ISO date-time", sample_julian_day, formatted_date_time_string);
      // Should print:
      // The Julian day 2459827.25 is 2022-09-04T18:00:00.000Z as ISO date-time
    }
  
    let historical_jd = 2334317.39336563;
    // Convert to a NaiveDateTime object and then apply its format method
    // The return value is an option consistent with other chrono::NaiveDateTime constructors
    if let Some(date_time) = NaiveDateTime::from_jd(historical_jd) {
      let formatted_date_time_string = date_time.format("%Y-%m-%dT%H:%M:%S%.3fZ").to_string();
      println!("The meteorite landed on Earth at {} Julian days or {}", historical_jd, formatted_date_time_string);
      // should print: 
      // The meteorite landed on Earth at 2334317.39336563 Julian days or 1679-01-15T21:26:26.791Z
    }

    let historical_datetime = "1876-09-25T15:45:00";
    // Convert to a NaiveDateTime object and then apply its format method
    // The return value is an option consistent with other chrono::NaiveDateTime constructors
    if let Ok(date_time) = NaiveDateTime::from_str(historical_datetime) {
      let jd = date_time.to_jd();
      println!("The date {} is {} in Julian days", historical_datetime, jd);
    }

    let prehistoric_julian_day = -190338.875;
    // Convert to a NaiveDateTime object and then apply its format method
    // The return value is a result consistent with other parse functions
    if let Some(date_time) = NaiveDateTime::from_jd(prehistoric_julian_day) {
      let formatted_date_time_string = date_time.format("%Y-%m-%dT%H:%M:%S").to_string();
      println!("An asteroid hit the Earth at {} Julian days or {}", prehistoric_julian_day, formatted_date_time_string);
    }

    let unix_timestamp = 169938309;
    // does not require conversion to a NaiveDateTime object
    let jd = unixtime_to_julian_day(unix_timestamp);
    println!("All valid 64-bit integers can be converted to Julian days, e.g. {} is {} Julian days", unix_timestamp, jd);
    
    let julian_day = 2134937.3937f64;
    // does not require conversion to a NaiveDateTime object
    let ts = julian_day_to_unixtime(julian_day);
    println!("Some historical and far-future Julian days may yield very big timestamp values e.g. {} is {} as a Unix timestamp", julian_day, ts);
}
```
### Constants

- `JULIAN_DAY_UNIX_EPOCH_DAYS`: **2440587.5**. The Julian day value at 1970-01-01 00:00:00 UTC

- `JULIAN_DAY_UNIX_EPOCH_WEEKDAY`: *4*. The weekday index for the Unix Epoch (1970-01-01 UTC) is Thursday (4)
- `JULIAN_DAY_MIN_SUPPORTED`: **-1_930_999.5**. Minimum Julian Day value date-time conversion via Chrono, -4713-11-24 12:00:00 UTC
- `JULIAN_DAY_MAX_SUPPORTED`: **5_373_484.499999**. Max Julian day value via Chrono, i.e. 9999-12-31 23:59:59 UTC

---

### Release Notes
#### 0.4.4 
*Chrono 0.4.31+* is now added as a dependency with `default-features` set to false, letting developers decide which optional dependencies they need. However, the *std* feature set is added as a dev-dependency for the integration test and some of the above examples with the *format* method. Thanks to [Doug A](https://github.com/DougAnderson444) for that.

#### 0.4.3 
Two new functions were added to convert to and from unix timestamps as milliseconds and the core `NaiveDateTime::to_jd()` and `NaiveDateTime::from_jd(jd: f64)` methods now use `DateTime::from_timestamp_millis(millis: i64)` for millisecond precision.

#### 0.4.2 
0.4.0 streamlined the crate (see notes on earlier versions). Subsequent updates contain editorial changes only.

#### 0.3.3
The core `to_jd()` and `from_jd(jd: f64)` methods have been updated to ensure future compatibility with the *chrono* crate by replacing all calls to deprecated methods with the newer methods introduced in version 0.4.31, which is now the minimum supported version.

`NaiveDateTime::from_jd(jd: f64)` now only works within a range from `-9999-01-01T00:00:00` to `9999-12-31T23:59:59`. However, `unixtime_to_julian_day(timestamp: i64)` and `julian_day_to_unixtime(jd: f64)` work within a much wider range supported by i64 and f64 respectively.

The supplementary fuzzy-datetime conversion functions have been marked as *deprecated* and made available with a broader range of options in [fuzzy-datetime](https://crates.io/crates/fuzzy-datetime).

A similar [julianday](https://crates.io/crates/julianday) crate exists to handle Julian days as integers and converts them to *chrono::NaiveDate* only. I developed this crate primarily to ensure interoperability with an [Astrological API server](https://github.com/neilg63/astro-calc-api) that leverages the [Swiss Ephemeris](https://github.com/aloistr/swisseph) calculation engine.

---

## Earlier versions

Versions before 0.4.0 had a date/time interpretation and correction function and trait able to handle ISO-like input strings with varying degrees of approximation. These have now moved to [fuzzy-datetime](https://crates.io/crates/fuzzy-datetime).

However, Chrono's `NaiveDateTime::parse_from_str(date_str: &str, fmt: &str)` is versatile enough for most purposes if all your input times follow the same format, as detailed in this overview of the crate's [strftime Module](https://docs.rs/chrono/latest/chrono/format/strftime/index.html#specifiers).

```rust
let historic_time_str = "04/11/1877 18:00";
let target_julian_day = 2406928.25;
let datetime_format = "%d/%m/%Y %H:%M";
if let Ok(dt) = NaiveDateTime::parse_from_str(&historic_time_str, datetime_format) {
  println!("{} is {} in Julian days", historic_time_str, dt.to_jd() );
  // should read: 04/11/1877 18:00 is 2406928.25 in Julian days
}
```
