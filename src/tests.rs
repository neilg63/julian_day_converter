use crate::*;
use chrono::{NaiveDateTime, NaiveDate, NaiveTime};

#[test]
fn test_basic_date() {
    let julian_day = 2459827.25;
    let expected_datetime_utc = "2022-09-04T18:00:00";
    let result = NaiveDateTime::from_jd(julian_day);
    let incorrect_datetime = NaiveDateTime::new(NaiveDate::from_ymd_opt(1970,1, 1).unwrap(),NaiveTime::from_hms_opt(0,0,0).unwrap());
    let formatted_datetime = result.unwrap_or(incorrect_datetime).format("%Y-%m-%dT%H:%M:%S").to_string();
    assert_eq!(formatted_datetime, expected_datetime_utc.to_string());
}

#[test]
fn test_julian_day() {
    let incorrect_datetime = NaiveDateTime::new(NaiveDate::from_ymd_opt(1970,1, 1).unwrap(),NaiveTime::from_hms_opt(0,0,0).unwrap());
    let expected_julian_day = 2459827.25;
    let datetime_utc = "2022-09-04T18:00:00";
    let dt = NaiveDateTime::from_fuzzy_iso_string(&datetime_utc).unwrap_or(incorrect_datetime);
    assert_eq!(dt.to_jd(), expected_julian_day);
}

#[test]
fn test_first_julian_day() {
  let julian_day = 0f64;
  let expected_datetime_utc = "-4713-11-24T12:00:00"; // BCE
  let result = NaiveDateTime::from_jd(julian_day);
  let incorrect_datetime = NaiveDateTime::new(NaiveDate::from_ymd_opt(1970,1, 1).unwrap(),NaiveTime::from_hms_opt(0,0,0).unwrap());
  let formatted_datetime = result.unwrap_or(incorrect_datetime).format("%Y-%m-%dT%H:%M:%S").to_string();
  assert_eq!(formatted_datetime, expected_datetime_utc.to_string());
}


#[test]
fn test_weekday_index() {
    let incorrect_datetime = NaiveDateTime::new(NaiveDate::from_ymd_opt(1970,1, 1).unwrap(),NaiveTime::from_hms_opt(0,0,0).unwrap());
    let expected_weekday_index = 0; // Sunday = 0
    let datetime_utc = "2022-09-04T18:00:00"; // Sunday
    let dt = NaiveDateTime::from_fuzzy_iso_string(&datetime_utc).unwrap_or(incorrect_datetime);
    // Default to UTC
    assert_eq!(dt.weekday_index(0), expected_weekday_index);
    // Should be the next day at UTC+10 (e.g. Australia)
    assert_eq!(dt.weekday_index(36000), expected_weekday_index + 1);
}

#[test]
fn test_fuzzy_datetime() {
    let incorrect_datetime = NaiveDateTime::new(NaiveDate::from_ymd_opt(1970,1, 1).unwrap(),NaiveTime::from_hms_opt(0,0,0).unwrap());
    let input_datetime_utc = "2022-09-04 18";
    let expected_datetime_utc = "2022-09-04T18:00:00";
    let dt = NaiveDateTime::from_fuzzy_iso_string(&input_datetime_utc).unwrap_or(incorrect_datetime);
    let formatted_datetime = dt.format("%Y-%m-%dT%H:%M:%S").to_string();
    assert_eq!(formatted_datetime, expected_datetime_utc.to_owned());
}

#[test]
fn test_unixtime_conversion() {
    let julian_day: f64 = 2459827.25;
    let expected_unixtime: i64 = 1662310800;
    let result: i64 = julian_day_to_unixtime(julian_day);
    assert_eq!(expected_unixtime, result);
}
