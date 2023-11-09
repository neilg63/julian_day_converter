use chrono::NaiveDateTime;

/// Public constant that may be useful to library users
pub const JULIAN_DAY_UNIX_EPOCH_DAYS: f64 = 2440587.5; // 1970-01-01 00:00:00 UTC

const JULIAN_DAY_UNIX_EPOCH_WEEKDAY: u8 = 4; // 1970-01-01 00:00:00 was a Wednesday UTC

///
///  Convert a unix timestamp as a 64 bit integer to julian days as a 64-bit float
/// 
/// ### Example:
/// ```
/// use julian_day_converter::*;
/// 
/// let julian_day: f64 = unixtime_to_julian_day(1672929282);
/// ```
///
pub fn unixtime_to_julian_day(ts: i64) -> f64 {
  (ts as f64 / 86_400f64) + JULIAN_DAY_UNIX_EPOCH_DAYS
}

/// convert julian day as 64-bit float to unix timestamp seconds as a signed 64 bit integer
/// 
/// ### Example:
/// ```
/// use julian_day_converter::*;
/// 
/// let julian_day: f64 = 2460258.488768587;
/// let unix_time: i64 = julian_day_to_unixtime(julian_day);
/// ```
///
pub fn julian_day_to_unixtime(jd: f64) -> i64 {
  ((jd - JULIAN_DAY_UNIX_EPOCH_DAYS) * 86400f64) as i64
}

/// convert julian day as 64-bit float to a timezone-neutral chrono::NaiveDateTime object
/// 
/// ### Example:
/// ```
/// use chrono::NaiveDateTime;
/// use julian_day_converter::*;
/// 
/// let julian_day: f64 = 2460258.488768587;
/// if let Ok(date_time) = julian_day_to_datetime(julian_day) {
///   println!("The date time is {}", date_time.format( "%Y-%m-%d %H:%M:%S"));
/// }
/// ```
///
pub fn julian_day_to_datetime(jd: f64) -> Result<NaiveDateTime, &'static str> {
  if let Some(dt) = NaiveDateTime::from_timestamp_opt(julian_day_to_unixtime(jd), 0) {
    Ok(dt)
  } else {
    Err("Julian Day out of range")
  }
}

/// convert ISO-8601-like string to a Julian days as f64 (64-bit float) via chrono::NaiveDateTime
/// 
/// ### Example:
/// ```
/// use chrono::NaiveDateTime;
/// use julian_day_converter::*;
/// 
/// let approx_date_time = "2023-10-29 19:47";
/// if let Ok(julian_day) = datetime_to_julian_day(approx_date_time) {
///   println!("The approximate date time {} is {} in julian days", approx_date_time, julian_day);
/// }
/// ```
///
pub fn datetime_to_julian_day(dt_str: &str) -> Result<f64, &'static str> {
  if let Some(dt) = iso_fuzzy_string_to_datetime(dt_str) {
      Ok(unixtime_to_julian_day(dt.timestamp()))
  } else {
      Err("invalid ISO date-compatible format")
  }
}

///
/// This trait may be implemented by any Date or DateTime object
/// An implementation for chrono::NaiveDateTime is provided below
///
pub trait JulianDay {
  
  /*
  * Convert from DateTime Object to a Julian Day as f64
  */
  fn to_jd(&self) -> f64;
  
  /*
  * Convert from a Julian Day as f64 to DateTime Object
  */
  fn from_jd(jd: f64) -> Option<Self> where Self: Sized;

}

///
/// This trait may be implemented by any Date or DateTime object
/// An implementation for chrono::NaiveDateTime is provided below
///
pub trait WeekdayIndex {
  ///
  /// Current weekday index, where Sunday = 0, Monday = 1 and Saturday = 6
  /// The local weekday index depends on the timezone offset in seconds where
  /// The offset_secs parameter is required for timezone-neutral date-time objects
  /// chrono::DateTime has its only 
  /// West of UTC => Negative hour offset * 3600, e.g. -18000 => UTC-5
  /// East of UTC => Positive hour offset * 3600, e.g. +3600 => UTC+1
  ///
  fn weekday_index(&self, offset_secs: i32) -> u8;
}


/*
* This trait may be implemented by any Date or DateTime object
* An implementation for chrono::NaiveDateTime is provided below
*/
pub trait FromFuzzyISOString {
  
