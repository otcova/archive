import { Accessor, createEffect, onCleanup, onMount } from "solid-js";
import { bindKey } from "./bindKey";

type CursorPosition = {
	elementPath: number[],
	start: number,
	end: number,
}

type Instant<T> = {
	data: T,
	cursorBefore: CursorPosition | null,
	cursorAfter: CursorPosition | null,
}

export function undoSignal<T>(signal: Accessor<T>, setSignal: (T) => void, container: HTMLElement) {

	const history: Instant<T>[] = []
	let historyIndex = 0
	let undoingSignal = false

	const lastCursorPosition = trackCursorPosition(container)

	createEffect(() => {
		if (signal() && !undoingSignal) {
			history.splice(0, historyIndex)
			history.unshift({
				data: signal(),
				cursorBefore: lastCursorPosition[0],
				cursorAfter: getCurrentCursorPosition(container),
			})
			historyIndex = 0
			if (history.length > 1000) history.pop()
		}
		undoingSignal = false
	})

	const undo = () => restoreHistory(1)
	const redu = () => restoreHistory(-1)

	const restoreHistory = (historyOffset: number) => {
		if (history.length) {
			undoingSignal = true
			historyIndex = Math.min(Math.max(historyIndex + historyOffset, 0), history.length - 1)
			setSignal(history[historyIndex].data)
			if (historyOffset > 0) setCursorPosition(history[historyIndex - historyOffset].cursorBefore, container)
			else setCursorPosition(history[historyIndex].cursorAfter, container)
		}
	}

	bindKey(document, "CTRL Z", undo)
	bindKey(document, "CTRL SHIFT Z", redu)
	bindKey(document, "CTRL Y", redu)
}

function getCurrentCursorPosition(relativeToContainer: HTMLElement): CursorPosition | null {
	const elementPath = []
	const selection = window.getSelection();
	if (!selection.rangeCount) return null

	let start = selection.getRangeAt(0).startOffset
	let end = selection.getRangeAt(0).endOffset

	let element = selection.anchorNode
	if (element["children"] && element["children"].length) {
		const input = element["children"][0] as HTMLInputElement
		start = input.selectionStart
		end = input.selectionEnd
		element = input
	}

	while (element && element.parentNode && !relativeToContainer.isSameNode(element)) {
		let index = Array.from(element.parentNode.childNodes)
			.findIndex(node => node.isSameNode(element))

		elementPath.unshift(index)
		element = element.parentNode
	}
	if (!relativeToContainer.isSameNode(element)) return null
	return { elementPath, start, end }
}

function setCursorPosition(cursorPosition: CursorPosition | null, relativeToContainer: HTMLElement) {
	if (!cursorPosition) return null
	let element: Node = relativeToContainer
	for (const index of cursorPosition.elementPath) {
		element = element.childNodes.item(index)
	}
	if (element["tagName"] && element["tagName"] == "INPUT") {
		const input = element as HTMLInputElement
		input.select()
		input.setSelectionRange(cursorPosition.start, cursorPosition.end)
	} else {
		window.getSelection().removeAllRanges()
		const range = new Range()
		range.setStart(element, cursorPosition.start)
		range.setEnd(element, cursorPosition.end)
		window.getSelection().addRange(range)
	}
}

function trackCursorPosition(relativeToContainer: HTMLElement) {
	const currentCursorPosition = [null]

	const recordPosition = () => setTimeout(() => {
		currentCursorPosition[0] = getCurrentCursorPosition(relativeToContainer)
	})

	onMount(() => {
		relativeToContainer.addEventListener("keydown", recordPosition)
		relativeToContainer.addEventListener("mousedown", recordPosition)
		relativeToContainer.addEventListener("mousemove", recordPosition)
	})
	onCleanup(() => {
		relativeToContainer.removeEventListener("keydown", recordPosition)
		relativeToContainer.removeEventListener("mousedown", recordPosition)
		relativeToContainer.removeEventListener("mousemove", recordPosition)
	})

	return currentCursorPosition
}