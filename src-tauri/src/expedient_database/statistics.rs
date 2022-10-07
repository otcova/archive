use super::{ExpedientDatabase, OrderState, UtcDate};

impl<'a> ExpedientDatabase<'a> {
    pub fn done_commands_count_vs_days(&self, from_day: UtcDate) -> Vec<usize> {
        let mut day_count_list = vec![];
		let first_day_hash = from_day.day_hash();
		
        self.database
            .read()
            .unwrap()
            .iter_all()
            .map(|(_, expedient)| expedient.orders.iter())
            .flatten()
			.filter(|order| order.state == OrderState::Done)
			.map(|order| order.date.day_hash())
			.filter(|day_hash| *day_hash <= first_day_hash)
			.for_each(|day_hash| {
				let index = (first_day_hash - day_hash) as usize;
				if day_count_list.len() <= index {
					 day_count_list.resize(index + 1, 0);
				}
				day_count_list[index] += 1;
			});

        day_count_list.reverse();
		day_count_list
    }
}
