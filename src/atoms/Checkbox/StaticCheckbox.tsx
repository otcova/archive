import { Show } from 'solid-js/web'
import { OrderState } from '../../database/types'
import style from './StaticCheckbox.module.sass'


type Props = {
	state: "Awaiting" | OrderState,
	onMouseUp?: (event: MouseEvent) => void,
}

export default function StaticCheckbox(props: Props) {
	const state = () => props.state

	const onClick = event => {
		if (!props.onMouseUp) return
		event.stopPropagation()
	}

	const pointer = () => props.onMouseUp ? " " + style.pointer : ""

	return <>
		<Show when={state() == "Todo"}>
			<div class={style.container + pointer()} onClick={onClick} onMouseUp={props.onMouseUp}>
			</div>
		</Show>
		<Show when={state() == "Urgent"}>
			<div class={style.star + pointer()} onClick={onClick} onMouseUp={props.onMouseUp}>
				{star()}
			</div>
		</Show>
		<Show when={state() == "Awaiting"}>
			<div class={style.awaiting + pointer()} onClick={onClick} onMouseUp={props.onMouseUp}>
				<div class={style.dot}></div>
			</div>
		</Show>
		<Show when={state() == "InStore"}>
			<div class={style.pending + pointer()} onClick={onClick} onMouseUp={props.onMouseUp}>
			</div>
		</Show>
		<Show when={state() == "Done"}>
			<div class={style.container + pointer()} onClick={onClick} onMouseUp={props.onMouseUp}>
				{tick()}
			</div>
		</Show>
	</>
}

const star = () => <svg width="25" height="26" style="fill:none;stroke:#0078d4;stroke-width:1;">
	<path d="M 12.49999,0.62512493 16.19591,8.113875 24.460236,9.3147512 18.480113,15.143931 19.89183,23.374864 12.499991,19.488745 5.1081529,23.374865 6.5198677,15.143931 0.53974449,9.3147529 8.8040712,8.1138752 Z" />
</svg>

const tick = () => <svg width="10" height="12" style="fill:none;stroke:#0078d4;stroke-width:1;">
	<path d="M 0.47800627,7.9835504 4.0024026,11.006314 9.4050979,0.72675193" />
</svg>