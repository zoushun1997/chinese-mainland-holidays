//! # Chinese Mainland Holidays
//!
//! Determines whether a date is a holiday in Chinese Mainland.
//!
//! ```
//! use chinese_mainland_holidays::HolidayLike;
//! use chrono::NaiveDate;
//!
//! assert!(NaiveDate::from_ymd_opt(2024, 10, 7).unwrap().is_holiday().unwrap());
//! ```
//!
//! ## Features
//!
//! Default features:
//!
//! - `chrono`: Implements [`HolidayLike`] for `chrono` date and datetime types.

#[cfg(feature = "chrono")]
mod chrono;
mod holidays;

use holidays::HOLIDAYS;
pub use holidays::{MAX_YEAR, MIN_YEAR};

/// The type of a holiday or working day.
///
/// Each `*Holiday` is a weekday but holiday.
/// Each `*Workday` is Saturday or Sunday but a working day.
///
/// This enum is marked non_exhaustive to accomodate newly established holidays.
#[derive(Clone)]
#[non_exhaustive]
pub enum HolidayKind {
    /// A regular Saturday or Sunday.
    RegularHoliday,
    /// A regular weekday.
    RegularWorkday,
    /// A holiday on a weekday for New Year.
    G0101Holiday,
    /// An adjusted working day for New Year.
    G0101Workday,
    /// A holiday on a weekday for Chinese New Year.
    L0101Holiday,
    /// An adjusted working day for Chinese New Year.
    L0101Workday,
    /// A holiday on a weekday for Qingming Festival.
    S05Holiday,
    /// An adjusted working day for Qingming Festival.
    S05Workday,
    /// A holiday on a weekday for May Day.
    G0501Holiday,
    /// An adjusted working day for May Day.
    G0501Workday,
    /// A holiday on a weekday for Dragon Boat Festival.
    L0505Holiday,
    /// An adjusted working day for Dragon Boat Festival.
    L0505Workday,
    /// A holiday on a weekday for Mid-Autumn Festival.
    L0815Holiday,
    /// An adjusted working day for Mid-Autumn Festival.
    L0815Workday,
    /// A holiday on a weekday for National Day.
    G1001Holiday,
    /// An adjusted working day for National Day.
    G1001Workday,
}

/// Utility type for looking up holiday info.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct HolidayDate {
    year: u16,
    month: u8,
    day: u8,
}

impl HolidayDate {
    /// Constructs from year, month and day.
    ///
    /// # Errors
    ///
    /// Returns `None` when the given date is invalid or the year is less than 1.
    pub fn from_ymd(year: u16, month: u8, day: u8) -> Option<Self> {
        if year == 0 {
            return None;
        }
        if day == 0 {
            return None;
        }
        match month {
            1 | 3 | 5 | 7 | 8 | 10 | 12 => {
                if day > 31 {
                    return None;
                }
            }
            4 | 6 | 9 | 11 => {
                if day > 30 {
                    return None;
                }
            }
            2 => {
                if day > 28 && !(day == 29 && (year % 4 == 0 && year % 100 != 0 || year % 400 == 0))
                {
                    return None;
                }
            }
            _ => {
                return None;
            }
        }
        Some(Self { year, month, day })
    }

    #[inline]
    const fn u32_value(&self) -> u32 {
        (self.year as u32 * 366) + (self.month as u32 * 31) + self.day as u32
    }
}

/// Returns day of week represented by 0-6, where Sunday is 0.
///
/// The formula is called Zeller's Congruence, adapted from <https://datatracker.ietf.org/doc/html/rfc3339#appendix-B>.
fn day_of_week(year: u16, month: u8, day: u8) -> u8 {
    let m: u8;
    let mut y: u16;
    if month > 2 {
        m = month - 2;
        y = year;
    } else {
        m = month + 10;
        y = year - 1;
    }
    let c = y / 100;
    y %= 100;
    // assert!(13 * m < u8::MAX);
    ((((13 * m - 1) / 5 + day) as u16 + y + y / 4 + c / 4 + 5 * c) % 7) as u8
}

/// Methods for determining whether a date is a holiday.
pub trait HolidayLike {
    /// Returns the holiday kind of the date.
    ///
    /// # Errors
    ///
    /// Returns `None` when the year is less than [`MIN_YEAR`] or greater than [`MAX_YEAR`].
    fn holiday_kind(&self) -> Option<HolidayKind>;

    /// Returns whether the date is a holiday.
    ///
    /// # Errors
    ///
    /// Returns `None` when the year is less than [`MIN_YEAR`] or greater than [`MAX_YEAR`].
    fn is_holiday(&self) -> Option<bool> {
        use HolidayKind::*;
        match self.holiday_kind()? {
            RegularHoliday | G0101Holiday | L0101Holiday | S05Holiday | G0501Holiday
            | L0505Holiday | L0815Holiday | G1001Holiday => Some(true),
            RegularWorkday | G0101Workday | L0101Workday | S05Workday | G0501Workday
            | L0505Workday | L0815Workday | G1001Workday => Some(false),
        }
    }
}

impl HolidayLike for HolidayDate {
    fn holiday_kind(&self) -> Option<HolidayKind> {
        if self.year < MIN_YEAR || self.year > MAX_YEAR {
            None
        } else {
            match HOLIDAYS.binary_search_by_key(&self.u32_value(), |(v, _)| *v) {
                Ok(i) => Some(HOLIDAYS[i].1.clone()),
                Err(_) => match day_of_week(self.year, self.month, self.day) {
                    0 | 6 => Some(HolidayKind::RegularHoliday),
                    1..=5 => Some(HolidayKind::RegularWorkday),
                    _ => unreachable!(),
                },
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_of_week() {
        assert_eq!(day_of_week(2024, 2, 29), 4);
        assert_eq!(day_of_week(2024, 10, 1), 2);
    }

    #[test]
    fn test_holiday_kind() {
        assert!(HolidayDate {
            year: 2023,
            month: 10,
            day: 1
        }
        .holiday_kind()
        .is_none());
        assert!(matches!(
            HolidayDate {
                year: 2024,
                month: 10,
                day: 12
            }
            .holiday_kind()
            .unwrap(),
            HolidayKind::G1001Workday
        ));
    }
}
