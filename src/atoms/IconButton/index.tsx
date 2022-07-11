import { JSXElement, onCleanup, onMount } from "solid-js"
import style from "./IconButton.module.sass"

type IconType = "folder" | "file"

type Props = {
	icon: IconType,
	shortCut?: string, // examples: ctrl a, alt r, ctrl alt n
	style?: number,
	action?: () => any,
}

export default function IconButton(props: Props) {

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
		{icons.get(props.icon)}
	</div>
}

const icons = new Map<IconType, JSXElement>()
icons.set("folder", <div>Folder</div>);
icons.set("file", <div>File</div>);