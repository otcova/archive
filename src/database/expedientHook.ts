import { invoke, InvokeArgs } from '@tauri-apps/api/tauri'
import { createEffect, createSignal, onCleanup } from 'solid-js'

declare global {
	interface Window {
		callbacks: Map<number, (...parameters: any) => void>,
		callHook: (hookId: number, ...parameters: any[]) => void
	}
}
window.callbacks = new Map()
window.callHook = (callbackId, data) => {
	if (window.callbacks.has(callbackId)) window.callbacks.get(callbackId)?.(data)
	else console.error("RUST CALL TO NON EXISTING CALLBACK ID")
}

let nextCallbackId = 0
function createCallback(callback: (args: any[]) => void) {
	const callbackId = nextCallbackId++
	window.callbacks.set(callbackId, callback)
	return callbackId;
}

export type HookType = "all_expedients" | "all_open_expedients"

export function createHook(hook_name: HookType, parameters?: InvokeArgs) {
	const [hookData, setHookData] = createSignal(null)
	const [hookParameters, setHookParamenters] = createSignal(parameters)
	const [hookId, setHookId] = createSignal(null)
	const [needsCleanup, setNeedsCleanup] = createSignal(false)

	const jsCallback = createCallback(setHookData)

	createEffect(async () => {
		const hookId = await invoke("hook_" + hook_name, { jsCallback, ...hookParameters() })
		setHookId(pastHookId => {
			if (pastHookId) invoke("release_hook", { hookId: pastHookId })
			return hookId
		})
	})
	
	createEffect(() => {
		if (needsCleanup() && hookId()) releaseHook({ jsCallback, hookId: hookId() })
	})
	
	onCleanup(() => setNeedsCleanup(true))
	
	return [hookData, setHookParamenters]
}

async function releaseHook({ jsCallback, hookId }) {
	await invoke("release_hook", { hookId })
	window.callbacks.delete(jsCallback)
}