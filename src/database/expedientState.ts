import { invoke } from "@tauri-apps/api/tauri"
import { Expedient, ExpedientId } from "./types"

export async function createExpedient(expedient: Expedient) {
	return await invoke("create_expedient", { expedient }) as ExpedientId
}

export async function updateExpedient(id: ExpedientId, expedient: Expedient) {
	return await invoke("update_expedient", { id, expedient })
}

export async function deleteExpedient(id: ExpedientId) {
	return await invoke("delete_expedient", { id }) as ExpedientId
}

export async function readExpedient(id: ExpedientId): Promise<Expedient | undefined> {
	return await invoke("read_expedient", { id }) as Expedient | undefined
}

export async function countExpedients(): Promise<number> {
	return await invoke("count_expedients")
}

export async function countOrders(): Promise<number> {
	return await invoke("count_orders")
}

export async function deleteRepeated(): Promise<number> {
	return await invoke("delete_repeated")
}