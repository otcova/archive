import { onCleanup, onMount } from "solid-js"

type Letter = "A" | "B" | "C" | "D" | "E" | "F" | "G" | "H" | "I" | "J" | "K" | "L" | "M"
	| "N" | "O" | "P" | "Q" | "R" | "S" | "T" | "U" | "V" | "W" | "X" | "Y" | "Z"
export type KeyMap = Letter | `CTRL ${Letter}` | `ALT ${Letter}` | `SHIFT ${Letter}`
	| `CTRL ALT ${Letter}` | `CTRL SHIFT ${Letter}`

type Listener = {
	addEventListener: (type: "keydown", listener: (event: KeyboardEvent) => void) => void,
	removeEventListener: (type: "keydown", listener: (event: KeyboardEvent) => void) => void,
}

export function bindKey(element: Listener, keymap: KeyMap, listener: () => void) {
	const onKeyDown = (event: KeyboardEvent) => {
		if (keymap.includes("CTRL") != event.ctrlKey) return
		if (keymap.includes("SHIFT") != event.shiftKey) return
		if (keymap.includes("ALT") != event.altKey) return
		if (event.code == "Key" + keymap.split(" ").pop()) {
			listener()
			event.stopPropagation()
			event.preventDefault()
		}
	}
	onMount(() => element.addEventListener("keydown", onKeyDown))
	onCleanup(() => element.removeEventListener("keydown", onKeyDown))
}