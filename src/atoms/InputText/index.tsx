import { createEffect, createSignal, For, on, onMount } from "solid-js"
import { bindKey } from "../../utils/bindKey"
import { FadeoutShow } from "../FadeoutShow"
import style from "./InputText.module.sass"

type Props = {
	placeholder?: string,
	value?: string,
	onChange?: (data: string) => void,
	autoFormat?: ("startWordCapital" | "firstCapital" | "allCapital"
		| "spaceAfterNumber" | "confusingLettersToNumbers")[]
	charRegex?: RegExp,
	maxLen?: number,
	noStyle?: boolean,
	validate?: (text: string) => boolean,
	selectOnFocus?: boolean,
	suggestions?: string[]
	escape?: "clear",
}

export default function InputText(props: Props) {
	const [showSuggestions, _setShowSuggestions] = createSignal(false)
	const setShowSuggestions = v => {
		_setShowSuggestions(v)
	}

	const forcePattern = event => {
		if (!event.data) return
		if (props.charRegex && !props.charRegex.test(event.data))
			event.preventDefault()
	}

	const format = () => {
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
			} if (props.autoFormat.includes("confusingLettersToNumbers")) {
				input.value = input.value
					.replaceAll(/q/ig, "9")
					.replaceAll(/o/ig, "0")
					.replaceAll(/i/ig, "1")
			}
			return input.value.length - initialValueLength
		})
	}

	let input: HTMLInputElement
	const onInput = () => {
		if (props.maxLen) {
			maintainCursorPosition(input, () => {
				input.value = input.value.slice(0, props.maxLen)
			})
		}
		if (props.autoFormat) format()
		props.onChange?.(input.value)
		validate()
		setShowSuggestions(input.value.length > 0)
	}

	const onFocus = event => {
		setShowSuggestions(input.value.length > 0)
		if (props.selectOnFocus) event.target.select()
	}


	const onMouseDown = (event: MouseEvent) => {
		if (event.button == 2) event.preventDefault()
	}

	const suggestions = () =>
		(props.suggestions ?? []).filter(user => input.value != user)

	onMount(() => bindKey(input, "Enter", () => {
		if (suggestions().length) {
			input.value = suggestions()[0]
			onInput()
			input.blur()
		} else {
			return "propagate"
		}
	}))

	const validate = () => {
		input.classList.remove(style.error)
		if (props.validate && !props.validate(input.value.trim())) {
			input.classList.add(style.error)
		}
	}

	if (props.value) {
		props.onChange?.(props.value)
		createEffect(on(() => props.value, validate))
	}

	// Initial format
	onMount(() => {
		if (props.autoFormat) {
			let before = input.value
			format()
			if (before != input.value) props.onChange?.(input.value)
		}
	})

	const onKeyDown = (event: KeyboardEvent) => {
		if (event.key == "Escape") {
			if (showSuggestions()) {
				setShowSuggestions(false)
				event.preventDefault()
				event.stopImmediatePropagation()
				event.stopPropagation()
			}
			if (props?.escape == "clear" && input.value != "") {
				input.value = ""
				props.onChange?.(input.value)
			}
		}
	}

	// Input have to be inside a div to be detected when window.getSelection() on ctrl+z
	return <div class={style.container} onKeyDown={onKeyDown}>
		<input
			type="text"
			ref={input}
			onMouseDown={onMouseDown}

			value={props.value ?? ""}
			onInput={onInput}
			onFocus={onFocus}
			onBlur={() => setShowSuggestions(false)}
			onBeforeInput={forcePattern}
			class={props.noStyle ? style.input_minimal : style.input}
			placeholder={props.placeholder}
			spellcheck={false}
		/>
		<FadeoutShow when={showSuggestions() && suggestions().length} >
			<div class={props.noStyle ? style.dropbox_container_minimal : style.dropbox_container}>
				<div class={style.dropbox}>
					<For each={suggestions()}>{(suggestion) =>
						<div class={style.row}
							onMouseDown={event => event.preventDefault()}
							onMouseUp={() => {
								input.value = suggestion
								onInput()
							}}>{suggestion}</div>
					}</For>
				</div>
			</div>
		</FadeoutShow>
	</div>
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