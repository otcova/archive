mod similarity;
use crate::{chunked_database, collections::UtcDate};
use serde::{Deserialize, Serialize};
pub use similarity::*;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Expedient {
    pub users: Vec<User>,
    pub model: String,
    pub license_plate: String,
    pub vin: String,
    pub description: String,
    pub orders: Vec<Order>,
    pub date: UtcDate,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Order {
    pub date: UtcDate,
    pub title: String,
    pub description: String,
    pub state: OrderState,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Clone, Copy)]
pub enum OrderState {
    Done,
    Todo,
    Urgent,
    Pending,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct User {
    pub name: String,
    pub emails: Vec<String>,
    pub phones: Vec<String>,
}

impl User {
    pub fn to_string(&self) -> String {
        [&self.name]
            .into_iter()
            .chain(self.emails.iter())
            .chain(self.phones.iter())
            .cloned()
            .intersperse(String::from(" "))
            .collect()
    }

    pub fn to_lowercase_string(&self) -> String {
        self.to_string().to_lowercase()
    }
}

impl Expedient {
    pub fn vin_is_complete(&self) -> bool {
        self.vin.len() >= 17
    }
    pub fn license_is_complete(&self) -> bool {
        self.license_plate.len() >= 7
    }
    pub fn global_order_state(&self) -> OrderState {
        self.orders
            .iter()
            .map(|order| order.state)
            .max()
            .unwrap_or(OrderState::Done)
    }
}

impl chunked_database::Item for Expedient {
    fn date(&self) -> i32 {
        self.orders
            .iter()
            .map(|order| order.date.date_hash())
            .max()
            .unwrap_or(self.date.date_hash())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::chunked_database::Item;

    #[test]
    fn date_hash() {
        let date_a = UtcDate {
            year: 2021,
            month: 12,
            day: 23,
            hour: 9,
        };

        let date_b = UtcDate {
            year: 2021,
            month: 12,
            day: 23,
            hour: 9,
        };

        let expedient = Expedient {
            description: String::from(""),
            license_plate: String::from(""),
            model: String::from(""),
            orders: vec![
                Order {
                    date: date_a,
                    title: String::from(":)"),
                    description: String::from(""),
                    state: OrderState::Urgent,
                },
                Order {
                    date: date_b,
                    title: String::from(""),
                    description: String::from("few ew fgwegfwe"),
                    state: OrderState::Done,
                },
            ],
            users: vec![],
            vin: String::from(""),
            date: UtcDate {
                year: 2010,
                month: 1,
                day: 3,
                hour: 23,
            },
        };

        assert_eq!(date_a.date_hash(), expedient.date());
    }

    #[test]
    fn sort_expedients() {
        let sorted_expedients = vec![
            Expedient {
                description: String::from("any stuff"),
                license_plate: String::from("very"),
                model: String::from("any stuff"),
                orders: vec![
                    Order {
                        date: UtcDate::utc_ymdh(2022, 10, 2, 0),
                        title: String::from("Placa"),
                        description: String::from("Pastilles de fre XL\n\n34€ en Sasr"),
                        state: OrderState::Done,
                    },
                    Order {
                        date: UtcDate::utc_ymdh(2022, 10, 2, 10),
                        title: String::from("Coses Rares"),
                        description: String::from("Pastilles de fre XL\n\n34€ en Sasr"),
                        state: OrderState::Done,
                    },
                ],
                users: vec![],
                vin: String::from("2HGES16503H591599"),
                date: UtcDate {
                    year: 2010,
                    month: 1,
                    day: 3,
                    hour: 23,
                },
            },
            Expedient {
                description: String::from("any stuff"),
                license_plate: String::from("very"),
                model: String::from("any stuff"),
                orders: vec![Order {
                    date: UtcDate::utc_ymdh(2022, 10, 2, 0),
                    title: String::from("Ell"),
                    description: String::from("Pastilles de fre XL\n\n34€ en Sasr"),
                    state: OrderState::Done,
                }],
                users: vec![],
                vin: String::from("2HGES16503H591599"),
                date: UtcDate {
                    year: 2010,
                    month: 1,
                    day: 3,
                    hour: 23,
                },
            },
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
                    hour: 23,
                },
            },
        ];
        let mut unsorted_expedients = vec![
            sorted_expedients[1].clone(),
            sorted_expedients[2].clone(),
            sorted_expedients[0].clone(),
        ];

        unsorted_expedients.sort_unstable_by_key(|e| -e.date());

        assert_eq!(sorted_expedients, unsorted_expedients,);
    }

    #[test]
    fn global_order_state() {
        let expedient = Expedient {
            description: String::from("any stuff"),
            license_plate: String::from("very"),
            model: String::from("any stuff"),
            orders: vec![
                Order {
                    date: UtcDate::utc_ymdh(2022, 10, 2, 0),
                    title: String::from("Pastilles de fre"),
                    description: String::from("Pastilles de fre XL\n\n34€ en Sasr"),
                    state: OrderState::Done,
                },
                Order {
                    date: UtcDate::utc_ymdh(2022, 10, 2, 10),
                    title: String::from(":O"),
                    description: String::from("Pastilles de fre XL\n\n34€ en Sasr"),
                    state: OrderState::Todo,
                },
            ],
            users: vec![],
            vin: String::from("2HGES16503H591599"),
            date: UtcDate {
                year: 2010,
                month: 1,
                day: 3,
                hour: 23,
            },
        };
        assert_eq!(OrderState::Todo, expedient.global_order_state());
    }
}
