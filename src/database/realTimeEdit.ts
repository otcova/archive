import { Accessor, createEffect, createSignal, on, Setter } from "solid-js";
import { createHook } from "./expedientHook";
import { updateExpedient } from "./expedientState";
import { Expedient, ExpedientId } from "./types";

export function realTimeDatabaseExpedientEditor(
	expedientId: ExpedientId,
): [Accessor<Expedient | null>, Setter<Expedient>] {

	const [editorValue, updateEditor] = createSignal<Expedient | null>(null)

	realTimeDatabaseEditor<Expedient>(
		createHook("expedient", expedientId)[0],
		(expedient) => updateExpedient(expedientId, expedient),
		editorValue,
		updateEditor,
	)

	return [editorValue, updateEditor]
}

let id = 0

export function realTimeDatabaseEditor<T>(
	databaseHookReceiver: Accessor<null | T>,
	sendUpdateToDatabase: (newValue: T) => void,
	editorValue: Accessor<null | T>,
	updateEditor: (newValue: T) => void) {

	let databaseValue: null | string = null
	let sendedValue: null | string = null

	const updateDatabase = () => {
		const value = editorValue()
		if (!value) return
		sendedValue = JSON.stringify(value)
		sendUpdateToDatabase(value)
	}

	createEffect(on(databaseHookReceiver, () => {
		const receivedData = databaseHookReceiver()
		databaseValue = JSON.stringify(receivedData)
		if (!receivedData) return
		if (sendedValue == databaseValue) {
			sendedValue = null
			if (JSON.stringify(editorValue()) != databaseValue)
				updateDatabase()
		} else {
			sendedValue = null
			updateEditor(receivedData)
		}
	}))

	createEffect(on(editorValue, () => {
		const editorState = editorValue()
		if (!editorState || JSON.stringify(editorState) == databaseValue) return
		if (sendedValue == null) updateDatabase()
	}))
}