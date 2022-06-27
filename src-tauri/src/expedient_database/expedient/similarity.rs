pub use super::*;
use crate::mean::*;

pub trait Similarity {
    fn similarity(&self, other: &Self) -> f32;
}

impl Similarity for str {
    fn similarity(&self, other: &Self) -> f32 {
        let a = self.to_lowercase();
        let b = other.to_lowercase();

        let a_in_b_match = a
            .split_whitespace()
            .map(|keyword| b.contains(keyword) as u8 as f32)
            .mean::<f32>();
        let b_in_a_match = b
            .split_whitespace()
            .map(|keyword| a.contains(keyword) as u8 as f32)
            .mean::<f32>();

        (a_in_b_match + b_in_a_match) / 2.0
    }
}

impl Similarity for String {
    fn similarity(&self, other: &Self) -> f32 {
        let a = self.to_lowercase();
        let b = other.to_lowercase();

        let a_in_b_match = a
            .split_whitespace()
            .map(|keyword| b.contains(keyword) as u8 as f32)
            .mean::<f32>();
        let b_in_a_match = b
            .split_whitespace()
            .map(|keyword| a.contains(keyword) as u8 as f32)
            .mean::<f32>();

        (a_in_b_match + b_in_a_match) / 2.0
    }
}

impl Similarity for Order {
    fn similarity(&self, other: &Self) -> f32 {
        [
            match_if_exist(&self.title, &other.title, 1.),
            match_if_exist(&self.description, &other.description, 1.),
        ]
        .into_iter()
        .flatten()
        .weighted_mean()
    }
}

impl Similarity for User {
    fn similarity(&self, other: &Self) -> f32 {
        let email_match = mean_match_if_exist(&self.emails, &other.emails, 4.);
        let phone_match = mean_match_if_exist(&self.phones, &other.phones, 4.);
        let name_match = Some((self.name.similarity(&other.name), 2.));

        [email_match, phone_match, name_match]
            .into_iter()
            .flatten()
            .weighted_mean()
    }
}

impl Similarity for Expedient {
    fn similarity(&self, other: &Self) -> f32 {
        [
            mean_match_if_exist(&self.users, &other.users, 2.),
            mean_match_if_exist(&self.orders, &other.orders, 1.),
            match_if_exist(&self.description, &other.description, 1.),
            match_if_exist(&self.model, &other.model, 1.),
            match_if_exist(&self.license_plate, &other.license_plate, 10.),
            match_if_exist(&self.vin, &other.vin, 10.),
        ]
        .into_iter()
        .flatten()
        .weighted_mean()
    }
}

fn match_if_exist(a: &String, b: &String, weight: f32) -> Option<(f32, f32)> {
    if a.len() == 0 && b.len() == 0 {
        None
    } else {
        Some((a.similarity(&b), weight))
    }
}

