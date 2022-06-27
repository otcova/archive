import { onCleanup, onMount } from "solid-js"
import style from "./Button.module.sass"

type Props = {
	text: string,
	shortCut?: string, // examples: ctrl a, alt r, ctrl alt n
	style?: number,
	action?: () => any,
}

export default function Button(props: Props) {

	const onKeydown = event => {
		if (!props.shortCut) return
		let cmd = props.shortCut.toLocaleUpperCase().trim()
		let cmdKey = cmd.split(/ +/g).pop();
		if (cmd.includes("CTRL") && !event.ctrlKey) return
		if (cmd.includes("ALT") && !event.altKey) return
		if (event.code == "Key" + cmdKey) props.action()
	}

	onMount(() => addEventListener("keydown", onKeydown))
	onCleanup(() => removeEventListener("keydown", onKeydown))

	return <div class={style.button} onClick={props.action}>
		{props.text}
	</div>
}