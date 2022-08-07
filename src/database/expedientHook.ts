import { invoke } from '@tauri-apps/api/tauri'
import { Accessor, createEffect, createSignal, on, onCleanup, Setter } from 'solid-js'
import { UtcDate } from './date'
import { Expedient, ExpedientId } from './types'

declare global {
	interface Window {
		callbacks: Map<number, (...parameters: any) => void>,
		callHook: (hookId: number, ...parameters: any[]) => void
	}
}
window.callbacks = new Map()
window.callHook = (callbackId, data) => {
	if (window.callbacks.has(callbackId)) setTimeout(() => window.callbacks.get(callbackId)(data))
	else setTimeout(() => console.error("RUST CALL TO NON EXISTING CALLBACK ID"))
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
	show_urgent: boolean,
	show_todo: boolean,
	show_awaiting: boolean,
	show_instore: boolean,
	show_done: boolean,
}

export function createHook(hook_name: "expedient", id: ExpedientId): [Accessor<Expedient | null>];
export function createHook(
	hook_name: "list_users" | "list_models" | "list_license_plates" | "list_vins" | "list_order_titles",
	filter: string,
	deferOptions?: { defer: true }
): [Accessor<string[] | null>, Setter<string>];
export function createHook(
	hook_name: "list_expedients",
	options: ListExpedientsHookOptions,
	deferOptions?: { defer: true }
);
export function createHook(hook_name: "list_orders", options: ListOrdersHookOptionsSortBy);
export function createHook(hook_name: string, options: {}, deferOptions?: { defer: true }) {
	const [hookData, setHookData] = createSignal(null)
	const [hookOptions, setHookOptions] = createSignal(options)
	let hookId = null
	let cleaned = false

	const jsCallback = createCallback(setHookData)

	createEffect(on(hookOptions, async () => {
		let params
		if (hook_name == "expedient") params = { jsCallback, expedientId: hookOptions() }
		else if (typeof hookOptions() == "string") params = { jsCallback, filter: hookOptions() }
		else params = { jsCallback, options: hookOptions() }

		const newHookId = await invoke("hook_" + hook_name, params)

		if (hookId) invoke("release_hook", { hookId })
		hookId = newHookId
		if (cleaned) releaseHook({ jsCallback, hookId })

	}, deferOptions))


	onCleanup(() => {
		cleaned = true
		if (hookId) {
			releaseHook({ jsCallback, hookId })
			hookId = null
		}
	})

	return [hookData, setHookOptions]
}

async function releaseHook({ jsCallback, hookId }) {
	await invoke("release_hook", { hookId })
	window.callbacks.delete(jsCallback)
}