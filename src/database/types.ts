import { dataDir } from "@tauri-apps/api/path"
import { databaseDir } from "./databaseState"
import { UtcDate, utcDateNow } from "./date"

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
		orders: [{
			title: "",
			description: "",
			state: "Todo",
			date: utcDateNow(),
		}],
		user: "",
		vin: "",
	}
}

export function folderOfExpedient(expedient: Expedient) {
	const expedientHash = expedient.date.timespan.toString(36).toUpperCase()
	return databaseDir + "\\Expedients Folder\\" + expedientHash
}