use crate::database::error::{ErrorKind, Result};
use chrono::prelude::*;
use std::cmp::Ordering;

const UTC_INSTANT_FORMAT: &str = "%Y_%m_%d %H_%M_%S";
const LOCAL_INSTANT_FORMAT: &str = "%Y_%m_%d %H_%M_%S";

#[derive(Debug, PartialEq)]
pub struct Instant(DateTime<Utc>);

impl Instant {
    pub fn now() -> Self {
        Self(Utc::now().round_subsecs(0))
    }
    pub fn from_utc(utc: &str) -> Result<Self> {
        if let Ok(time) = NaiveDateTime::parse_from_str(utc, UTC_INSTANT_FORMAT) {
            return Ok(Self(DateTime::<Utc>::from_utc(time, Utc)));
        }
        ErrorKind::DataIsCorrupted.into()
    }

    pub fn to_local_time(&self) -> String {
        let local = DateTime::<Local>::from(self.0);
        local.format(LOCAL_INSTANT_FORMAT).to_string()
    }

    pub fn str(&self) -> String {
        self.0.format(UTC_INSTANT_FORMAT).to_string()
    }
    
    pub fn year(&self) -> i32 {
        self.0.year()
    }
}

impl PartialOrd for Instant {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

#[cfg(test)]
mod tests {
    use super::{Instant, UTC_INSTANT_FORMAT};
    use chrono::prelude::*;

    #[test]
    fn instant_now_can_be_parsed() {
        let now = Instant::now();
        let time = NaiveDateTime::parse_from_str(now.str().as_str(), UTC_INSTANT_FORMAT).unwrap();
        assert_eq!(now.str(), time.format(UTC_INSTANT_FORMAT).to_string());
    }
    
    #[test]
    fn instant_from_invalid_format_throws_data_corrupted_error() {
        let instant1 = Instant::from_utc("2022_00_01 17_02_29");
        assert_eq!(format!("{:?}", instant1), "Err(DataIsCorrupted)");
        
        let instant2 = Instant::from_utc("2022/02/01 17:02:29");
        assert_eq!(format!("{:?}", instant2), "Err(DataIsCorrupted)");
        
        let instant2 = Instant::from_utc("2022_02_00 17_02_29");
        assert_eq!(format!("{:?}", instant2), "Err(DataIsCorrupted)");
    }
    
    #[test]
    fn instant_from_random_string_throws_data_corrupted_error() {
        let instant = Instant::from_utc("Hello World!");
        assert_eq!(format!("{:?}", instant), "Err(DataIsCorrupted)");
    }

    #[test]
    fn instant_from_converts_to_local_time() {
        let instant = Instant::from_utc("2022_05_01 15_02_29").unwrap();
        assert_eq!(instant.to_local_time(), "2022_05_01 17_02_29");
    }
    
    #[test]
    fn instant_now_and_from_dont_mutate() {
        let initial_time = Instant::now();
        let final_time = Instant::from_utc(initial_time.str().as_str()).unwrap();
        println!("{:?}", initial_time);
        println!("{:?}", final_time);
        assert_eq!(initial_time.str(), final_time.str());
    }
    
    #[test]
    fn instant_cmp() {
        let instant1 = Instant::from_utc("2020_05_01 15_02_29").unwrap();
        let instant2 = Instant::from_utc("2022_05_01 15_02_29").unwrap();
        let instant3 = Instant::from_utc("2022_05_01 15_02_29").unwrap();
        let instant4 = Instant::from_utc("2022_05_01 15_02_30").unwrap();
        let instant5 = Instant::from_utc("2022_05_03 15_02_29").unwrap();
        
        assert!(instant1 < instant2);
        assert!(instant2 == instant3);
        assert!(instant3 < instant4);
        assert!(instant4 < instant5);
    }
    
    #[test]
    fn instant_from_parses_year() {
        let instant1 = Instant::from_utc("2022_05_01 15_02_29").unwrap();
        assert_eq!(instant1.year(), 2022);
        
        let instant2 = Instant::from_utc("1921_12_01 23_02_29").unwrap();
        assert_eq!(instant2.year(), 1921);
        
        let instant3 = Instant::from_utc("4021_01_01 00_00_00").unwrap();
        assert_eq!(instant3.year(), 4021);
    }
}
