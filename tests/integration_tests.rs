use std::str::FromStr;

use julian_day_converter::*;
use chrono::{Datelike, NaiveDate, NaiveDateTime, NaiveTime, Timelike};

#[cfg(test)]

#[test]
fn test_basic_date() {
    // Test conversion from Julian Day to NaiveDateTime and formatting
    let julian_day = 2459827.25;
    let expected_datetime_utc = "2022-09-04T18:00:00";
    let result = NaiveDateTime::from_jd(julian_day);
    let incorrect_datetime = NaiveDateTime::new(NaiveDate::from_ymd_opt(1970,1, 1).unwrap(),NaiveTime::from_hms_opt(0,0,0).unwrap());
    let formatted_datetime = result.unwrap_or(incorrect_datetime).format("%Y-%m-%dT%H:%M:%S").to_string();
    assert_eq!(formatted_datetime, expected_datetime_utc.to_string());

    // Test conversion from NaiveDateTime to Julian Day
    let historical_datetime = "1876-09-25T12:00:00";
    let date_time = NaiveDateTime::from_str(historical_datetime).unwrap();
    let jd = date_time.to_jd();
    assert_eq!(jd,  2406523.0);
}

#[test]
fn test_parse_from_str() {
    // Test parsing from string to NaiveDateTime and conversion to Julian Day
    let historic_time_str = "04/11/1877 18:00";
    let target_julian_day = 2406928.25;
    let result = NaiveDateTime::parse_from_str(&historic_time_str, "%d/%m/%Y %H:%M");
    assert_eq!(result.unwrap().to_jd(), target_julian_day);
}

#[test]
fn test_first_julian_day() {
    // Test conversion of the first Julian Day to NaiveDateTime
    let julian_day = 0f64;
    let expected_datetime_utc = "-4713-11-24T12:00:00"; // BCE
    let result = NaiveDateTime::from_jd(julian_day);
    let incorrect_datetime = NaiveDateTime::new(NaiveDate::from_ymd_opt(1970,1, 1).unwrap(),NaiveTime::from_hms_opt(0,0,0).unwrap());
    let formatted_datetime = result.unwrap_or(incorrect_datetime).format("%Y-%m-%dT%H:%M:%S").to_string();
    assert_eq!(formatted_datetime, expected_datetime_utc.to_string());
  
    let ancient_jd: f64 = 0.0;
    assert_eq!(julian_day_to_datetime(ancient_jd).ok().unwrap().format("%Y-%m-%dT%H:%M:%S").to_string(), "-4713-11-24T12:00:00".to_string());
}

#[test]
fn test_unixtime_conversion() {
    // Test conversion from Julian Day to Unix timestamp
    let julian_day: f64 = 2459827.25;
    let expected_unixtime: i64 = 1662314400;
    let result: i64 = julian_day_to_unixtime(julian_day);
    assert_eq!(expected_unixtime, result);
}

#[test]
fn test_julian_day_datetime_utc() {
    // Test conversion from Julian Day to NaiveDateTime and formatting
    let datetime = julian_day_to_datetime(2460193.875).ok().unwrap();
    let expected_datetime_string = "2023-09-06T09:00:00".to_string();
    let result = datetime.format("%Y-%m-%dT%H:%M:%S").to_string();
    assert_eq!(expected_datetime_string, result);
}

#[test]
fn test_julian_day_range() {
    // Test conversion of minimum and maximum supported Julian Days to NaiveDateTime
    assert_eq!(julian_day_to_datetime(JULIAN_DAY_MIN_SUPPORTED).ok().unwrap().format("%Y-%m-%dT%H:%M:%S").to_string(), "-9999-01-01T00:00:00".to_string());
    assert_eq!(julian_day_to_datetime(JULIAN_DAY_MAX_SUPPORTED).ok().unwrap().format("%Y-%m-%dT%H:%M:%S").to_string(), "9999-12-31T23:59:59".to_string());
}

#[test]
fn test_weekday_index() {
    // Test calculation of weekday index with different timezone offsets
    let incorrect_datetime = NaiveDateTime::new(NaiveDate::from_ymd_opt(1970,1, 1).unwrap(),NaiveTime::from_hms_opt(0,0,0).unwrap());
    let expected_weekday_index = 0; // Sunday = 0
    let datetime_utc = "2022-09-04T18:00:00"; // Sunday
    let dt = NaiveDateTime::from_str(str::trim(&datetime_utc)).unwrap_or(incorrect_datetime);
    // Default to UTC
    assert_eq!(dt.weekday_index(0), expected_weekday_index);
    // Should be the next day at UTC+10 (e.g. Australia)
    assert_eq!(dt.weekday_index(36000), expected_weekday_index + 1);

    // Test with Java/C#/Python style day of week index
    let iso_weekday_number = 7; // Sunday = 7 with UTC offset
    assert_eq!(dt.weekday_number(0), iso_weekday_number);

    assert_eq!(dt.weekday_index(0), dt.format("%w").to_string().parse::<u8>().unwrap());
}

#[test]
fn test_year_range_with_i64_timestamps() {
    // Test conversion of maximum and minimum i64 timestamps to Julian Days and year ranges
    let max_i64 = i64::MAX;
    let min_i64 = i64::MIN;

    let julian_i64_max = unix_millis_to_julian_day(max_i64);
    let max_years = julian_i64_max / 365.25;
    
    let julian_i64_min = unix_millis_to_julian_day(min_i64);
    let min_years = julian_i64_min / 365.25;

    assert!(max_years >= 292_277_704.5 && max_years <= 292_277_705.5);
    assert!(min_years >= -292_264_341.5 && min_years <= -292_264_340.5);
}

#[test]
fn test_milliseconds() {
    // Test formatting of Julian Day to ISO 8601 string with milliseconds and timezone suffix
    let jd = 2499827.2939383729278;
    let iso_8601with_millis_and_tz_format = "%Y-%m-%dT%H:%M:%S%.3fZ";

    // Format with milliseconds and Z timezone suffix compatible with JavaScript Date object constructors
    // to ensure the UTC datetime string is not offset by local time
    let formatted_datetime_string = NaiveDateTime::from_jd(jd).unwrap().format(iso_8601with_millis_and_tz_format).to_string();

    // Extract a slice of the last five characters e.g. ".275Z"
    // and compare character by character
    let last_five_chars = &formatted_datetime_string[formatted_datetime_string.len()-5..].chars().collect::<Vec<char>>();
    assert!(last_five_chars[0] == '.' && last_five_chars[1].is_digit(10) && last_five_chars[2].is_digit(10) && last_five_chars[3].is_digit(10) && last_five_chars[4] == 'Z');

    // Check that the milliseconds are not all zeros
    let millis_slice = last_five_chars[1..4].iter().collect::<String>();
    assert_ne!(millis_slice, "000");
}
