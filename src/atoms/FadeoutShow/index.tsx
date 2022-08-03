import { createEffect, createSignal, JSXElement, Show } from "solid-js"
import style from "./FadeoutShow.module.sass"

type Props = {
	when: any
	children: JSXElement
}

export function FadeoutShow(props: Props) {
	const [render, setRender] = createSignal(false)
	const [show, setShow] = createSignal(props.when)

	let disableTimeout = 0

	createEffect(() => {
		clearTimeout(disableTimeout)
		if (props.when) {
			setRender(true)
			setTimeout(() => setShow(true))
		}
		else {
			setShow(false)
			disableTimeout = setTimeout(() => setRender(false), 1000)
		}
	})

	return <Show when={render()}>
		<div class={show() ? style.show : style.hide}>
			{props.children}
		</div>
	</Show>
}