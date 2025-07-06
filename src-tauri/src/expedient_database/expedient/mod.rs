mod similarity;
use crate::{chunked_database, collections::UtcDate};
use serde::{Deserialize, Serialize};
pub use similarity::*;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Expedient {
    pub user: String,
    pub model: String,
    pub license_plate: String,
    pub vin: String,
    pub description: String,
    pub orders: Vec<Order>,
    pub date: UtcDate,
}

fn eq_ignore_whitespace_case(a: &str, b: &str) -> bool {
    let mut ia = a.chars().filter_map(|c| {
        if c.is_whitespace() {
            None
        } else {
            Some(c.to_ascii_lowercase())
        }
    });
    let mut ib = b.chars().filter_map(|c| {
        if c.is_whitespace() {
            None
        } else {
            Some(c.to_ascii_lowercase())
        }
    });

    ia.eq(ib)
}

impl PartialEq for Expedient {
    fn eq(&self, other: &Self) -> bool {
        eq_ignore_whitespace_case(&self.user, &other.user)
            && eq_ignore_whitespace_case(&self.model, &other.model)
            && eq_ignore_whitespace_case(&self.license_plate, &other.license_plate)
            && eq_ignore_whitespace_case(&self.vin, &other.vin)
            && eq_ignore_whitespace_case(&self.description, &other.description)
            && self.orders == other.orders
            && self.date == other.date
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Order {
    pub date: UtcDate,
    pub title: String,
    pub description: String,
    pub state: OrderState,
}

impl PartialEq for Order {
    fn eq(&self, other: &Self) -> bool {
        self.date == other.date
            && eq_ignore_whitespace_case(&self.title, &other.title)
            && eq_ignore_whitespace_case(&self.description, &other.description)
            && self.state == other.state
    }
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Copy)]
pub enum OrderState {
    Urgent,
    Todo,
    Awaiting,
    InStore,
    Done,
}

impl Expedient {
    pub fn newest_date(&self) -> UtcDate {
        self.orders
            .iter()
            .map(|order| order.date.date_hash())
            .max()
            .map_or(self.date, |hash| UtcDate::from_hash(hash))
    }
}

impl chunked_database::Item for Expedient {
    fn date(&self) -> i64 {
        self.newest_date().date_hash()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::chunked_database::Item;

    #[test]
    fn date_hash() {
        let date_a = UtcDate::ymdh(2021, 12, 23, 9);
        let date_b = UtcDate::ymdh(2021, 12, 23, 9);

        let expedient = Expedient {
            description: "".into(),
            license_plate: "".into(),
            model: "".into(),
            orders: vec![
                Order {
                    date: date_a,
                    title: ":)".into(),
                    description: "".into(),
                    state: OrderState::Urgent,
                },
                Order {
                    date: date_b,
                    title: "".into(),
                    description: "few ew fgwegfwe".into(),
                    state: OrderState::Done,
                },
            ],
            user: "".into(),
            vin: "".into(),
            date: UtcDate::ymdh(2010, 1, 3, 23),
        };

        assert_eq!(date_a.date_hash(), expedient.date());
    }

    #[test]
    fn sort_expedients() {
        let sorted_expedients = vec![
            Expedient {
                description: "any stuff".into(),
                license_plate: "very".into(),
                model: "any stuff".into(),
                orders: vec![
                    Order {
                        date: UtcDate::ymdh(2022, 10, 2, 0),
                        title: "Placa".into(),
                        description: "Pastilles de fre XL\n\n34€ en Sasr".into(),
                        state: OrderState::Done,
                    },
                    Order {
                        date: UtcDate::ymdh(2022, 10, 2, 10),
                        title: "Coses Rares".into(),
                        description: "Pastilles de fre XL\n\n34€ en Sasr".into(),
                        state: OrderState::Done,
                    },
                ],
                user: "".into(),
                vin: "2HGES16503H591599".into(),
                date: UtcDate::ymdh(2010, 1, 3, 23),
            },
            Expedient {
                description: "any stuff".into(),
                license_plate: "very".into(),
                model: "any stuff".into(),
                orders: vec![Order {
                    date: UtcDate::ymdh(2022, 10, 2, 0),
                    title: "Ell".into(),
                    description: "Pastilles de fre XL\n\n34€ en Sasr".into(),
                    state: OrderState::Done,
                }],
                user: "".into(),
                vin: "2HGES16503H591599".into(),
                date: UtcDate::ymdh(2010, 1, 3, 23),
            },
            Expedient {
                description: "any stuff".into(),
                license_plate: "very".into(),
                model: "any stuff".into(),
                orders: vec![],
                user: "".into(),
                vin: "2HGES16503H591599".into(),
                date: UtcDate::ymdh(2010, 1, 3, 23),
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
}
