import { invoke } from "@tauri-apps/api/tauri"
import { Expedient, ExpedientId } from "."

export async function createExpedient(expedient: Expedient) {
	return await invoke("create_expedient", { expedient }) as ExpedientId
}

export async function updateExpedient(id: ExpedientId, expedient: Expedient) {
	return await invoke("update_expedient", { id, expedient }) as ExpedientId
}

export async function deleteExpedient(id: ExpedientId) {
	return await invoke("update_expedient", { id }) as ExpedientId
}