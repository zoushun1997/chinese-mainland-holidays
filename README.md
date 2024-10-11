# Chinese Mainland Holidays

Experiment with [chinese_holiday](https://github.com/leoppro/chinese_holiday_rs).

Determines whether a date is a holiday in Chinese Mainland.

```rust
use chinese_mainland_holidays::HolidayLike;
use chrono::NaiveDate;

assert!(NaiveDate::from_ymd_opt(2024, 10, 7).unwrap().is_holiday().unwrap());
```
