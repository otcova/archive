import { Show } from 'solid-js/web'
import { Expedient, ExpedientId, expedientUtils } from '../../database'
import { updateExpedient } from '../../database/expedientState'
import { OrderState } from '../../database/types'
import style from './StaticCheckbox.module.sass'


type Props = {
	state: OrderState,
	onChange?: (newState: OrderState) => void,
}

export default function StaticCheckbox(props: Props) {
	const onClick = event => {
		if (!props.onChange) return
		event.stopPropagation()
	}

	const onMouseUp = event => {
		if (!props.onChange) return
		event.stopPropagation()

		const leftButton = event.button == 0
		switch (props.state) {
			case "Urgent": return leftButton ? props.onChange("Pending") : props.onChange("Done")
			case "Todo": return leftButton ? props.onChange("Pending") : props.onChange("Done")
			case "Pending": return props.onChange("Done")
			case "Done": return leftButton ? props.onChange("Todo") : props.onChange("Urgent")
		}
	}

	const pointer = () => props.onChange ? " " + style.pointer : ""

	return <>
		<Show when={props.state == "Todo"}>
			<div class={style.container + pointer()} onClick={onClick} onMouseUp={onMouseUp}></div>
		</Show>
		<Show when={props.state == "Urgent"}>
			<div class={style.star + pointer()} onClick={onClick} onMouseUp={onMouseUp}>{star()}</div>
		</Show>
		<Show when={props.state == "Pending"}>
			<div class={style.pending + pointer()} onClick={onClick} onMouseUp={onMouseUp}></div>
		</Show>
		<Show when={props.state == "Done"}>
			<div class={style.container + pointer()} onClick={onClick} onMouseUp={onMouseUp}>{tick()}</div>
		</Show>
	</>
}

const star = () => <svg width="25" height="26" style="fill:none;stroke:#0078d4;stroke-width:1;">
	<path d="M 12.49999,0.62512493 16.19591,8.113875 24.460236,9.3147512 18.480113,15.143931 19.89183,23.374864 12.499991,19.488745 5.1081529,23.374865 6.5198677,15.143931 0.53974449,9.3147529 8.8040712,8.1138752 Z" />
</svg>

const tick = () => <svg width="10" height="12" style="fill:none;stroke:#0078d4;stroke-width:1;">
	<path d="M 0.47800627,7.9835504 4.0024026,11.006314 9.4050979,0.72675193" />
</svg>