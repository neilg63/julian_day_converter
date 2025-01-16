use chrono::{DateTime, NaiveDateTime};

/// Public constant that may be useful to library users
pub const JULIAN_DAY_UNIX_EPOCH_DAYS: f64 = 2440587.5; // 1970-01-01 00:00:00 UTC

/// The weekday index for the Unix Epoch (1970-01-01 UTC) is Thursday (4)
pub const JULIAN_DAY_UNIX_EPOCH_WEEKDAY: u8 = 4;

/// Minimum Julian Day value for date-time conversion. Note: 0 is -4713-11-24 12:00:00 UTC
/// Note: Some databases may not support dates before -4713
/// -9999-01-01 00:00:00 UTC
pub const JULIAN_DAY_MIN_SUPPORTED: f64 = -1_930_999.5;

/// Maximum Julian Day value for date-time conversion. Note: 0 is -4713-11-24 12:00:00 UTC
/// 9999-12-31 23:59:59 UTC
pub const JULIAN_DAY_MAX_SUPPORTED: f64 = 5_373_484.499999;

/// Custom Error Type for date range conversion errors
#[derive(Debug)]
pub struct DateRangeConversionError;

/// Convert a Unix timestamp as a 64-bit integer to Julian days as a 64-bit float
/// 
/// ### Example:
/// ```
/// use julian_day_converter::*;
/// 
/// let julian_day: f64 = unixtime_to_julian_day(1672929282);
/// ```
pub fn unixtime_to_julian_day(ts: i64) -> f64 {
  (ts as f64 / 86_400f64) + JULIAN_DAY_UNIX_EPOCH_DAYS
}

/// Convert Julian day as a 64-bit float to Unix timestamp seconds as a signed 64-bit integer
/// 
/// ### Example:
/// ```
/// use julian_day_converter::*;
/// 
/// let julian_day: f64 = 2460258.488768587;
/// let unix_time: i64 = julian_day_to_unixtime(julian_day);
/// ```
pub fn julian_day_to_unixtime(jd: f64) -> i64 {
  ((jd - JULIAN_DAY_UNIX_EPOCH_DAYS) * 86400f64) as i64
}

/// Convert Julian day as a 64-bit float to a timezone-neutral chrono::NaiveDateTime object
/// 
/// ### Example:
/// ```
/// use chrono::NaiveDateTime;
/// use julian_day_converter::*;
/// 
/// let julian_day: f64 = 2460258.488768587;
/// if let Ok(date_time) = julian_day_to_datetime(julian_day) {
///   println!("The date time is {}", date_time.format("%Y-%m-%d %H:%M:%S"));
/// }
/// ```
pub fn julian_day_to_datetime(jd: f64) -> Result<NaiveDateTime, DateRangeConversionError> {
  if jd >= JULIAN_DAY_MIN_SUPPORTED && jd <= JULIAN_DAY_MAX_SUPPORTED {
    if let Some(dt) = DateTime::from_timestamp(julian_day_to_unixtime(jd), 0) {
      return Ok(dt.naive_utc());
    }
  }
  Err(DateRangeConversionError)
}

/// This trait may be implemented by any Date or DateTime object
/// An implementation for chrono::NaiveDateTime is provided below
pub trait JulianDay {
  
  /// Convert from DateTime Object to a Julian Day as f64
  fn to_jd(&self) -> f64;
  
  /// Convert from a Julian Day as f64 to DateTime Object
  fn from_jd(jd: f64) -> Option<Self> where Self: Sized;
}

impl JulianDay for NaiveDateTime {

  /// Convert datetime object to a Julian day as a 64-bit float
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

  /// Construct a DateTime object from a Julian day value (64-bit float)
  /// 
  /// ### Example:
  /// ```
  /// use chrono::NaiveDateTime;
  /// use julian_day_converter::*;
  /// 
  /// let jd: f64 = 2321789.393736365;
  /// if let Some(date_time) = NaiveDateTime::from_jd(jd) {
  ///    println!("The Julian day {} translates to {}", jd, date_time.format("%Y-%m-%d %H:%M:%S"));
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


/// This trait may be implemented by any Date or DateTime object
/// An implementation for chrono::NaiveDateTime is provided below
pub trait WeekdayIndex {
  /// Current weekday index, where Sunday = 0, Monday = 1, and Saturday = 6
  /// The local weekday index depends on the timezone offset in seconds where
  /// The offset_secs parameter is required for timezone-neutral date-time objects
  /// This matches chrono::DateTime has its own %w specifier for weekday index
  /// with the correct timezone offset
  /// West of UTC => Negative hour offset * 3600, e.g. -18000 => UTC-5
  /// East of UTC => Positive hour offset * 3600, e.g. +3600 => UTC+1
  fn weekday_index(&self, offset_secs: i32) -> u8;

  /// ISO 8601 and Java/C# style day of week index starting from Monday = 1 to Sunday = 7
  /// NB: Python's datetime.weekday() method returns 0 for Monday and 6 for Sunday
  fn weekday_number(&self, offset_secs: i32) -> u8;
}

impl WeekdayIndex for NaiveDateTime {
  /// Return the weekday index (Sun = 0, Mon = 1 ... Sat = 6) in a timezone-neutral context by adding the offset in seconds
  fn weekday_index(&self, offset_secs: i32) -> u8 {
    julian_day_to_weekday_index(self.to_jd(), offset_secs)
  }

  /// Return the weekday index (Mon = 0, Tue = 1 ... Sun = 6) in a timezone-neutral context by adding the offset in seconds
  /// as used in Java, C# and ISO 8601 (Python's datetime.weekday() method is 0-based from Monday)
  fn weekday_number(&self, offset_secs: i32) -> u8 {
    julian_day_to_weekday_number(self.to_jd(), offset_secs)
  }
}

/// Calculate the weekday index from a given Julian Day with timezone offsets in seconds
/// NB: This is zero-based, where Sunday = 0, Monday = 1, ..., Saturday = 6
/// Some languages use a different weekday index, e.g. Python, Java, and C#
pub fn julian_day_to_weekday_index(jd: f64, offset_secs: i32) -> u8 {
  let ref_jd = jd + (offset_secs as f64 / 86400f64);
  let days_since_1970 = ref_jd - JULIAN_DAY_UNIX_EPOCH_DAYS as f64;
  let ds = (days_since_1970 as u64) % 7;
  let days_since_index = if ds < 7 { ds as u8 } else { 0u8 };
  (days_since_index + JULIAN_DAY_UNIX_EPOCH_WEEKDAY) % 7
}

/// Return the weekday index (Mon = 0, Tue = 1 ... Sun = 6) in a timezone-neutral context by adding the offset in seconds
/// as used in Java, C# and ISO 8601 (Python's datetime.weekday() method is 0-based from Monday)
pub fn julian_day_to_weekday_number(jd: f64, offset_secs: i32) -> u8 {
  let index = julian_day_to_weekday_index(jd, offset_secs);
  if index == 0 {
    7
  } else {
    index
  }
}
