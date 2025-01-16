use std::str::FromStr;

use julian_day_converter::*;
use chrono::{NaiveDateTime, NaiveDate, NaiveTime};

#[cfg(test)]

#[test]
fn test_basic_date() {
    let julian_day = 2459827.25;
    let expected_datetime_utc = "2022-09-04T18:00:00";
    let result = NaiveDateTime::from_jd(julian_day);
    let incorrect_datetime = NaiveDateTime::new(NaiveDate::from_ymd_opt(1970,1, 1).unwrap(),NaiveTime::from_hms_opt(0,0,0).unwrap());
    let formatted_datetime = result.unwrap_or(incorrect_datetime).format("%Y-%m-%dT%H:%M:%S").to_string();
    assert_eq!(formatted_datetime, expected_datetime_utc.to_string());

    let historical_datetime = "1876-09-25T12:00:00";
    let date_time = NaiveDateTime::from_str(historical_datetime).unwrap();
    let jd = date_time.to_jd();
    assert_eq!(jd,  2406523.0);
}

#[test]
fn test_parse_from_str() {
    
    let historic_time_str = "04/11/1877 18:00";
    let target_julian_day = 2406928.25;
    let result = NaiveDateTime::parse_from_str(&historic_time_str, "%d/%m/%Y %H:%M");
    assert_eq!(result.unwrap().to_jd(), target_julian_day);
}


#[test]
fn test_first_julian_day() {
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
    let julian_day: f64 = 2459827.25;
    let expected_unixtime: i64 = 1662314400;
    let result: i64 = julian_day_to_unixtime(julian_day);
    assert_eq!(expected_unixtime, result);
}

#[test]
fn test_julian_day_datetime_utc() {
  let datetime = julian_day_to_datetime(2460193.875).ok().unwrap();
  let expected_datetime_string = "2023-09-06T09:00:00".to_string();
  let result = datetime.format("%Y-%m-%dT%H:%M:%S").to_string();
  assert_eq!(expected_datetime_string, result);
}


#[test]
fn test_julian_day_range() {
  assert_eq!(julian_day_to_datetime(JULIAN_DAY_MIN_SUPPORTED).ok().unwrap().format("%Y-%m-%dT%H:%M:%S").to_string(), "-9999-01-01T00:00:00".to_string());
  assert_eq!(julian_day_to_datetime(JULIAN_DAY_MAX_SUPPORTED).ok().unwrap().format("%Y-%m-%dT%H:%M:%S").to_string(), "9999-12-31T23:59:59".to_string());
}

// "This test is for a deprecated function and will be removed in version 0.4.0"
#[test]
fn test_weekday_index() {
    let incorrect_datetime = NaiveDateTime::new(NaiveDate::from_ymd_opt(1970,1, 1).unwrap(),NaiveTime::from_hms_opt(0,0,0).unwrap());
    let expected_weekday_index = 0; // Sunday = 0
    let datetime_utc = "2022-09-04T18:00:00"; // Sunday
    let dt = NaiveDateTime::from_str(str::trim(&datetime_utc)).unwrap_or(incorrect_datetime);
    // Default to UTC
    assert_eq!(dt.weekday_index(0), expected_weekday_index);
    // Should be the next day at UTC+10 (e.g. Australia)
    assert_eq!(dt.weekday_index(36000), expected_weekday_index + 1);

    // Test with Java/C3/Puython style day of week index
    let python_day_of_week = 6; // Sunday = 6
    assert_eq!(dt.day_of_week(0), python_day_of_week);


    assert_eq!(dt.weekday_index(0), dt.format("%w").to_string().parse::<u8>().unwrap());
}


