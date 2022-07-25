import style from "./InputText.module.sass"

type Props = {
	placeholder?: string,
	value?: string,
	onChange?: (data: string) => void,
	autoFormat?: ("firstCapital" | "allCapital" | "spaceAfterNumber")[]
	charRegex?: RegExp,
	maxLen?: number,
	noStyle?: boolean,
}

export default function InputText(props: Props) {

	const forcePattern = event => {
		if (!event.data) return
		if (props.maxLen && event.target.value.length >= props.maxLen)
			event.preventDefault()
		if (props.charRegex && !props.charRegex.test(event.data))
			event.preventDefault()
	}

	const onInput = event => {
		const input = event.target as HTMLInputElement
		if (props.autoFormat) {
			maintainCursorPosition(input, () => {
				input.value = input.value.trimStart().replace(/\s+/g, " ")

				if (props.autoFormat.includes("firstCapital")) {
					input.value = capitalizeFirstLetter(input.value)
				} if (props.autoFormat.includes("allCapital")) {
					input.value = input.value.toUpperCase()
				} if (props.autoFormat.includes("spaceAfterNumber")) {
					input.value = input.value.replace(/(?<=\d)(?=[a-zA-Z])/g, " ")
				}
			})
		}
		props.onChange?.(input.value)
	}

	if (props.value) props.onChange?.(props.value)

	return <input
		type="text"
		value={props.value ?? ""}
		onInput={onInput}
		onBeforeInput={forcePattern}
		class={props.noStyle ? style.input_minimal : style.input}
		placeholder={props.placeholder}
		spellcheck={false}
	/>
}

function capitalizeFirstLetter(string) {
	return string.charAt(0).toUpperCase() + string.slice(1)
}

function maintainCursorPosition(element: HTMLInputElement, callback: () => void) {
	const startPos = element.selectionStart
	const endPos = element.selectionEnd
	const value = element.value.length
	callback()
	const offset = element.value.length - value
	element.setSelectionRange(startPos + offset, endPos + offset)
}