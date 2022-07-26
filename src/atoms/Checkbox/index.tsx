import { createEffect, createSignal } from 'solid-js'
import { updateExpedient } from '../../database/expedientState'
import { compareDate as compareUtcDate, Expedient, ExpedientId, jsDateToUtc, OrderState, UtcDate, utcDateNow, utcToJsDate } from '../../database/types'
import { AwaitDateSelectorPanel } from '../../templates/AwaitDateSelectorPanel'
import StaticCheckbox from './StaticCheckbox'

type Props = {
	expedientId: ExpedientId,
	expedient: Expedient,
	orderIndex: number,
}

export default function Checkbox(props: Props) {
	const [showAwaitDatePanel, setShowAwaitDateSelectorPanel] = createSignal(false)

	const order = () => props.expedient.orders[props.orderIndex]

	const changeState = (newState: OrderState, date = utcDateNow()) => {
		let expedient = props.expedient
		expedient.orders[props.orderIndex].state = newState
		expedient.orders[props.orderIndex].date = date
		updateExpedient(props.expedientId, expedient)
	}

	const defaultAwaitDate = () => {
		let date = new Date()
		if (date.getHours() <= 12 && date.getMinutes() <= 46) {
			date.setHours(15)
		} else if (date.getHours() <= 18 && date.getMinutes() <= 46) {
			date.setHours(date.getHours() + 24)
			date.setHours(8)
		} else {
			date.setHours(date.getHours() + 24)
			date.setHours(15)
		}
		return jsDateToUtc(date)
	}

	const onMouseUp = (event: MouseEvent) => {
		event.stopPropagation()

		if (event.shiftKey && event.button == 0) {
			// Left + shift button
			setShowAwaitDateSelectorPanel(true)
		} else if (event.button == 0) {
			// Left button
			switch (order().state) {
				case "Urgent": return changeState("Awaiting", defaultAwaitDate())
				case "Todo": return changeState("Awaiting", defaultAwaitDate())
				case "Awaiting": return changeState("InStore")
				case "InStore": return changeState("Done")
				case "Done": return changeState("Todo")
			}
		} else if (event.button == 2) {
			// Right button
			switch (order().state) {
				case "Urgent": return changeState("Done")
				case "Todo": return changeState("Done")
				case "Awaiting": return changeState("Todo")
				case "InStore": return changeState("Todo")
				case "Done": return changeState("Awaiting", defaultAwaitDate())
			}
		} else {
			// Midle button
			switch (order().state) {
				case "Urgent": return changeState("Todo")
				default: return changeState("Urgent")
			}
		}
	}

	const awaitPanelResponse = (utcDate: UtcDate) => {
		setShowAwaitDateSelectorPanel(false)
		if (utcDate) changeState("Awaiting", utcDate)
	}

	// Auto change from Awaiting to InStore
	createEffect(() => {
		if (order().state == "Awaiting") {
			if (compareUtcDate(order().date, utcDateNow()) <= 0) {
				changeState("InStore")
			}
		}
	})

	return <>
		<StaticCheckbox state={order().state} onMouseUp={onMouseUp} />
		<AwaitDateSelectorPanel show={showAwaitDatePanel()}
			date={order().state == "Awaiting" ? order().date : defaultAwaitDate()}
			response={awaitPanelResponse}
		/>
	</>
}