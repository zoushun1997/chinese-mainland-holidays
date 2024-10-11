use chrono::prelude::*;

use crate::{HolidayDate, HolidayKind, HolidayLike};

impl From<NaiveDate> for HolidayDate {
    #[inline]
    fn from(value: NaiveDate) -> Self {
        Self {
            year: value.year() as u16,
            month: value.month() as u8,
            day: value.day() as u8,
        }
    }
}

impl From<NaiveDateTime> for HolidayDate {
    #[inline]
    fn from(value: NaiveDateTime) -> Self {
        value.date().into()
    }
}

impl<Tz: TimeZone> From<DateTime<Tz>> for HolidayDate {
    #[inline]
    fn from(value: DateTime<Tz>) -> Self {
        let tz = FixedOffset::east_opt(28800).unwrap();
        value.with_timezone(&tz).date_naive().into()
    }
}

macro_rules! impl_meth_holiday_kind {
    () => {
        /// An inefficient implementation.
        #[inline]
        fn holiday_kind(&self) -> Option<HolidayKind> {
            HolidayDate::from(self.clone()).holiday_kind()
        }
    };
}

impl HolidayLike for NaiveDate {
    impl_meth_holiday_kind!();
}

impl HolidayLike for NaiveDateTime {
    impl_meth_holiday_kind!();
}

impl<Tz: TimeZone> HolidayLike for DateTime<Tz> {
    impl_meth_holiday_kind!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_datetimetz_to_holiday() {
        let date: HolidayDate = Utc.with_ymd_and_hms(2024, 10, 1, 20, 0, 0).unwrap().into();
        assert_eq!(date, HolidayDate::from_ymd(2024, 10, 2).unwrap());
    }
}
