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
	year: number
	month: number,
	day: number,
	hour: number,
}

export function compareDate(a: LocalDate, b: LocalDate) {
	if (a.year > b.year) return 1
	if (a.year < b.year) return -1
	if (a.month > b.month) return 1
	if (a.month < b.month) return -1
	if (a.day > b.day) return 1
	if (a.day < b.day) return -1
	if (a.hour > b.hour) return 1
	if (a.hour < b.hour) return -1
	return 0
}

export function localDateToString(date: LocalDate): string {
	return date.day + " - " + date.month + " - " + date.year
}

export function localDateNow(): LocalDate {
	const date = new Date()
	return {
		year: date.getFullYear(),
		month: date.getMonth() + 1,
		day: date.getDate(),
		hour: date.getHours(),
	}
}

export function equalDay(a: LocalDate, b: LocalDate) {
	return a.day == b.day && a.month == b.month && a.year == b.year
}

export type Order = {
	title: string,
	description: string,
	state: OrderState,
	date: LocalDate,
}

export type OrderState = "Urgent" | "Todo" | "Pending" | "Done"

export const expedientUtils = {
	strUsers: (expedient: Expedient) => {
		let str = ""
		expedient.users.forEach(user => str += user.name + " | ")
		return str.substring(0, str.length - 3)
	},
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
	},
	newBlank: () => ({
		date: localDateNow(),
		description: "",
		license_plate: "",
		model: "",
		orders: [{
			title: "",
			description: "",
			state: "Todo",
			date: localDateNow(),
		}],
		users: [],
		vin: "",
	})
}