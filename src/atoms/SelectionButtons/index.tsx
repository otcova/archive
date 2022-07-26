import { createEffect, createSignal, Index, onCleanup, onMount } from "solid-js"
import style from "./SelectionButtons.module.sass"

type Props = {
	buttons: string[],
	default?: number,
	onSelect?: (selected: string, index: number) => any,
}

export default function SelectionButtons(props: Props) {
	const [selected, setSelected] = createSignal(props.default ?? 0)

	createEffect(() => {
		if (!props.onSelect) return
		props.onSelect(props.buttons[selected()], selected())
	})

	return <div class={style.container}>
		<Index each={props.buttons}>{(text, index) =>
			<div class={index == selected() ? style.selected_item : style.item}
				onClick={() => setSelected(index)}>
				{text()}
			</div>
		}</Index>
	</div>
}