use chrono::prelude::*;

pub struct DateMap<T> {
    first_day_hash: DayHash,
    days: Vec<Vec<(u8, T)>>,
}

pub struct DateMapIter<'a, T> {
    datemap: &'a DateMap<T>,
    index: (usize, usize),
}

impl<T> DateMap<T> {
    fn new() -> Self {
        Self {
            first_day_hash: 0,
            days: vec![],
        }
    }
    fn iter(&self) -> DateMapIter<'_, T> {
        DateMapIter {
            datemap: self,
            index: (0, 0),
        }
    }
}

impl<'a, T> Iterator for DateMapIter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        None
    }
}


/// Stores year, month, day and hour in utc
#[derive(Debug, PartialEq, Eq)]
pub struct UtcDate {
    pub year: i16,
    pub month: u8,
    pub day: u8,
    pub hour: u8,
}
/// Stores year, month, day and hour in local time
pub struct LocalDate {
    pub year: i16,
    pub month: u8,
    pub day: u8,
    pub hour: u8,
}

type DayHash = i32;

impl UtcDate {
    pub fn utc_ymdh(year: i16, month: u8, day: u8, hour: u8) -> Self {
        Self {
            year,
            month,
            day,
            hour,
        }
    }
    pub fn to_local_date(&self) -> LocalDate {
        let date = NaiveDate::from_ymd(self.year as i32, self.month as u32, self.day as u32);
        let time = NaiveTime::from_hms(self.hour as u32, 0, 0);
        let datetime = NaiveDateTime::new(date, time);

        let local = Local.from_utc_datetime(&datetime);

        LocalDate {
            year: local.year() as i16,
            month: local.month() as u8,
            day: local.day() as u8,
            hour: local.hour() as u8,
        }
    }

    /// (JDN Formula)[http://www.cs.utsa.edu/~cs1063/projects/Spring2011/Project1/jdn-explanation.html]
    /// with 2000/1/1 being 0
    pub fn day_hash(&self) -> DayHash {
        let a = (14 - self.month as i32) / 12;
        let y = self.year as i32 + 4800 - a;
        let m = self.month as i32 + 12 * a - 3;
        let d = self.day as i32;
        d + (153 * m + 2) / 5 + 365 * y + y / 4 - y / 100 + y / 400 - (32045 + 2451545)
    }

    /// inverse JDN Formula from [wikipedia](https://wikipedia.org/wiki/Julian_day)
    pub fn from_day_hash(hash: DayHash, hour: u8) -> Self {
        #![allow(non_snake_case)]
        let J = hash + 2451545;

        let y = 4716;
        let j = 1401;
        let m = 2;
        let n = 12;
        let r = 4;
        let p = 1461;
        let v = 3;
        let u = 5;
        let s = 153;
        let w = 2;
        let B = 274277;
        let C = -38;

        let f = J + j + (((4 * J + B) / 146097) * 3) / 4 + C;
        let e = r * f + v;
        let g = (e % p) / r;
        let h = u * g + w;
        let D = (h % s) / u + 1;
        let M = ((h / s + m) % n) + 1;
        let Y = (e / p) - y + (n + m - M) / n;

        Self {
            year: Y as i16,
            month: M as u8,
            day: D as u8,
            hour: hour as u8,
        }
    }
}

impl LocalDate {
    pub fn to_utc_date(&self) -> UtcDate {
        let date = NaiveDate::from_ymd(self.year as i32, self.month as u32, self.day as u32);
        let time = NaiveTime::from_hms(self.hour as u32, 0, 0);
        let datetime = NaiveDateTime::new(date, time);

        let dt_with_tz = Local.from_local_datetime(&datetime).unwrap();
        let utc = dt_with_tz.naive_utc();

        UtcDate {
            year: utc.year() as i16,
            month: utc.month() as u8,
            day: utc.day() as u8,
            hour: utc.hour() as u8,
        }
    }
}

#[cfg(test)]
mod test {
    use super::{DateMap, UtcDate};

    #[test]
    fn utc_date_to_local_to_utc() {
        let utc = UtcDate::utc_ymdh(2010, 11, 2, 8);
        assert_eq!(utc, utc.to_local_date().to_utc_date());
    }

    #[test]
    fn utc_from_hash() {
        let date = UtcDate::utc_ymdh(2000, 1, 1, 0);
        assert_eq!(UtcDate::from_day_hash(date.day_hash(), 0), date);

        let date = UtcDate::utc_ymdh(2000, 1, 1, 18);
        assert_eq!(UtcDate::from_day_hash(date.day_hash(), 18), date);

        let date = UtcDate::utc_ymdh(2011, 11, 14, 22);
        assert_eq!(UtcDate::from_day_hash(date.day_hash(), 22), date);
    }

    #[test]
    fn ymd_hash_is_zero_on_2000() {
        assert_eq!(UtcDate::utc_ymdh(2000, 1, 1, 0).day_hash(), 0);
        assert_eq!(UtcDate::utc_ymdh(2000, 1, 1, 3).day_hash(), 0);
        assert_eq!(UtcDate::utc_ymdh(2000, 1, 1, 16).day_hash(), 0);
    }
    #[test]
    fn ymd_hash_day_difference() {
        let date_a = UtcDate::utc_ymdh(2010, 3, 4, 0).day_hash();
        let date_b = UtcDate::utc_ymdh(2010, 3, 14, 20).day_hash();
        let date_c = UtcDate::utc_ymdh(2011, 11, 24, 3).day_hash();
        let date_d = UtcDate::utc_ymdh(2056, 2, 3, 23).day_hash();

        assert_eq!(10 + date_a, date_b);
        assert_eq!(630 + date_a, date_c);
        assert_eq!(16772 + date_a, date_d);
        assert_eq!(620 + date_b, date_c);
        assert_eq!(16762 + date_b, date_d);
        assert_eq!(16142 + date_c, date_d);
    }

    #[test]
    fn iter_empty_datemap() {
        let datemap = DateMap::<f32>::new();
        let mut datemap_iter = datemap.iter();
        let item = datemap_iter.next();
        assert!(item.is_none());
    }
}
