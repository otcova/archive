import { Accessor, createSignal } from 'solid-js'
import { Show } from 'solid-js/web'
import { Expedient, ExpedientId, expedientUtils } from '../../database'
import { updateExpedient } from '../../database/expedientState'
import { OrderState } from '../../database/types'
import style from './Checkbox.module.sass'

type Props = {
	expedientId: ExpedientId,
	expedient: Expedient,
	orderIndex?: number,
}
export default function Checkbox(props: Props) {

	const state = () => Number.isInteger(props.orderIndex) ?
		props.expedient.orders[props.orderIndex].state :
		expedientUtils.globalOrderState(props.expedient)

	const changeState = (newState: OrderState) => {
		let expedient = props.expedient
		if (Number.isInteger(props.orderIndex))
			expedient.orders[props.orderIndex].state = newState
		else
			expedientUtils.setGlobalOrderState(expedient, newState)

		updateExpedient(props.expedientId, expedient)
	}
	
	let onMouseUp = event => {
		const leftButton = event.button == 0
		switch (state()) {
			case "Done": return leftButton ? changeState("Todo") : changeState("Urgent")
			case "Todo": return leftButton ? changeState("Done") : changeState("Urgent")
			case "Urgent": return leftButton ? changeState("Done") : changeState("Todo")
		}
	}

	return <>
		<Show when={state() == "Todo"}>
			<div class={style.container} onMouseUp={onMouseUp} onClick={e => e.stopPropagation()}></div>
		</Show>
		<Show when={state() == "Done"}>
			<div class={style.container} onMouseUp={onMouseUp} onClick={e => e.stopPropagation()}>{tick()}</div>
		</Show>
		<Show when={state() == "Urgent"}>
			<div class={style.star} onMouseUp={onMouseUp} onClick={e => e.stopPropagation()}>{star()}</div>
		</Show>
	</>
}

const star = () => <svg width="25" height="26" style="fill:none;stroke:#0078d4;stroke-width:1;">
	<path d="M 12.49999,0.62512493 16.19591,8.113875 24.460236,9.3147512 18.480113,15.143931 19.89183,23.374864 12.499991,19.488745 5.1081529,23.374865 6.5198677,15.143931 0.53974449,9.3147529 8.8040712,8.1138752 Z" />
</svg>

const tick = () => <svg width="10" height="12" style="fill:none;stroke:#0078d4;stroke-width:1;">
	<path d="M 0.47800627,7.9835504 4.0024026,11.006314 9.4050979,0.72675193" />
</svg>