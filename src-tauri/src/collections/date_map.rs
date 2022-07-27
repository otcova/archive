use chrono::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
pub struct UtcDate {
    timespan: i64,
}

pub type DayHash = i32;

impl UtcDate {
    pub fn ymdh(year: i32, month: u32, day: u32, hour: u32) -> Self {
        let date = Utc.ymd(year, month, day).and_hms(hour, 0, 0);
        Self {
            timespan: date.timestamp_millis(),
        }
    }

    pub fn date_hash(&self) -> i64 {
        self.timespan
    }
}

#[cfg(test)]
mod test {
    use super::*;

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
