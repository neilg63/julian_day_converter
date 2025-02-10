use chrono::{DateTime, NaiveDateTime};
use std::fmt;

/// Public constant that may be useful to library users
/// 1970-01-01 00:00:00 UTC
pub const JULIAN_DAY_UNIX_EPOCH_DAYS: f64 = 2440587.5; 

/// The weekday index for the Unix Epoch (1970-01-01 UTC) is Thursday (4)
pub const JULIAN_DAY_UNIX_EPOCH_WEEKDAY: u8 = 4;

/// Minimum Julian Day value for date-time conversion.
/// Note: 0 is -4713-11-24 12:00:00 UTC
/// Note: Some databases may not support dates before -4713
/// -9999-01-01 00:00:00 UTC
pub const JULIAN_DAY_MIN_SUPPORTED: f64 = -1_930_999.5;

/// Maximum Julian Day value for date-time conversion.
/// Note: 0 is -4713-11-24 12:00:00 UTC
/// 9999-12-31 23:59:59 UTC
pub const JULIAN_DAY_MAX_SUPPORTED: f64 = 5_373_484.499999;

/// Custom Error Type for date range conversion errors
#[derive(Debug)]
pub struct DateRangeConversionError;

impl fmt::Display for DateRangeConversionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "DateRangeConversionError: The provided Julian Day is out of the supported range.")
    }
}

/// Convert a Unix timestamp milliseconds as a 64-bit integer to Julian days as a 64-bit float
///
/// ## Example:
/// ```
/// use julian_day_converter::*;
///
/// let julian_day: f64 = unix_millis_to_julian_day(1_672_929_282_000);
/// ```
pub fn unix_millis_to_julian_day(ms: i64) -> f64 {
    (ms as f64 / 86_400_000.0) + JULIAN_DAY_UNIX_EPOCH_DAYS
}


/// Convert a Unix timestamp seconds as a 64-bit integer to Julian days as a 64-bit float
///
/// ## Example:
/// ```
/// use julian_day_converter::*;
///
/// let julian_day: f64 = unixtime_to_julian_day(1_672_929_282);
/// ```
pub fn unixtime_to_julian_day(ms: i64) -> f64 {
  (ms as f64 / 86_400.0) + JULIAN_DAY_UNIX_EPOCH_DAYS
}

/// Convert Julian day as a 64-bit float to Unix timestamp milliseconds as a signed 64-bit integer
///
/// ## Example:
/// ```
/// use julian_day_converter::*;
///
/// let julian_day: f64 = 2_460_258.488768587;
/// let unix_millis: i64 = julian_day_to_unix_millis(julian_day);
/// ```
pub fn julian_day_to_unix_millis(jd: f64) -> i64 {
    ((jd - JULIAN_DAY_UNIX_EPOCH_DAYS) * 86_400_000.0) as i64
}

/// Convert Julian day as a 64-bit float to Unix timestamp seconds as a signed 64-bit integer
///
/// ## Example:
/// ```
/// use julian_day_converter::*;
///
/// let julian_day: f64 = 2_460_258.488768587;
/// let unix_seconds: i64 = julian_day_to_unixtime(julian_day);
/// ```
pub fn julian_day_to_unixtime(jd: f64) -> i64 {
  ((jd - JULIAN_DAY_UNIX_EPOCH_DAYS) * 86_400.0) as i64
}

