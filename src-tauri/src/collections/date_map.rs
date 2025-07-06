use chrono::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
pub struct UtcDate {
    timespan: i64,
}

impl UtcDate {
    pub fn ymdh(year: i32, month: u32, day: u32, hour: u32) -> Self {
        let date = Utc.ymd(year, month, day).and_hms(hour, 0, 0);
        Self {
            timespan: date.timestamp_millis(),
        }
    }

    pub fn day_hash(&self) -> i64 {
        const MS_ON_A_DAY: i64 = 1000 * 60 * 60 * 24;
        self.timespan / MS_ON_A_DAY
    }

    pub fn date_hash(&self) -> i64 {
        self.timespan
    }

    pub fn from_hash(hash: i64) -> Self {
        Self { timespan: hash }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ymd_hash_day_difference() {
        let date_a = UtcDate::ymdh(2010, 3, 3, 23).day_hash();
        let date_b = UtcDate::ymdh(2010, 3, 4, 0).day_hash();
        let date_c = UtcDate::ymdh(2010, 3, 4, 23).day_hash();
        let date_d = UtcDate::ymdh(2010, 3, 5, 0).day_hash();
        let date_e = UtcDate::ymdh(2011, 11, 14, 2).day_hash();
        let date_f = UtcDate::ymdh(2056, 2, 3, 13).day_hash();
        let date_g = UtcDate::ymdh(2056, 2, 3, 23).day_hash();

        assert_eq!(date_b - date_a, 1);
        assert_eq!(date_b, date_c);
        assert_eq!(date_d - date_c, 1);
        assert!(date_d < date_e);
        assert!(date_e < date_f);
        assert_eq!(date_f, date_g);
    }

    #[test]
    fn ymd_hash_date_difference() {
        let date_a = UtcDate::ymdh(2010, 3, 4, 0).date_hash();
        let date_b = UtcDate::ymdh(2010, 3, 14, 20).date_hash();
        let date_c = UtcDate::ymdh(2011, 11, 24, 3).date_hash();
        let date_d = UtcDate::ymdh(2056, 2, 3, 23).date_hash();

        assert!(date_a < date_b);
        assert!(date_b < date_c);
        assert!(date_c < date_d);
    }
}