fn mean_match_if_exist<T: Similarity>(
    list_a: &Vec<T>,
    list_b: &Vec<T>,
    weight: f32,
) -> Option<(f32, f32)> {
    if list_a.len() == 0 && list_b.len() == 0 {
        None
    } else {
        Some((
            list_a
                .iter()
                .map::<f32, _>(|user_a| {
                    list_b.iter().map(|user_b| user_a.similarity(user_b)).mean()
                })
                .mean(),
            weight,
        ))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn str() {
        assert_eq!(0., "".similarity(&""));
        assert_eq!(0., "some word".similarity(&""));
        assert_eq!(0., "".similarity(&"any"));
        assert_eq!(0.5, "pepe Mariano".similarity(&"Pepe Rodrigo"));
        assert_eq!(0.75, "pepe".similarity(&"Pepe Mariano"));
        assert_eq!(0.75, "Pepe Mariano".similarity(&"pepe"));
        assert_eq!(1., "equal".similarity(&"EQUAL"));
        assert_eq!(1., "same words".similarity(&"same words"));
    }

    #[test]
    fn string() {
        assert_eq!(0., String::from("").similarity(&String::from("")));
        assert_eq!(0., String::from("some word").similarity(&String::from("")));
        assert_eq!(0., String::from("").similarity(&String::from("any")));
        assert_eq!(0.5, String::from("p Ma").similarity(&String::from("P R")));
        assert_eq!(0.75, String::from("p").similarity(&String::from("P M")));
        assert_eq!(0.75, String::from("P M").similarity(&String::from("p")));
        assert_eq!(1., String::from("equal").similarity(&String::from("EQUAL")));
        assert_eq!(1., String::from("= w").similarity(&String::from("= w")));
    }

    #[test]
    fn order() {
        let orders = [
            Order {
                date: UtcDate::utc_ymdh(2022, 10, 2, 0),
                title: String::from("Pastilles Fre"),
                description: String::from("Pastilles de fre XL\n\n34€ en Sasr"),
                state: OrderState::Done,
            },
            Order {
                date: UtcDate::utc_ymdh(2020, 2, 1, 0),
                title: String::from("Frena Ara"),
                description: String::from("frena JA!!!!"),
                state: OrderState::Done,
            },
            Order {
                date: UtcDate::utc_ymdh(2020, 2, 1, 0),
                title: String::from("Fre"),
                description: String::from("Me aburro!!!\nEn Sasr"),
                state: OrderState::Todo,
            },
        ];

        assert_eq!(1., orders[0].similarity(&orders[0]));
        assert_eq!(1., orders[1].similarity(&orders[1]));
        assert_eq!(1., orders[2].similarity(&orders[2]));
        assert!(orders[0].similarity(&orders[2]) > orders[0].similarity(&orders[1]));
        assert_eq!(orders[0].similarity(&orders[1]), orders[1].similarity(&orders[0]));
    }

    #[test]
    fn user() {
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
            emails: vec![],
            phones: vec![],
        };

        assert_eq!(1., juan.similarity(&juan));
        assert_eq!(2. / 3., juan.similarity(&juan_phone));
        assert_eq!(0.4, mario.similarity(&mario_emal));
        assert_eq!(2. / 9., juan.similarity(&juan_name));
        assert_eq!(1. / 12., juan.similarity(&mario));
        assert_eq!(0.0, juan.similarity(&pepa));
    }

    #[test]
    fn expedient() {
        // Same vin
        assert_eq!(
            5. / 6.,
            Expedient {
                description: String::from("any stuff"),
                license_plate: String::from(""),
                model: String::from("any stuff"),
                orders: vec![],
                users: vec![],
                vin: String::from("2HGES16503H591599"),
                date: UtcDate {
                    year: 2010,
                    month: 1,
                    day: 3,
                    hour: 23
                }
            }
            .similarity(&Expedient {
                description: String::from("random"),
                license_plate: String::from(""),
                model: String::from("random"),
                orders: vec![],
                users: vec![],
                vin: String::from("2HGES16503H591599"),
                date: UtcDate {
                    year: 2010,
                    month: 1,
                    day: 3,
                    hour: 23
                }
            })
        );
        // Same vin different license_plate
        assert_eq!(
            5. / 11.,
            Expedient {
                description: String::from("any stuff"),
                license_plate: String::from("very"),
                model: String::from("any stuff"),
                orders: vec![],
                users: vec![],
                vin: String::from("2HGES16503H591599"),
                date: UtcDate {
                    year: 2010,
                    month: 1,
                    day: 3,
                    hour: 23
                }
            }
            .similarity(&Expedient {
                description: String::from("random"),
                license_plate: String::from("different"),
                model: String::from("random"),
                orders: vec![],
                users: vec![],
                vin: String::from("2HGES16503H591599"),
                date: UtcDate {
                    year: 2010,
                    month: 1,
                    day: 3,
                    hour: 23
                }
            })
        );
        // Same license_plate different vin
        assert_eq!(
            5. / 11.,
            Expedient {
                description: String::from("any stuff"),
                license_plate: String::from("5KEB573"),
                model: String::from("any stuff"),
                orders: vec![],
                users: vec![],
                vin: String::from("1RGEF16503R521594"),
                date: UtcDate {
                    year: 2010,
                    month: 1,
                    day: 3,
                    hour: 23
                }
            }
            .similarity(&Expedient {
                description: String::from("random"),
                license_plate: String::from("5KEB573"),
                model: String::from("random"),
                orders: vec![],
                users: vec![],
                vin: String::from("2HGES16503H591599"),
                date: UtcDate {
                    year: 2010,
                    month: 1,
                    day: 3,
                    hour: 23
                }
            })
        );
        // Same Users (Inclusive Users)
        assert_eq!(
            1.0,
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
                date: UtcDate {
                    year: 2010,
                    month: 1,
                    day: 3,
                    hour: 23
                }
            }
            .similarity(&Expedient {
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
                date: UtcDate {
                    year: 2010,
                    month: 1,
                    day: 3,
                    hour: 23
                }
            })
        );
        // Same Users (Inclusive Users), Different license plate
        assert_eq!(
            1. / 6.,
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
                date: UtcDate {
                    year: 2010,
                    month: 1,
                    day: 3,
                    hour: 23
                }
            }
            .similarity(&Expedient {
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
                date: UtcDate {
                    year: 2010,
                    month: 1,
                    day: 3,
                    hour: 23
                }
            })
        );
        // Filter By description
        assert_eq!(
            3. / 65.,
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
                date: UtcDate {
                    year: 2010,
                    month: 1,
                    day: 3,
                    hour: 23
                }
            }
            .similarity(&Expedient {
                description: String::from("Vermell Audi"),
                license_plate: String::from(""),
                model: String::from(""),
                orders: vec![],
                users: vec![],
                vin: String::from(""),
                date: UtcDate {
                    year: 2010,
                    month: 1,
                    day: 3,
                    hour: 23
                }
            })
        );
        // Blanck expedients
        assert_eq!(
            0.0,
            Expedient {
                description: String::from(""),
                license_plate: String::from(""),
                model: String::from(""),
                orders: vec![],
                users: vec![],
                vin: String::from(""),
                date: UtcDate {
                    year: 2010,
                    month: 1,
                    day: 3,
                    hour: 23
                }
            }
            .similarity(&Expedient {
                description: String::from(""),
                license_plate: String::from(""),
                model: String::from(""),
                orders: vec![],
                users: vec![],
                vin: String::from(""),
                date: UtcDate {
                    year: 2010,
                    month: 1,
                    day: 3,
                    hour: 23
                }
            })
        );
    }
}
