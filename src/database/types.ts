import { databaseDir } from "./databaseState"
import { compareUtcDate, UtcDate, utcDateNow } from "./date"

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

export type Order = {
	title: string,
	description: string,
	state: OrderState,
	date: UtcDate,
}

export type OrderState = "Urgent" | "Todo" | "Awaiting" | "InStore" | "Done"

export function newBlankExpedient(): Expedient {
	return {
		date: utcDateNow(),
		description: "",
		license_plate: "",
		model: "",
		orders: [newBlankOrder()],
		user: "",
		vin: "",
	}
}

export function folderOfExpedient(expedient: Expedient) {
	const expedientHash = expedient.date.timespan.toString(36).toUpperCase()
	return databaseDir + "\\Expedients Folder\\" + expedientHash
}

export function newBlankOrder(): Order {
	return {
		title: "",
		description: "",
		state: "Todo",
		date: utcDateNow(),
	}
}

export function sortOrdersByPriority(orders: Order[]): [Order, number][] {
	const arrangedOrders: [Order, number][] = []

	const indexedOrders: [Order, number][] = [...orders].map((order, index) => [order, index])
	const sortedOrders = indexedOrders.sort(([a], [b]) => -compareUtcDate(a.date, b.date))

	for (const state of ["Urgent", "Todo", "InStore", "Awaiting", "Done"]) {
		for (const order of sortedOrders) {
			if (order[0].state == state)
				arrangedOrders.push(order)
		}
	}

	return arrangedOrders
}

export function userFirstName(user: User): string {
	for (const word of user.split(/\s/)) {
		if (word.trim().match(/^[a-z]+$/i)) {
			return word.trim()
		}
	}
	return user.split(/\s/)[0].trim()
}