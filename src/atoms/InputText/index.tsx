import style from "./InputText.module.sass"

type Props = {
	placeholder?: string,
	ref?: HTMLInputElement | ((el: HTMLInputElement) => void),
	value?: string,
	onChange?: (data: string) => void,
	autoFormat?: ("startWordCapital" | "firstCapital" | "allCapital" | "spaceAfterNumber")[]
	charRegex?: RegExp,
	maxLen?: number,
	noStyle?: boolean,
	selectOnFocus?: boolean,
}

export default function InputText(props: Props) {

	const forcePattern = event => {
		if (!event.data) return
		if (props.charRegex && !props.charRegex.test(event.data))
			event.preventDefault()
	}

	const onInput = event => {
		const input = event.target as HTMLInputElement

		if (props.maxLen) {
			maintainCursorPosition(input, () => {
				input.value = input.value.slice(0, props.maxLen)
			})
		}
		if (props.autoFormat) {
			maintainCursorPosition(input, (cursorPos: number) => {
				let cursorOffset = 0
				const initialValueLength = input.value.length
				input.value = input.value.trimStart().replace(/\s+/g, " ")

				if (props.autoFormat.includes("startWordCapital")) {
					input.value = capitalizeFirstLetterOfWord(input.value)
				} if (props.autoFormat.includes("firstCapital")) {
					input.value = capitalizeFirstLetter(input.value)
				} if (props.autoFormat.includes("allCapital")) {
					input.value = input.value.toUpperCase()
				} if (props.autoFormat.includes("spaceAfterNumber")) {
					const arroundCursor = input.value.substring(cursorPos - 1, cursorPos + 1)
					input.value = input.value.replace(/(?<=\d)(?=[a-zA-Z])/g, " ")
					if (arroundCursor.length == 2 && arroundCursor[0].match(/\d/) &&
						arroundCursor[1].match(/[a-zA-Z]/)) {
						cursorOffset -= 1
					}
				}
				return input.value.length - initialValueLength
			})
		}
		props.onChange?.(input.value)
	}

	const onFocus = event => {
		if (props.selectOnFocus) event.target.select()
	}

	if (props.value) props.onChange?.(props.value)

	const onMouseDown = (event: MouseEvent) => {
		if (event.button == 2) event.preventDefault()
	}

	return <input
		type="text"
		ref={props.ref}
		onMouseDown={onMouseDown}
		value={props.value ?? ""}
		onInput={onInput}
		onFocus={onFocus}
		onBeforeInput={forcePattern}
		class={props.noStyle ? style.input_minimal : style.input}
		placeholder={props.placeholder}
		spellcheck={false}
	/>
}

function capitalizeFirstLetter(string) {
	return string.charAt(0).toUpperCase() + string.slice(1)
}

function capitalizeFirstLetterOfWord(string) {
	return string.split(/(?<=\s+)(?=\w)/g).map(
		word => capitalizeFirstLetter(word)
	).join("")
}

function maintainCursorPosition(element: HTMLInputElement, callback: (cursorPos: number) => number | void) {
	const startPos = element.selectionStart
	const endPos = element.selectionEnd
	const value = element.value.length
	let offset = callback(startPos) || 0
	element.setSelectionRange(
		Math.min(element.value.length, Math.max(0, startPos + offset)),
		Math.min(element.value.length, Math.max(0, endPos + offset))
	)
}