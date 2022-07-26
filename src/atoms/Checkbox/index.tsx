import { updateExpedient } from '../../database/expedientState'
import { Expedient, ExpedientId, expedientUtils, OrderState, utcDateNow } from '../../database/types'
import StaticCheckbox from './StaticCheckbox'

type Props = {
	expedientId: ExpedientId,
	expedient: Expedient,
	orderIndex: number,
}

export default function Checkbox(props: Props) {
	const state = () => Number.isInteger(props.orderIndex) ?
		props.expedient.orders[props.orderIndex].state :
		expedientUtils.globalOrderState(props.expedient)

	const changeState = (newState: OrderState) => {
		let expedient = props.expedient
		expedient.orders[props.orderIndex].state = newState
		expedient.orders[props.orderIndex].date = utcDateNow()
		updateExpedient(props.expedientId, expedient)
	}

	return <StaticCheckbox state={state()} onChange={newState => changeState(newState)} />
}