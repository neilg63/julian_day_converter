use chrono::{DateTime, NaiveDateTime, ParseError};

/// Public constant that may be useful to library users
pub const JULIAN_DAY_UNIX_EPOCH_DAYS: f64 = 2440587.5; // 1970-01-01 00:00:00 UTC

/// The weekday index for the Unix Epoch (1970-01-01 UTC) is Thursday (4)
pub const JULIAN_DAY_UNIX_EPOCH_WEEKDAY: u8 = 4; // 1970-01-01 00:00:00 was a Wednesday UTC

/// Minimum Julian Day value date-time conversion. NB: 0 is -4713-11-24 12:00:00 UTC
/// NB: Some databases may not support dates before -4713
/// -9999-01-01 00:99:00 UTC
pub const JULIAN_DAY_MIN_SUPPORTED: f64 = -1_930_999.5;

/// Maximum Julian Day value for date-time conversion. NB: 0 is -4713-11-24 12:00:00 UTC
/// 9999-12-31 23:59:59 UTC
pub const JULIAN_DAY_MAX_SUPPORTED: f64 = 5_373_484.499999;

///
/// Custom Error Type for date range conversion errors
/// 
#[derive(Debug)]
pub struct DateRangeCoversionError;

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
pub fn julian_day_to_datetime(jd: f64) -> Result<NaiveDateTime, DateRangeCoversionError> {
  if jd >= JULIAN_DAY_MIN_SUPPORTED && jd <= JULIAN_DAY_MAX_SUPPORTED {
    if let Some(dt) = DateTime::from_timestamp(julian_day_to_unixtime(jd), 0) {
      return Ok(dt.naive_utc());
    }
  }
  Err(DateRangeCoversionError)
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
    unixtime_to_julian_day(self.and_utc().timestamp())
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
/// Will be removed in 0.4.0. You can use either chrono::NaiveDateTime::from_str() with standard ISO 6801 formats or parse_from_str() with custom format specifiers.
/// Form more versatile date-time parsing and correction, use the fuzzy-datetime crate, especially for inconsistent data sources
///
#[deprecated(since = "0.3.3", note = "use fuzzy-datetime crate instead")]
pub fn datetime_to_julian_day(dt_str: &str) -> Result<f64, ParseError> {
  match iso_fuzzy_string_to_datetime(dt_str) {
      Ok(dt) => Ok(unixtime_to_julian_day(dt.and_utc().timestamp())),
      Err(error) => Err(error)
  }
}

/// 
/// This trait may be implemented by any Date or DateTime object
/// An implementation for chrono::NaiveDateTime is provided below
/// 
/// It Will be removed in 0.4.0. The functionality has moved to the fuzzy-datetime crate
/// with more advanced date-time parsing and correction options
#[deprecated(since = "0.3.3", note = "use fuzzy-datetime crate instead")]
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

/// Implement the FromFuzzyISOString trait for NaiveDateTime
/// Use the fuzzy-datetime crate for more robust date-time parsing and correction
impl FromFuzzyISOString for NaiveDateTime {
  /// construct a DateTime object from an exact or approximate ISO-8601-compatible string
  fn from_fuzzy_iso_string(dt_str: &str) -> Option<Self> {
    if let Ok(dt) = iso_fuzzy_string_to_datetime(dt_str) {
      Some(dt)
    } else {
      None
    }
  }
}

/// Converts a flexible ISO-8601-like date string into a `chrono::NaiveDateTime`.
///
/// This function is designed to handle various date-time string formats that might
/// be encountered in CSV files or spreadsheets:
/// - Dates can be separated from times by a space or the letter 'T'.
/// - Time components (hours, minutes, seconds) are optional; missing parts are filled with '00'.
/// - Date components (year, month, day) can be missing; they are replaced with '01' where necessary.
/// - Supports formats like:
///   - "YYYY-MM-DD"
///   - "YYYY-MM-DD HH:MM:SS"
///   - "YYYY-MM-DDTHH:MM"
///   - "YY-MM-DD HH:MM"
///
/// # Examples
///
/// ```
/// use chrono::NaiveDateTime;
/// use julian_day_converter::*;
///
/// if let Ok(date_time) = iso_fuzzy_string_to_datetime("2023-11-08 14:17") {
///     println!("The Julian day to the nearest minute is {}", date_time.to_jd());
/// }
/// if let Ok(date_time) = iso_fuzzy_string_to_datetime("22-06-23") {
///     // This would be interpreted as 2022-06-23 00:00:00
///     println!("Converted to: {}", date_time);
/// }
/// ```
///
/// # Behavior
/// - If the year is given with only two digits, it's assumed to be in the 2000s (e.g., "22" becomes "2022").
/// - Missing time parts default to 00:00:00.
/// - Missing date parts default to 01.
///
/// # Errors
/// - Returns a `chrono::ParseError` if the string cannot be parsed into a valid date-time after formatting.
///
#[deprecated(since = "0.3.3", note = "use fuzzy-datetime crate instead")]
pub fn iso_fuzzy_string_to_datetime(dt: &str) -> Result<NaiveDateTime, ParseError> {
  let dt_base = dt.split('.').next().unwrap_or(dt);
  let clean_dt = dt_base.replace("T", " ").trim().to_string();
  let mut dt_parts = clean_dt.split_whitespace();
  let date_part = dt_parts.next().unwrap_or("2000-01-01");
  let time_part = dt_parts.next().unwrap_or("00:00:00");

  let mut date_parts: Vec<i16> = date_part.split('-').filter_map(|s| s.parse::<i16>().ok()).collect();
  while date_parts.len() < 3 {
      date_parts.push(0);
  }
  if date_parts[1] < 1 {
      date_parts[1] = 1;
  }
  if date_parts[2] < 1 {
    date_parts[2] = 1;
  }
  let formatted_date = format!("{:4}-{:02}-{:02}", date_parts[0], date_parts[1], date_parts[2]);

  let mut time_parts: Vec<u8> = time_part.split(':').filter_map(|s| s.parse::<u8>().ok()).collect();
  while time_parts.len() < 3 {
      time_parts.push(0);
  }
  let formatted_time = format!("{:02}:{:02}:{:02}", time_parts[0], time_parts[1], time_parts[2]);

  let formatted_str = format!("{} {}", formatted_date, formatted_time);
  NaiveDateTime::parse_from_str(&formatted_str, "%Y-%m-%d %H:%M:%S")
}

