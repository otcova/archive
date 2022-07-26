export type ExpedientId = { DYNAMIC: number } | { ANCIENT: number }

export type Expedient = {
	date: UtcDate,
	description: string,
	license_plate: string,
	model: string,
	orders: Order[],
	user: User,
	vin: string
}

export type User = string

export type UtcDate = {
	year: number
	month: number,
	day: number,
	hour: number,
}

export function compareDate(a: UtcDate, b: UtcDate) {
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

export function utcToJsDate(utcDate: UtcDate): Date {
	return new Date(
		`${utcDate.month}/${utcDate.day}/${utcDate.year} ${utcDate.hour}:0 UTC`
	)
}

export function jsDateToUtc(jsDate: Date): UtcDate {
	return {
		year: jsDate.getUTCFullYear(),
		month: jsDate.getUTCMonth() + 1,
		day: jsDate.getUTCDate(),
		hour: jsDate.getUTCHours(),
	}
}

export function utcDateToString(utcDate: UtcDate): string {
	const jsDate = utcToJsDate(utcDate)
	const today = utcDateNow()
	if (equalDay(utcDate, today)) {
		if (jsDate.getHours() < 14) return "Matí"
		else return "Tarda"
	} else if (equalDay(utcDate, yesterdayOf(today))) {
		return "Ahir"
	} else if (equalDay(yesterdayOf(utcDate), today)) {
		return "Demà"
	}
	return jsDate.getDate() + " - " + (jsDate.getMonth() + 1) + " - " + jsDate.getFullYear()
}

export function utcDateNow(): UtcDate {
	const date = new Date()
	return {
		year: date.getUTCFullYear(),
		month: date.getUTCMonth() + 1,
		day: date.getUTCDate(),
		hour: date.getUTCHours(),
	}
}

export function utcDateFuture(): UtcDate {
	return { year: 30000, month: 1, day: 1, hour: 0 }
}

export function equalDay(utcA: UtcDate, utcB: UtcDate) {
	const a = utcToJsDate(utcA)
	const b = utcToJsDate(utcB)
	return a.toLocaleDateString() == b.toLocaleDateString()
}

export function yesterdayOf(utcDate: UtcDate) {
	return addHours(utcDate, -24)
}

export function addHours(utcDate: UtcDate, hours: number): UtcDate {
	let date = utcToJsDate(utcDate)
	date.setHours(date.getHours() + hours)
	return jsDateToUtc(date)
}

export type Order = {
	title: string,
	description: string,
	state: OrderState,
	date: UtcDate,
}

export type OrderState = "Urgent" | "Todo" | "Awaiting" | "InStore" | "Done"

export const expedientUtils = {
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
	newBlank: (): Expedient => ({
		date: utcDateNow(),
		description: "",
		license_plate: "",
		model: "",
		orders: [{
			title: "",
			description: "",
			state: "Todo",
			date: utcDateNow(),
		}],
		user: "",
		vin: "",
	})
}