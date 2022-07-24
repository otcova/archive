import { createSignal } from "solid-js"

export const [databaseError, setDatabaseError] = createSignal<
	{ error?: string, msg?: string, button?: string, action?: () => void } | null
>({ msg: "Obrint Base de Dades" })

import "./databaseState"
import "./temporal"