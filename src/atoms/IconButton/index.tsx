import { JSXElement, onCleanup, onMount } from "solid-js"
import style from "./IconButton.module.sass"

type IconType = "folder" | "file" | "close" | "minimize"

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

	return <div class={style.container} onClick={props.action}>
		{icons.get(props.icon)()}
	</div>
}

const icons = new Map<IconType, () => JSXElement>()
icons.set("folder", () => <div class={style.button}>Folder</div>);
icons.set("file", () => <div class={style.button}>File</div>);
icons.set("close", () => <div class={style.close}></div>);
icons.set("minimize", () => <div class={style.minimize}></div>);