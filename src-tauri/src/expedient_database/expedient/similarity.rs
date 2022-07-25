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

impl Similarity for Expedient {
    fn similarity(&self, other: &Self) -> f32 {
        [
            match_if_exist(&self.user, &other.user, 2.),
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
        assert_eq!(0., String::from("").similarity(&"".into()));
        assert_eq!(0., String::from("some word").similarity(&"".into()));
        assert_eq!(0., String::from("").similarity(&"any".into()));
        assert_eq!(0.5, String::from("p Ma").similarity(&"P R".into()));
        assert_eq!(0.75, String::from("p").similarity(&"P M".into()));
        assert_eq!(0.75, String::from("P M").similarity(&"p".into()));
        assert_eq!(1., String::from("equal").similarity(&"EQUAL".into()));
        assert_eq!(1., String::from("= w").similarity(&"= w".into()));
    }

    #[test]
    fn order() {
        let orders = [
            Order {
                date: UtcDate::utc_ymdh(2022, 10, 2, 0),
                title: "Pastilles Fre".into(),
                description: "Pastilles de fre XL\n\n34€ en Sasr".into(),
                state: OrderState::Done,
            },
            Order {
                date: UtcDate::utc_ymdh(2020, 2, 1, 0),
                title: "Frena Ara".into(),
                description: "frena JA!!!!".into(),
                state: OrderState::Done,
            },
            Order {
                date: UtcDate::utc_ymdh(2020, 2, 1, 0),
                title: "Fre".into(),
                description: "Me aburro!!!\nEn Sasr".into(),
                state: OrderState::Todo,
            },
        ];

        assert_eq!(1., orders[0].similarity(&orders[0]));
        assert_eq!(1., orders[1].similarity(&orders[1]));
        assert_eq!(1., orders[2].similarity(&orders[2]));
        assert!(orders[0].similarity(&orders[2]) > orders[0].similarity(&orders[1]));
        assert_eq!(
            orders[0].similarity(&orders[1]),
            orders[1].similarity(&orders[0])
        );
    }

    #[test]
    fn expedient() {
        // Same vin
        assert_eq!(
            5. / 6.,
            Expedient {
                description: "any stuff".into(),
                license_plate: "".into(),
                model: "any stuff".into(),
                orders: vec![],
                user: "".into(),
                vin: "2HGES16503H591599".into(),
                date: UtcDate {
                    year: 2010,
                    month: 1,
                    day: 3,
                    hour: 23
                }
            }
            .similarity(&Expedient {
                description: "random".into(),
                license_plate: "".into(),
                model: "random".into(),
                orders: vec![],
                user: "".into(),
                vin: "2HGES16503H591599".into(),
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
                description: "any stuff".into(),
                license_plate: "very".into(),
                model: "any stuff".into(),
                orders: vec![],
                user: "".into(),
                vin: "2HGES16503H591599".into(),
                date: UtcDate {
                    year: 2010,
                    month: 1,
                    day: 3,
                    hour: 23
                }
            }
            .similarity(&Expedient {
                description: "random".into(),
                license_plate: "different".into(),
                model: "random".into(),
                orders: vec![],
                user: "".into(),
                vin: "2HGES16503H591599".into(),
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
                description: "any stuff".into(),
                license_plate: "5KEB573".into(),
                model: "any stuff".into(),
                orders: vec![],
                user: "".into(),
                vin: "1RGEF16503R521594".into(),
                date: UtcDate {
                    year: 2010,
                    month: 1,
                    day: 3,
                    hour: 23
                }
            }
            .similarity(&Expedient {
                description: "random".into(),
                license_plate: "5KEB573".into(),
                model: "random".into(),
                orders: vec![],
                user: "".into(),
                vin: "2HGES16503H591599".into(),
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
                description: "".into(),
                license_plate: "".into(),
                model: "".into(),
                orders: vec![],
                user: "Pepa 923149288".into(),
                vin: "".into(),
                date: UtcDate {
                    year: 2010,
                    month: 1,
                    day: 3,
                    hour: 23
                }
            }
            .similarity(&Expedient {
                description: "".into(),
                license_plate: "".into(),
                model: "".into(),
                orders: vec![],
                user: "Pepa 923149288".into(),
                vin: "".into(),
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
                description: "".into(),
                license_plate: "5KEB573".into(),
                model: "".into(),
                orders: vec![],
                user: "Pepa 923149288".into(),
                vin: "".into(),
                date: UtcDate {
                    year: 2010,
                    month: 1,
                    day: 3,
                    hour: 23
                }
            }
            .similarity(&Expedient {
                description: "".into(),
                license_plate: "5WEC222".into(),
                model: "".into(),
                orders: vec![],
                user: "Pepa 923149288".into(),
                vin: "".into(),
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
                description: "Audi vermell, de 4.2 persones".into(),
                license_plate: "5KEB573".into(),
                model: "".into(),
                orders: vec![],
                user: "Pepa 923149288".into(),
                vin: "".into(),
                date: UtcDate {
                    year: 2010,
                    month: 1,
                    day: 3,
                    hour: 23
                }
            }
            .similarity(&Expedient {
                description: "Vermell Audi".into(),
                license_plate: "".into(),
                model: "".into(),
                orders: vec![],
                user: "".into(),
                vin: "".into(),
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
                description: "".into(),
                license_plate: "".into(),
                model: "".into(),
                orders: vec![],
                user: "".into(),
                vin: "".into(),
                date: UtcDate {
                    year: 2010,
                    month: 1,
                    day: 3,
                    hour: 23
                }
            }
            .similarity(&Expedient {
                description: "".into(),
                license_plate: "".into(),
                model: "".into(),
                orders: vec![],
                user: "".into(),
                vin: "".into(),
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
