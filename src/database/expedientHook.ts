import { invoke, InvokeArgs } from '@tauri-apps/api/tauri'
import { Accessor, createEffect, createSignal, onCleanup } from 'solid-js'
import { Expedient, ExpedientId, UtcDate } from './types'

declare global {
	interface Window {
		callbacks: Map<number, (...parameters: any) => void>,
		callHook: (hookId: number, ...parameters: any[]) => void
	}
}
window.callbacks = new Map()
window.callHook = (callbackId, data) => {
	if (window.callbacks.has(callbackId)) setTimeout(() => window.callbacks.get(callbackId)(data))
	else console.error("RUST CALL TO NON EXISTING CALLBACK ID")
}

let nextCallbackId = 0
function createCallback(callback: (args: any[]) => void) {
	const callbackId = nextCallbackId++
	window.callbacks.set(callbackId, callback)
	return callbackId
}
export type ListExpedientsHookOptions = {
	filter: Expedient,
	max_list_len: number,
}
export type ListOrdersHookOptionsSortBy = {
	sort_by: "Oldest" | "Newest",
	max_list_len: number,
	from_date: UtcDate,
	show_todo: boolean,
	show_urgent: boolean,
	show_pending: boolean,
	show_done: boolean,
}

export function createHook(hook_name: "expedient", id: ExpedientId): [Accessor<Expedient>];
export function createHook(hook_name: "list_expedients", options: ListExpedientsHookOptions);
export function createHook(hook_name: "list_orders", options: ListOrdersHookOptionsSortBy);
export function createHook(hook_name: string, options: object) {
	const [hookData, setHookData] = createSignal(null)
	const [hookOptions, setHookOptions] = createSignal(options)
	const [hookId, setHookId] = createSignal(null)

	const jsCallback = createCallback(setHookData)

	let needsCleanup = false
	const tryCleanup = (hookId) => {
		if (needsCleanup && Number.isInteger(hookId)) releaseHook({ jsCallback, hookId })
	}

	createEffect(async () => {
		let params
		if (hook_name == "expedient") params = { jsCallback, expedientId: hookOptions() }
		else params = { jsCallback, options: hookOptions() }

		const hookId = await invoke("hook_" + hook_name, params)

		setHookId(pastHookId => {
			if (pastHookId) invoke("release_hook", { hookId: pastHookId })
			return hookId
		})
		tryCleanup(hookId)
	})


	onCleanup(() => {
		needsCleanup = true
		tryCleanup(hookId())
	})

	return [hookData, setHookOptions]
}

async function releaseHook({ jsCallback, hookId }) {
	await invoke("release_hook", { hookId })
	window.callbacks.delete(jsCallback)
}