  ///
  /// Convert from any ISO-8601-like string (yyyy-mm-dd HH:MM:SS) to a DateTime object
  /// Valid formats
  /// Full date-time: e.g. 2023-11-15T17:53:26
  /// with optional millisecends (ignored): e.g. 2023-11-15T17:53:26.383Z
  /// with space rather than T: 2023-11-15 17:53:26
  /// without seconds: 2023-11-15T17:53 (rounded to the start of the minute)
  /// without minutes: 2023-11-15T17 (rounded to the top of the hour)
  /// without time: 2023-11-15 (rounded to the start of the day)
  /// without the month day: 2023-11 (rounded to the start of the month)
  /// Year only: 2023 (rounded to the year start)
  ///
  fn from_fuzzy_iso_string(dt_str: &str) -> Option<Self>  where Self: Sized;

}

impl JulianDay for NaiveDateTime {

  /// convert datetime object to a Julian day as a 64-bit bit
  /// 
  /// ### Example:
  /// ```
  /// use chrono::NaiveDateTime;
  /// use julian_day_converter::*;
  /// 
  /// if let Ok(date_time) = NaiveDateTime::parse_from_str("2023-11-08 12:53:31", "%Y-%m-%d %H:%M:%S") {
  ///    println!("The astronomical application needs this value {}", date_time.to_jd());
  /// }
  /// ```
  fn to_jd(&self) -> f64 {
    unixtime_to_julian_day(self.timestamp())
  }

  /// construct a DateTime object from a Julian day value (64-bit float)
  /// 
  /// ### Example:
  /// ```
  /// use chrono::NaiveDateTime;
  /// use julian_day_converter::*;
  /// 
  /// let jd: f64 = 2321789.393736365;
  /// if let Some(date_time) = NaiveDateTime::from_jd(jd) {
  ///    println!("The julian day {} translates to {}", jd, date_time.format("%Y-%m-%d %H:%M:%S"));
  /// }
  /// ```
  fn from_jd(jd: f64) -> Option<Self> {
    if let Ok(dt) = julian_day_to_datetime(jd) {
      Some(dt)
    } else {
      None
    }
  }
}

impl FromFuzzyISOString for NaiveDateTime {
  /// construct a DateTime object from an exact or approximate ISO-8601-compatible string
  fn from_fuzzy_iso_string(dt_str: &str) -> Option<Self> {
    iso_fuzzy_string_to_datetime(dt_str)
  }
}

impl WeekdayIndex for NaiveDateTime {
  /// return the weekday index (Sun = 0, Mon = 1 ... Sat = 6) in a timezone-neutral context by adding the offset in seconds
  fn weekday_index(&self, offset_secs: i32) -> u8 {
    julian_day_to_weekday_index(self.to_jd(), offset_secs)
  }
}

///
/// Calculate the weekday index from a given Julian Day with timezone offsets in seconds
///
pub fn julian_day_to_weekday_index(jd: f64, offset_secs: i32) -> u8 {
	let ref_jd = jd + (offset_secs as f64 / 86400f64);
	let days_since_1970 = ref_jd - JULIAN_DAY_UNIX_EPOCH_DAYS as f64;
  let ds = (days_since_1970 as u64) % 7;
  let days_since_index = if ds < 7 { ds as u8 } else { 0u8 };
	(days_since_index + JULIAN_DAY_UNIX_EPOCH_WEEKDAY) % 7
}

///
/// Utility function to convert any ISO-8601-like date string to a Chrono NaiveDateTime object
/// This function accepts YYYY-mm-dd HH:MM:SS separated by a space or letter T and with or without hours, minutes or seconds.
/// Missing time parts will be replaced by 00, hence 2022-06-23 will be 2022-06-23 00:00:00 UTC and 22-06-23 18:20 will be 2022-06-23 18:30:00
/// Missing month and day parts will be replaced by `01`.
/// 
/// ## Example:
/// ```
/// use chrono::NaiveDateTime;
/// use julian_day_converter::*;
/// 
/// if let Some(date_time) = iso_fuzzy_string_to_datetime("2023-11-08 14:17") {
///   println!("The Julian day to the nearest minute is {}", date_time.to_jd());
/// }
/// ```
///
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

}