/// Convert Julian day as a 64-bit float to a timezone-neutral chrono::NaiveDateTime object
///
/// ## Example:
/// ```
/// use chrono::NaiveDateTime;
/// use julian_day_converter::*;
///
/// let julian_day: f64 = 2460258.488768587;
/// if let Ok(date_time) = julian_day_to_datetime(julian_day) {
///     println!("The date time is {}", date_time.format("%Y-%m-%d %H:%M:%S"));
/// }
/// ```
pub fn julian_day_to_datetime(jd: f64) -> Result<NaiveDateTime, DateRangeConversionError> {
    if jd >= JULIAN_DAY_MIN_SUPPORTED && jd <= JULIAN_DAY_MAX_SUPPORTED {
        let milliseconds = julian_day_to_unix_millis(jd);
        if let Some(dt) = DateTime::from_timestamp_millis(milliseconds) {
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
    /// ## Example:
    /// ```
    /// use chrono::NaiveDateTime;
    /// use julian_day_converter::*;
    ///
    /// if let Ok(date_time) = NaiveDateTime::parse_from_str("2023-11-08 12:53:31", "%Y-%m-%d %H:%M:%S") {
    ///     println!("The astronomical application needs this value {}", date_time.to_jd());
    /// }
    /// ```
    fn to_jd(&self) -> f64 {
        unix_millis_to_julian_day(self.and_utc().timestamp_millis())
    }

    /// Construct a DateTime object from a Julian day value (64-bit float)
    ///
    /// ## Example:
    /// ```
    /// use chrono::NaiveDateTime;
    /// use julian_day_converter::*;
    ///
    /// let jd: f64 = 2321789.393736365;
    /// if let Some(date_time) = NaiveDateTime::from_jd(jd) {
    ///     println!("The Julian day {} translates to {}", jd, date_time.format("%Y-%m-%d %H:%M:%S"));
    /// }
    /// ```
    fn from_jd(jd: f64) -> Option<Self> {
        julian_day_to_datetime(jd).ok()
    }
}

/// This trait may be implemented by any Date or DateTime object
/// An implementation for chrono::NaiveDateTime is provided below
pub trait WeekdayIndex {
    /// Current weekday index, where Sunday = 0, Monday = 1, and Saturday = 6
    /// The local weekday index depends on the timezone offset in seconds
    /// West of UTC => Negative hour offset * 3600, e.g. -18000 => UTC-5
    /// East of UTC => Positive hour offset * 3600, e.g. +3600 => UTC+1
    fn weekday_index(&self, offset_secs: i32) -> u8;

    /// ISO 8601 and Java/C# style day of week index starting from Monday = 1 to Sunday = 7
    /// NB: Python's datetime.weekday() method returns 0 for Monday and 6 for Sunday
    fn weekday_number(&self, offset_secs: i32) -> u8;
}

impl WeekdayIndex for NaiveDateTime {
    /// Return the weekday index (Sun = 0, Mon = 1 ... Sat = 6) in a timezone-neutral context
    fn weekday_index(&self, offset_secs: i32) -> u8 {
        julian_day_to_weekday_index(self.to_jd(), offset_secs)
    }

    /// Return the weekday index (Mon = 1, Tue = 2 ... Sun = 7) in a timezone-neutral context
    fn weekday_number(&self, offset_secs: i32) -> u8 {
        julian_day_to_weekday_number(self.to_jd(), offset_secs)
    }
}

/// Calculate the weekday index from a given Julian Day with timezone offsets in seconds
/// This is zero-based, where Sunday = 0, Monday = 1, ..., Saturday = 6
pub fn julian_day_to_weekday_index(jd: f64, offset_secs: i32) -> u8 {
    let ref_jd = jd + (offset_secs as f64 / 86400.0);
    let days_since_1970 = ref_jd - JULIAN_DAY_UNIX_EPOCH_DAYS;
    let ds = (days_since_1970 as u64) % 7;
    let days_since_index = if ds < 7 { ds as u8 } else { 0u8 };
    (days_since_index + JULIAN_DAY_UNIX_EPOCH_WEEKDAY) % 7
}

/// Return the weekday number (Mon = 1 to Sun = 7) in a timezone-neutral context
/// as used in Java, C# and ISO 8601 (Python's datetime.weekday() method is 0-based from Monday)
pub fn julian_day_to_weekday_number(jd: f64, offset_secs: i32) -> u8 {
    let index = julian_day_to_weekday_index(jd, offset_secs);
    if index == 0 { 7 } else { index }
}