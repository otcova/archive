import style from "./InputText.module.sass"

type Props = {
	placeholder?: string,
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

	return <input
		type="text"
		onBeforeInput={forcePattern}
		class={props.noStyle? style.input_minimal : style.input}
		placeholder={props.placeholder}
		spellcheck={false}
	/>
}

