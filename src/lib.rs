use chrono::NaiveDateTime;

const JULIAN_DAY_UNIX_EPOCH_DAYS: f64 = 2440587.5; // 1970-01-01 00:00:00 UTC

const JULIAN_DAY_UNIX_EPOCH_WEEKDAY: u8 = 4; // 1970-01-01 00:00:00 was a Wednesday UTC

/*
  Convert a unix timestamp as a 64 bit integer to julian days as a 64-bit float
*/
pub fn unixtime_to_julian_day(ts: i64) -> f64 {
  (ts as f64 / 86_400f64) + JULIAN_DAY_UNIX_EPOCH_DAYS
}

// convert julian day as 64-bit float to unix timestamp seconds as a signed 64 bit integer
pub fn julian_day_to_unixtime(jd: f64) -> i64 {
  ((jd - JULIAN_DAY_UNIX_EPOCH_DAYS) * 86400f64) as i64
}

// convert julian day as 64-bit float to a timezone-neutral chrono::NaiveDateTime object
pub fn julian_day_to_datetime(jd: f64) -> Result<NaiveDateTime, &'static str> {
  if let Some(dt) = NaiveDateTime::from_timestamp_opt(julian_day_to_unixtime(jd), 0) {
    Ok(dt)
  } else {
    Err("Julian Day out of range")
  }
}

// convert ISO-8601-like string to a Julian days as f64 (64-bit float) via chrono::NaiveDateTime
pub fn datetime_to_julian_day(dt_str: &str) -> Result<f64, &'static str> {
  if let Some(dt) = iso_fuzzy_string_to_datetime(dt_str) {
      Ok(unixtime_to_julian_day(dt.timestamp()))
  } else {
      Err("invalid ISO date-compatible format")
  }
}


pub trait JulianDay {
  
  /*
  * Convert from DateTime Object to a Julian Day as f64
  */
  fn to_jd(&self) -> f64;
  
  /*
  * Convert from a Julian Day as f64 to DateTime Object
  */
  fn from_jd(jd: f64) -> Option<Self> where Self: Sized;
  
   /*
    * Convert from any ISO-8601-like string to a DateTime object
    * Valid formats
    * Full date-time: e.g. 2023-11-15T17:53:26
    * with optional millisecends (ignored): e.g. 2023-11-15T17:53:26.383Z
    * with space rather than T: 2023-11-15T17:53:26
    * without seconds: 2023-11-15T17:53 (rounded to the minute start)
    * without minutes: 2023-11-15T17 (rounded to the hour start)
    * without hour: 2023-11-15 (rounded to the day start)
    * without the month day: 2023-11 (rounded to the month start)
    * Year only: 2023 (rounded to the year start)
  */
  fn from_fuzzy_iso_string(dt_str: &str) -> Option<NaiveDateTime>;

  /*
  * Current weekday index, where Sunday = 0, Monday = 1 and Saturday = 6
  * The local weekday index depends on the timezone offset in seconds where
  * West of UTC => Negative hour offset * 3600, e.g. -18000 => UTC-5
  * East of UTC => Positive hour offset * 3600, e.g. +3600 => UTC+1
  */
  fn weekday_index(&self, offset_secs: i32) -> u8;
}

impl JulianDay for NaiveDateTime {
  fn to_jd(&self) -> f64 {
    unixtime_to_julian_day(self.timestamp())
  }

  fn from_jd(jd: f64) -> Option<Self> {
    if let Ok(dt) = julian_day_to_datetime(jd) {
      Some(dt)
    } else {
      None
    }
  }

  fn from_fuzzy_iso_string(dt_str: &str) -> Option<NaiveDateTime> {
    iso_fuzzy_string_to_datetime(dt_str)
  }

  fn weekday_index(&self, offset_secs: i32) -> u8 {
    julian_day_to_weekday_index(self.to_jd(), offset_secs)
  }
}

/*
* Calculate the weekday index from a given Julian Day with timezone offsets in seconds
*/
pub fn julian_day_to_weekday_index(jd: f64, offset_secs: i32) -> u8 {
	let ref_jd = jd + (offset_secs as f64 / 86400f64);
	let days_since_1970 = ref_jd - JULIAN_DAY_UNIX_EPOCH_DAYS as f64;
  let ds = (days_since_1970 as u64) % 7;
  let days_since_index = if ds < 7 { ds as u8 } else { 0u8 };
	(days_since_index + JULIAN_DAY_UNIX_EPOCH_WEEKDAY) % 7
}

/**
 * Utility function to convert any ISO-8601-like date string to a Chrono NaiveDateTime object
 * This function accepts YYYY-mm-dd HH:MM:SS separated by a space or letter T and with or without hours, minutes or seconds.
 * Missing time parts will be replaced by 00, hence 2022-06-23 will be 2022-06-23 00:00:00 UTC and 22-06-23 18:20 will be 2022-06-23 18:30:00
 */
pub fn iso_fuzzy_string_to_datetime(dt: &str) -> Option<NaiveDateTime> {
  let dt_base = if dt.contains('.') { dt.split(".").next().unwrap() } else { dt };
  let clean_dt = dt_base.replace("T", " ").trim().to_string();
  let mut dt_parts = clean_dt.split(" ");
  let mut date_part = if clean_dt.clone().contains(" ") { dt_parts.next().unwrap().to_string() } else { clean_dt.clone() };
  let mut date_parts: Vec<&str> = if date_part.len() > 1 { date_part.split("-").into_iter().collect() } else { vec!("2000", "01", "01") };
  if date_parts.len() < 2 { 
    date_parts.push("01");
   }
  if date_parts.len() < 3 { 
    date_parts.push("01");
  }
  date_part = format!("{}-{}-{}", date_parts[0], date_parts[1], date_parts[2]);
  let time_part = if clean_dt.clone().contains(" ") { dt_parts.next().unwrap().to_string() } else { "".to_string() };
  let mut time_parts = if time_part.len() > 1 { time_part.split(":").into_iter().collect() } else { vec!("00", "00", "00") };
  let num_time_parts = time_parts.len();
  if num_time_parts < 3 { 
    time_parts.push("00");
  }
  if num_time_parts < 2 {
    time_parts.push("00");
  }
  let formatted_str = format!("{} {}:{}:{}", date_part, time_parts[0], time_parts[1], time_parts[2]);
  if let Ok(dt) = NaiveDateTime::parse_from_str(formatted_str.as_str(), "%Y-%m-%d %H:%M:%S") {
    Some(dt)
  } else {
    // "invalid ISO-compatibile format")
    None
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use chrono::{NaiveDate, NaiveTime};
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

}
