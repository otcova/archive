import { createEffect, createSignal, JSXElement, Show } from "solid-js"
import style from "./FadeoutShow.module.sass"

type Props = {
	when: any
	children: JSXElement
}

export function FadeoutShow(props: Props) {
	const [render, setRender] = createSignal(false)
	const [show, setShow] = createSignal(props.when)

	let disableRenderTimeout = 0
	let disableShowTimeout = 0

	createEffect(() => {
		clearTimeout(disableRenderTimeout)
		clearTimeout(disableShowTimeout)
		if (props.when) {
			setRender(true)
			disableShowTimeout = setTimeout(() => setShow(true))
		}
		else {
			setShow(false)
			disableRenderTimeout = setTimeout(() => setRender(false), 100)
		}
	})

	return <Show when={render()}>
		<div class={show() ? style.show : style.hide}>
			{props.children}
		</div>
	</Show>
}