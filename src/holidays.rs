use crate::{
    HolidayDate,
    HolidayKind::{self, *},
};

/// Minimum year of which holidays are recorded.
pub const MIN_YEAR: u16 = 2024;
/// Maximum year of which holidays are recorded.
pub const MAX_YEAR: u16 = 2024;

macro_rules! record {
    ($y:literal $m:literal $d:literal $kind:ident) => {
        (
            HolidayDate {
                year: $y,
                month: $m,
                day: $d,
            }
            .u32_value(),
            $kind,
        )
    };
}

pub(crate) const HOLIDAYS: [(u32, HolidayKind); 27] = [
    // https://www.gov.cn/zhengce/zhengceku/202310/content_6911528.htm
    record!(2024 1 1 G0101Holiday),
    record!(2024 2 4 L0101Workday),
    record!(2024 2 12 L0101Holiday),
    record!(2024 2 13 L0101Holiday),
    record!(2024 2 14 L0101Holiday),
    record!(2024 2 15 L0101Holiday),
    record!(2024 2 16 L0101Holiday),
    record!(2024 2 18 L0101Workday),
    record!(2024 4 4 S05Holiday),
    record!(2024 4 5 S05Holiday),
    record!(2024 4 7 S05Workday),
    record!(2024 4 28 G0501Workday),
    record!(2024 5 1 G0501Holiday),
    record!(2024 5 2 G0501Holiday),
    record!(2024 5 3 G0501Holiday),
    record!(2024 5 11 G0501Workday),
    record!(2024 6 10 L0505Holiday),
    record!(2024 9 14 L0815Workday),
    record!(2024 9 16 L0815Holiday),
    record!(2024 9 17 L0815Holiday),
    record!(2024 9 29 G1001Workday),
    record!(2024 10 1 G1001Holiday),
    record!(2024 10 2 G1001Holiday),
    record!(2024 10 3 G1001Holiday),
    record!(2024 10 4 G1001Holiday),
    record!(2024 10 7 G1001Holiday),
    record!(2024 10 12 G1001Workday),
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_holidays() {
        assert!(MIN_YEAR > 0 && MIN_YEAR <= MAX_YEAR);
        assert!(!HOLIDAYS.is_empty());
        let first_value = HOLIDAYS.first().unwrap().0;
        assert!(MIN_YEAR as u32 * 366 < first_value && first_value < (MIN_YEAR + 1) as u32 * 366);
        let last_value = HOLIDAYS.last().unwrap().0;
        assert!(MAX_YEAR as u32 * 366 < last_value && last_value < (MAX_YEAR + 1) as u32 * 366);
        let mut prev = 0;
        for (v, _) in HOLIDAYS {
            assert!(prev < v);
            prev = v;
        }
    }
}
