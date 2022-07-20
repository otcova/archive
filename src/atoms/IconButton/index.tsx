import { JSXElement, onCleanup, onMount } from "solid-js"
import style from "./IconButton.module.sass"

type IconType = "folder" | "document" | "image" | "pdf" | "close" | "minimize"

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

icons.set("folder", () => <svg width="27" height="24" style="fill:#fff;stroke:#5c5c5c;stroke-width:1;">
	<path d="M 0.5,0.5 H 11.5 L 14.5,5.5 H 26.5 L 26.5,23.5 0.5,23.5 Z" />
</svg>);

icons.set("pdf", () => <svg width="18" height="24" style="fill:#fff;stroke:#5c5c5c;stroke-width:1;">
	<path d="m 0.578,0.625 h 11.000131 l 5.843569,5.844 3e-4,16.906 H 0.578 Z" />
	<path d="M 11.577631,0.625 V 6.469 H 17.4217" style="fill:none" />
	<path d="m 6.8867055,16.162177 v 4.234401 H 5.5741973 V 9.1933834 h 3.0781441 q 1.7968866,0 2.7812676,0.8750056 0.992193,0.875005 0.992193,2.468765 0,1.59376 -1.101569,2.609391 -1.093757,1.015632 -2.9609559,1.015632 z m 0,-5.781286 v 4.593778 H 8.261714 q 1.3593834,0 2.070325,-0.617191 0.718755,-0.625004 0.718755,-1.757823 0,-2.218764 -2.6250165,-2.218764 z" style="fill:#5c5c5c;stroke:none;"/>
</svg>);

icons.set("document", () => <svg width="18" height="24" style="fill:#fff;stroke:#5c5c5c;stroke-width:1;">
	<path d="m 0.578,0.625 h 11.000131 l 5.843569,5.844 3e-4,16.906 H 0.578 Z" />
	<path d="M 11.577631,0.625 V 6.469 H 17.4217" style="fill:none" />
</svg>);

icons.set("image", () => <svg width="27" height="23" style="fill:#fff;stroke:#5c5c5c;stroke-width:1;">
	<path d="M 26.544129,17.82665 20.427951,11.764526 16.513586,15.644288 9.2631875,8.4579162 0.49992417,17.143283 v 5.356721 H 26.499641 V 0.5 H 0.49992417 v 16.621246"/>
	<path d="M 20.543204,6.32899 A 2.2812591,2.28125 0 0 1 18.261895,8.61024 2.2812591,2.28125 0 0 1 15.980686,6.32899 2.2812591,2.28125 0 0 1 18.261895,4.04774 2.2812591,2.28125 0 0 1 20.543204,6.32899 Z" />
</svg>);

icons.set("close", () => <div class={style.close}></div>);
icons.set("minimize", () => <div class={style.minimize}></div>);