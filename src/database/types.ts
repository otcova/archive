export type ExpedientId = { DYNAMIC: number } | { ANCIENT: number }

export type Expedient = {
	date: LocalDate,
	description: string,
	license_plate: string,
	model: string,
	orders: Order[],
	users: User[],
	vin: string
}

export type User = {
	name: string,
	phones: string[],
	emails: string[],
}

export type LocalDate = {
	day: number,
	hour: number,
	month: number,
	year: number
}

export type Order = {
	title: string,
	description: string,
	state: OrderState,
	date: LocalDate,
}

export type OrderState = "Urgent" | "Todo" | "Done"

export const expedientUtils = {
	strUsers: (expedient: Expedient) => {
		let str = ""
		expedient.users.forEach(user => str += user.name + " | ")
		return str.substring(0, str.length - 3)
	},
	strDate: (date: LocalDate) => date.day + " - " + date.month + " - " + date.year,
	futureDate: () => ({ year: 32767, month: 1, day: 1, hour: 1 }),
	globalOrderState: (expedient: Expedient): OrderState => (
		expedient.orders.reduce((state, order) =>
			state == "Done" || order.state == "Urgent" ? order.state : state
			, "Done")
	),
	setGlobalOrderState: (expedient: Expedient, newState: OrderState) => {
		let state = expedientUtils.globalOrderState(expedient)
		switch (newState) {
			case "Done":
				return expedient.orders.forEach(order => order.state = "Done")
				
			case "Todo":
				if (state == "Done")
					return expedient.orders.forEach(order => order.state = "Todo")
				else if (state == "Urgent")
					return expedient.orders.forEach(
						order => order.state = order.state == "Urgent" ? "Todo" : order.state)
						
			case "Urgent":
				if (state == "Done")
					return expedient.orders.forEach(order => order.state = "Urgent")
				else if (state == "Todo")
					return expedient.orders.forEach(
						order => order.state = order.state == "Todo" ? "Urgent" : order.state)
		}
	}

}