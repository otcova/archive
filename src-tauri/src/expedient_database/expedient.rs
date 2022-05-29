// #![allow(unused_imports)]
use chrono::prelude::*;

#[derive(Debug, PartialEq, Eq)]
pub enum OrderState {
    Done,
    Todo,
    Urgent,
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

#[derive(Debug)]
pub struct User {
    name: String,
    emails: Vec<String>,
    phones: Vec<String>,
}

#[derive(Debug)]
pub struct Order {
    dates: Vec<UtcDate>,
    description: String,
    state: OrderState,
}

#[derive(Debug)]
pub struct Expedient {
    users: Vec<User>,
    model: String,
    license_plate: String,
    vin: String,
    description: String,
    orders: Vec<Order>,
}

impl Expedient {
    fn vin_is_complete(&self) -> bool {
        self.vin.len() >= 17
    }
    fn license_is_complete(&self) -> bool {
        self.license_plate.len() >= 7
    }
}

impl UtcDate {
    fn utc_ymdh(year: i16, month: u8, day: u8, hour: u8) -> Self {
        Self {
            year,
            month,
            day,
            hour,
        }
    }
    fn from_local_time(year: i16, month: u8, day: u8, hour: u8) -> Self {
        let date = NaiveDate::from_ymd(year as i32, month as u32, day as u32);
        let time = NaiveTime::from_hms(hour as u32, 0, 0);
        let datetime = NaiveDateTime::new(date, time);

        let dt_with_tz = Local.from_local_datetime(&datetime).unwrap();
        let utc = dt_with_tz.naive_utc();

        Self {
            year: utc.year() as i16,
            month: utc.month() as u8,
            day: utc.day() as u8,
            hour: utc.hour() as u8,
        }
    }
    fn to_local_time(&self, year: i16, month: u8, day: u8, hour: u8) -> LocalDate {
        let date = NaiveDate::from_ymd(year as i32, month as u32, day as u32);
        let time = NaiveTime::from_hms(hour as u32, 0, 0);
        let datetime = NaiveDateTime::new(date, time);

        let local = Local.from_utc_datetime(&datetime);

        LocalDate {
            year: local.year() as i16,
            month: local.month() as u8,
            day: local.day() as u8,
            hour: local.hour() as u8,
        }
    }
}

// Match -----------------

#[derive(Debug, PartialEq, Clone, Copy)]
enum MatchType {
    Inclusive,
    Similar(f32),
}

impl MatchType {
    fn max(&self, other: &Self) -> MatchType {
        match self {
            Self::Inclusive => MatchType::Inclusive,
            Self::Similar(self_match) => match other {
                Self::Inclusive => MatchType::Inclusive,
                Self::Similar(other_match) => MatchType::Similar(self_match.max(*other_match)),
            },
        }
    }
    fn downgrade_inclusive(&self) -> MatchType {
        match self {
            Self::Inclusive => MatchType::Similar(1.0),
            Self::Similar(_) => *self,
        }
    }
    fn is_inclusive(&self) -> bool {
        match self {
            Self::Inclusive => true,
            _ => false,
        }
    }
    fn magnitude(&self) -> f32 {
        match self {
            Self::Inclusive => 1.0,
            Self::Similar(v) => *v,
        }
    }
}

trait Filter {
    fn filter(&self, filter: &Self) -> MatchType;
}

impl Filter for String {
    fn filter(&self, filter: &Self) -> MatchType {
        let filter_lowercase = filter.to_lowercase();
        let self_lowercase = self.to_lowercase();

        if self_lowercase.contains(&filter_lowercase) {
            return MatchType::Inclusive;
        }

        let mut keywords_match_count = 0;
        let score = filter_lowercase
            .split_whitespace()
            .fold(0f32, |score, keyword| {
                keywords_match_count += 1;
                if self_lowercase.contains(keyword) {
                    return score + 1.;
                }
                score
            })
            / keywords_match_count as f32;

        MatchType::Similar(score)
    }
}

impl Filter for Expedient {
    fn filter(&self, filter: &Self) -> MatchType {
        // Filter by Vin and License
        let vin_match = self.vin.filter(&filter.vin);
        let license_match = self.license_plate.filter(&filter.license_plate);
        println!(
            "{:?}; {:?}; {:?}; {:?}",
            self, filter, vin_match, license_match
        );

        if (filter.vin_is_complete() && vin_match.is_inclusive())
            || (filter.license_is_complete() && license_match.is_inclusive())
        {
            return MatchType::Inclusive;
        }

        if !vin_match.is_inclusive() || !license_match.is_inclusive() {
            return MatchType::Similar(0.0);
        }

        // Filter by user, model, description, orders
        MatchType::Similar(
            (self.users.filter(&filter.users).magnitude()
                + self.model.filter(&filter.model).magnitude()
                + self.description.filter(&filter.description).magnitude()
                + self.orders.filter(&filter.orders).magnitude())
                / 4.0,
        )
    }
}

impl<T: Filter + core::fmt::Debug> Filter for Vec<T> {
    fn filter(&self, filter: &Self) -> MatchType {
        println!("{:?}", filter);
        if filter.len() == 0 {
            return MatchType::Inclusive;
        }
        filter
            .iter()
            .fold(MatchType::Similar(0.0), |best_match, filter_str| {
                self.iter().fold(best_match, |best_match, str| {
                    best_match.max(&str.filter(filter_str))
                })
            })
    }
}

impl Filter for User {
    fn filter(&self, filter: &Self) -> MatchType {
        let mut best_match = MatchType::Similar(0.0);
        if filter.phones.len() > 0 {
            best_match = best_match.max(&self.phones.filter(&filter.phones));
        }
        if filter.emails.len() > 0 {
            best_match = best_match.max(&self.emails.filter(&filter.emails));
        }
        best_match.max(&self.name.filter(&filter.name).downgrade_inclusive())
    }
}

impl Filter for Order {
    fn filter(&self, filter: &Self) -> MatchType {
        let description_match = self.description.filter(&filter.description);
        if let MatchType::Similar(magnitude) = description_match {
            if self.state != filter.state {
                return MatchType::Similar(magnitude / 2.);
            }
        }
        description_match
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn filter_string() {
        let text = "Hello to all RATIONAL creature!!";
        let s = String::from(text);

        let str_match = s.filter(&String::from(text));
        assert_eq!(format!("{:?}", str_match), "Inclusive");

        let str_match = s.filter(&String::from(text).to_uppercase());
        assert_eq!(format!("{:?}", str_match), "Inclusive");

        let str_match = s.filter(&String::from("Hello"));
        assert_eq!(format!("{:?}", str_match), "Inclusive");

        let str_match = s.filter(&String::from("rational hello"));
        assert_eq!(format!("{:?}", str_match), "Similar(1.0)");

        let str_match = s.filter(&String::from("helloall"));
        assert_eq!(format!("{:?}", str_match), "Similar(0.0)");
    }

    #[test]
    fn filter_user() {
        let juan = User {
            name: String::from("Juan Antonio Mario"),
            emails: vec![],
            phones: vec![String::from("932123456")],
        };
        let juan_name = User {
            name: String::from("Juan"),
            emails: vec![],
            phones: vec![],
        };
        let juan_phone = User {
            name: String::from(""),
            emails: vec![],
            phones: vec![String::from("932123456")],
        };
        let mario = User {
            name: String::from("Mario Bro"),
            emails: vec![String::from("mariobro@email.com")],
            phones: vec![String::from("123456789")],
        };
        let mario_emal = User {
            name: String::from(""),
            emails: vec![String::from("mariobro@email.com")],
            phones: vec![],
        };
        let pepa = User {
            name: String::from("Pepa la pera"),
            emails: vec![String::from("")],
            phones: vec![String::from("")],
        };

        assert_eq!(format!("{:?}", juan.filter(&juan)), "Inclusive");
        assert_eq!(format!("{:?}", juan.filter(&juan_name)), "Similar(1.0)");
        assert_eq!(format!("{:?}", juan.filter(&juan_phone)), "Inclusive");
        assert_eq!(format!("{:?}", juan.filter(&mario)), "Similar(0.5)");
        assert_eq!(format!("{:?}", juan.filter(&pepa)), "Inclusive");
        assert_eq!(format!("{:?}", mario.filter(&mario_emal)), "Inclusive");
    }

    #[test]
    fn filter_order() {
        let orders = [
            Order {
                dates: vec![UtcDate::utc_ymdh(2022, 10, 2, 0)],
                description: String::from("Pastilles de fre XL\n\n34€ en Sasr"),
                state: OrderState::Done,
            },
            Order {
                dates: vec![
                    UtcDate::utc_ymdh(2020, 2, 1, 0),
                    UtcDate::utc_ymdh(2020, 1, 1, 0),
                ],
                description: String::from("frena JA!!!!"),
                state: OrderState::Done,
            },
            Order {
                dates: vec![
                    UtcDate::utc_ymdh(2020, 2, 1, 0),
                    UtcDate::utc_ymdh(2020, 1, 1, 0),
                ],
                description: String::from("Me aburro!!!\nEn Sasr"),
                state: OrderState::Todo,
            },
        ];

        assert_eq!(format!("{:?}", orders[0].filter(&orders[0])), "Inclusive");
        assert_eq!(format!("{:?}", orders[1].filter(&orders[1])), "Inclusive");
        assert_eq!(
            format!("{:?}", orders[1].filter(&orders[0])),
            "Similar(0.2857143)"
        );
        assert_eq!(
            format!("{:?}", orders[0].filter(&orders[1])),
            "Similar(0.0)"
        );
        assert_eq!(
            format!("{:?}", orders[0].filter(&orders[2])),
            "Similar(0.25)"
        );
    }

    #[test]
    fn filter_expedient() {
        // Same vin
        assert_eq!(
            format!(
                "{:?}",
                Expedient {
                    description: String::from("any stuff"),
                    license_plate: String::from(""),
                    model: String::from("any stuff"),
                    orders: vec![],
                    users: vec![],
                    vin: String::from("2HGES16503H591599"),
                }
                .filter(&Expedient {
                    description: String::from("random"),
                    license_plate: String::from(""),
                    model: String::from("random"),
                    orders: vec![],
                    users: vec![],
                    vin: String::from("2HGES16503H591599"),
                })
            ),
            "Inclusive"
        );
        // Same vin different license_plate
        assert_eq!(
            format!(
                "{:?}",
                Expedient {
                    description: String::from("any stuff"),
                    license_plate: String::from("very"),
                    model: String::from("any stuff"),
                    orders: vec![],
                    users: vec![],
                    vin: String::from("2HGES16503H591599"),
                }
                .filter(&Expedient {
                    description: String::from("random"),
                    license_plate: String::from("different"),
                    model: String::from("random"),
                    orders: vec![],
                    users: vec![],
                    vin: String::from("2HGES16503H591599"),
                })
            ),
            "Inclusive"
        );
        // Same license_plate different vin
        assert_eq!(
            format!(
                "{:?}",
                Expedient {
                    description: String::from("any stuff"),
                    license_plate: String::from("5KEB573"),
                    model: String::from("any stuff"),
                    orders: vec![],
                    users: vec![],
                    vin: String::from("1RGEF16503R521594"),
                }
                .filter(&Expedient {
                    description: String::from("random"),
                    license_plate: String::from("5KEB573"),
                    model: String::from("random"),
                    orders: vec![],
                    users: vec![],
                    vin: String::from("2HGES16503H591599"),
                })
            ),
            "Inclusive"
        );
        // Same Users (Inclusive Users)
        assert_eq!(
            format!(
                "{:?}",
                Expedient {
                    description: String::from(""),
                    license_plate: String::from(""),
                    model: String::from(""),
                    orders: vec![],
                    users: vec![User {
                        name: String::from("Pepa"),
                        emails: vec![],
                        phones: vec![String::from("923149288")]
                    }],
                    vin: String::from(""),
                }
                .filter(&Expedient {
                    description: String::from(""),
                    license_plate: String::from(""),
                    model: String::from(""),
                    orders: vec![],
                    users: vec![User {
                        name: String::from("Pepa"),
                        emails: vec![],
                        phones: vec![String::from("923149288")]
                    }],
                    vin: String::from(""),
                })
            ),
            "Similar(1.0)"
        );
        // Same Users (Inclusive Users), Different license plate
        assert_eq!(
            format!(
                "{:?}",
                Expedient {
                    description: String::from(""),
                    license_plate: String::from("5KEB573"),
                    model: String::from(""),
                    orders: vec![],
                    users: vec![User {
                        name: String::from("Pepa"),
                        emails: vec![],
                        phones: vec![String::from("923149288")]
                    }],
                    vin: String::from(""),
                }
                .filter(&Expedient {
                    description: String::from(""),
                    license_plate: String::from("5WEC222"),
                    model: String::from(""),
                    orders: vec![],
                    users: vec![User {
                        name: String::from("Pepa"),
                        emails: vec![],
                        phones: vec![String::from("923149288")]
                    }],
                    vin: String::from(""),
                })
            ),
            "Similar(0.0)"
        );
        // Filter By description
        assert_eq!(
            format!(
                "{:?}",
                Expedient {
                    description: String::from("Audi vermell, de 4.2 persones"),
                    license_plate: String::from("5KEB573"),
                    model: String::from(""),
                    orders: vec![],
                    users: vec![User {
                        name: String::from("Pepa"),
                        emails: vec![],
                        phones: vec![String::from("923149288")]
                    }],
                    vin: String::from(""),
                }
                .filter(&Expedient {
                    description: String::from("Vermell Audi"),
                    license_plate: String::from(""),
                    model: String::from(""),
                    orders: vec![],
                    users: vec![],
                    vin: String::from(""),
                })
            ),
            "Similar(1.0)"
        );
        // Blanck expedients
        assert_eq!(
            format!(
                "{:?}",
                Expedient {
                    description: String::from(""),
                    license_plate: String::from(""),
                    model: String::from(""),
                    orders: vec![],
                    users: vec![],
                    vin: String::from(""),
                }
                .filter(&Expedient {
                    description: String::from(""),
                    license_plate: String::from(""),
                    model: String::from(""),
                    orders: vec![],
                    users: vec![],
                    vin: String::from(""),
                })
            ),
            "Similar(1.0)"
        );
    }

    #[test]
    fn filte_vec_users() {
        assert_eq!(
            format!(
                "{:?}",
                vec![User {
                    name: String::from("Pepa"),
                    emails: vec![],
                    phones: vec![String::from("923149288")]
                }]
                .filter(&vec![])
            ),
            "Inclusive"
        );
        assert_eq!(
            format!(
                "{:?}",
                vec![User {
                    name: String::from("Pepa"),
                    emails: vec![],
                    phones: vec![String::from("923149288")]
                }]
                .filter(&vec![User {
                    name: String::from("Pepa"),
                    emails: vec![],
                    phones: vec![String::from("923149288")]
                }])
            ),
            "Inclusive"
        );
    }
}
