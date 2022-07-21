import { createEffect } from 'solid-js'
import { Show } from 'solid-js/web'
import { Expedient, ExpedientId, expedientUtils } from '../../database'
import { updateExpedient } from '../../database/expedientState'
import { OrderState } from '../../database/types'
import StaticCheckbox from './StaticCheckbox'

type Props = {
	expedientId: ExpedientId,
	expedient: Expedient,
	orderIndex?: number,
}
export default function Checkbox(props: Props) {
	if (props.t) {
		createEffect(() => console.log(props.expedient.orders[props.orderIndex].state))
	}
	
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

	return <StaticCheckbox state={state()} onChange={newState => changeState(newState)} />
}