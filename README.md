# Julian Day Compatibility methods for Chrono

This library provides compatibility with astronomical applications that use Julian Days as 64-bit floats. 

It adds a Trait and 4 utility methods to the Rust's standard datetime crate, Chrono, and as well as standalone functions to convert to and from Unix timestamps. All date-time objects are UTC and may be converted to chrono::DateTime but adding a time zone.

## Direct functions

### unixtime_to_julian_day(ts: i64) -> f64
Converts a unix timestamp directly to Julian Day as a 64-bit float, compatibile with many astronomical applications.

### julian_day_to_unixtime(jd: f64) -> i64
Converts a Julian Day as a signed 64-bit integer. As many date-time functions require timestamps as 32-bit integers, some dates before 1901 and after 2038 may be out of range

### julian_day_to_weekday_index(jd: f64, offset_secs: i32) -> u8
Calculates the weekday index, where Sunday = 0, Monday = 1 and Saturday = 6. This will work for any historical or future Julian Day, whether or not it can be converted to a NaiveDateTime object.

## Traits

## JulianDay
must implements:
- to_jd() -> f64
- from_jd(jd: f64) -> Result<Self, Error>
- from_fuzzy_iso_string(dt_str: &str) -> Result<Self, Error>
- weekday_index(&self, offset_secs: i32) -> u8

## Implentation for chrono::NaiveDateTime

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
  if let Ok(date_time) = NaiveDateTime::from_fuzzy_iso_string(approx_date_time) {
    let formatted_date_time_string = date_time.format("%Y-%m-%dT%H:%M:%S").to_string();
    println!("The input time of `{}` is assumed to be {} UTC and in Julian Days is {}", approx_date_time, formatted_date_time_string, date_time.to_jd());
  }

  let historical_jd = 2334317.39336563;
  // Convert to a NaiveDateTime object and then apply its format method
  // The return value is a result consistent with other parse functions
  if let Ok(date_time) = NaiveDateTime::from_jd(historical_jd) {
    let formatted_date_time_string = date_time.format("%Y-%m-%dT%H:%M:%S").to_string();
    println!("The meteorite landed on Earth at {} Julian days or {}", historical_jd, formatted_date_time_string);
  }

  let unix_timestamp = 169938309;
  // does not require conversion to a NaiveDateTime object
  let jd = unixtime_to_julian_day(unix_timestamp);
  println!("All valid 64 bit integers can be converted to Julian days, e.g. {} is {} Julian days", unix_timestamp, jd);
  
  let julian_day = 2134937.3937f64;
  // does not require conversion to a NaiveDateTime object
  let ts = julian_day_to_unixtime(julian_day);
  println!("Some historical Julian days may yield very high or low unix timestamp values e.g. {} is {} as a unix timestamp", julian_day, ts);

}

```