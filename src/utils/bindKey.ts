import { onCleanup, onMount } from "solid-js"

export type Digit = "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9"
export type Letter = "A" | "B" | "C" | "D" | "E" | "F" | "G" | "H" | "I" | "J" | "K" | "L"
	| "M" | "N" | "O" | "P" | "Q" | "R" | "S" | "T" | "U" | "V" | "W" | "X" | "Y" | "Z"
export type Key = "Escape" | "Enter" | "Tab" | Letter | Digit

export type KeyMap = Key | `Ctrl ${Key}` | `Alt ${Key}` | `Shift ${Key}`
	| `Ctrl Alt ${Key}` | `Ctrl Shift ${Key}`

type Listener = {
	addEventListener: (type: "keydown", listener: (event: KeyboardEvent) => void) => void,
	removeEventListener: (type: "keydown", listener: (event: KeyboardEvent) => void) => void,
}

export function bindKey(element: Listener, keymap: KeyMap, listener: () => "propagate" | void) {
	const onKeyDown = (event: KeyboardEvent) => {
		if (keymap.includes("Ctrl") != event.ctrlKey) return
		if (keymap.includes("Shift") != event.shiftKey) return
		if (keymap.includes("Alt") != event.altKey) return
		const key = keymap.split(" ").pop()
		if (event.code == key || event.code == "Key" + key || event.code == "Digit" + key) {
			if (!listener()) {
				event.stopPropagation()
				event.preventDefault()
			}
		}
	}
	onMount(() => element.addEventListener("keydown", onKeyDown))
	onCleanup(() => element.removeEventListener("keydown", onKeyDown))
}