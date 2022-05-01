use crate::database::error::{ErrorKind, Result};
use chrono::prelude::*;

const UTC_DAY_FORMAT: &str = "%Y/%m/%d";
const UTC_INSTANT_FORMAT: &str = "%Y/%m/%d %H:%M:%S%.9f";
const LOCAL_INSTANT_FORMAT: &str = "%Y/%m/%d %H:%M:%S";

#[derive(Debug, PartialEq)]
pub struct Day(String);
#[derive(Debug, PartialEq)]
pub struct Instant(String);

impl Day {
    fn now() -> Self {
        Self(Utc::now().format(UTC_DAY_FORMAT).to_string())
    }
    fn from_utc(utc: &str) -> Result<Self> {
        if let Ok(time) = NaiveDateTime::parse_from_str(utc, UTC_DAY_FORMAT) {
            return Ok(Self(time.format(UTC_DAY_FORMAT).to_string()));
        }
        ErrorKind::DataIsCorrupted.into()
    }
    fn str(&self) -> &str {
        self.0.as_str()
    }
}

impl Instant {
    fn now() -> Self {
        Self(Utc::now().format(UTC_INSTANT_FORMAT).to_string())
    }
    fn from_utc(utc: &str) -> Result<Self> {
        if let Ok(time) = NaiveDateTime::parse_from_str(utc, UTC_INSTANT_FORMAT) {
            return Ok(Self(time.format(UTC_INSTANT_FORMAT).to_string()));
        }
        ErrorKind::DataIsCorrupted.into()
    }

    fn to_local_time(&self) -> String {
        let parsed = DateTime::parse_from_str(
            format!("{} +0000", self.0).as_str(),
            format!("{} %z", UTC_INSTANT_FORMAT).as_str(),
        )
        .unwrap();

        let local_time = Local::now().offset().from_utc_datetime(&parsed.naive_utc());
        local_time.format(LOCAL_INSTANT_FORMAT).to_string()
    }

    fn str(&self) -> &str {
        self.0.as_str()
    }
}

#[cfg(test)]
mod tests {
    use super::{Instant, UTC_INSTANT_FORMAT, UTC_DAY_FORMAT, Day};
    use chrono::prelude::*;

    #[test]
    fn instant_now_can_be_parsed() {
        let now = Instant::now();
        let time = NaiveDateTime::parse_from_str(now.str(), UTC_INSTANT_FORMAT).unwrap();
        assert_eq!(now.str(), time.format(UTC_INSTANT_FORMAT).to_string());
    }

    #[test]
    fn instant_from_random_string_throws_data_corrupted_error() {
        let instant = Instant::from_utc("Hello World!");
        assert_eq!(format!("{:?}", instant), "Err(DataIsCorrupted)");
    }

    #[test]
    fn instant_from_converts_to_local_time() {
        let instant = Instant::from_utc("2022/05/01 15:02:29.542741900").unwrap();
        assert_eq!(instant.to_local_time(), "2022/05/01 17:02:29");
    }
    
    #[test]
    fn instant_now_and_from_dont_mutate() {
        let initial_time = Instant::now();
        let final_time = Instant::from_utc(initial_time.str()).unwrap();
        assert_eq!(initial_time, final_time);
    }
    
    #[test]
    fn day_now_can_be_parsed() {
        let now = Day::now();
        let time = NaiveDateTime::parse_from_str(now.str(), UTC_DAY_FORMAT).unwrap();
        assert_eq!(now.str(), time.format(UTC_DAY_FORMAT).to_string());
    }

    #[test]
    fn day_from_random_string_throws_data_corrupted_error() {
        let day = Day::from_utc("Hello World!");
        assert_eq!(format!("{:?}", day), "Err(DataIsCorrupted)");
    }

    #[test]
    fn day_now_and_from_dont_mutate() {
        let initial_day = Day::now();
        let final_day = Day::from_utc(initial_day.str()).unwrap();
        assert_eq!(initial_day, final_day);
    }
}
