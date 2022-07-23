import { createSignal } from "solid-js"

export const [databaseError, setDatabaseError] = createSignal<
	{ error?: string, msg?: string, button?: string, action?: () => void } | null
>({ msg: "Obrint Base de Dades" })

export { expedientUtils } from "./types"
export type { Expedient, ExpedientId, LocalDate, Order, User } from "./types"

import "./databaseState"
import "./temporal"