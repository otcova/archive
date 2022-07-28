import { invoke } from "@tauri-apps/api/tauri"
import { Expedient, ExpedientId } from "./types"

export async function createExpedient(expedient: Expedient) {
	return await invoke("create_expedient", { expedient }) as ExpedientId
}

export async function updateExpedient(id: ExpedientId, expedient: Expedient) {
	console.log("    u", id, expedient)
	return await invoke("update_expedient", { id, expedient })
}

export async function deleteExpedient(id: ExpedientId) {
	return await invoke("delete_expedient", { id }) as ExpedientId
}

export async function readExpedient(id: ExpedientId): Promise<Expedient | undefined> {
	return await invoke("read_expedient", { id }) as Expedient | undefined
}