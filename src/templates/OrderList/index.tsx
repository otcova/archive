import { Accessor, createEffect, For } from 'solid-js'
import { UtcDate, utcDateToString } from '../../database/date'
import { readExpedient, updateExpedient } from '../../database/expedientState'
import { Expedient, ExpedientId, OrderState } from '../../database/types'
import { bindKey } from '../../utils/bindKey'
import { useTab } from '../TabSystem'
import style from './OrderList.module.sass'
import { Row, RowLable } from './row'

type Props = {
	orderList: Accessor<([ExpedientId, number, Expedient] | string)[]>
}

export default function OrderList(props: Props) {

	trackStateChanges(props.orderList)

	return <div class={style.container}>
		<For each={props.orderList()?.map(data => JSON.stringify(data))}>{(data, _) => {
			const order = JSON.parse(data)
			if (typeof order == "string") return <RowLable text={order} />
			return <Row data={order} />
		}}</For>
	</div>
}



export function lableOrderListByDate(list?: [ExpedientId, number, Expedient][]): ([ExpedientId, number, Expedient] | string)[] {
	if (!list) return []
	let currentLable = ""
	let labledList = []

	for (const data of list) {
		const order = data[2].orders[data[1]]
		const thisLable = utcDateToString(order.date)
		if (currentLable != thisLable) {
			currentLable = thisLable
			labledList.push(thisLable)
		}

		labledList.push(data)
	}

	return labledList
}

function trackStateChanges(list: () => ([ExpedientId, number, Expedient] | string)[]) {
	const { isActive } = useTab()

	let pastState: [ExpedientId, number, OrderState, UtcDate][] | null = null
	let currentState: [ExpedientId, number, OrderState, UtcDate][] | null = null
	let pastEffectState: string | null = null
	let lastUpdateTimeout = null

	const getState = () => {
		const orderList = list().filter(
			row => typeof row != "string"
		) as [ExpedientId, number, Expedient][]

		return JSON.parse(JSON.stringify(
			orderList.map(([expedientId, orderIndex, expedient]) => {
				const order = expedient.orders[orderIndex]
				return [expedientId, orderIndex, order.state, order.date]
			})
		)) as [ExpedientId, number, OrderState, UtcDate][]
	}

	createEffect(() => {
		if (!list() || !list().length) return
		if (!isActive()) {
			pastState = null
			currentState = null
			pastEffectState = null
			lastUpdateTimeout = null
			return
		}

		const effectState = getState()
		const strEffectState = JSON.stringify(effectState)
		if (strEffectState == pastEffectState) return
		pastEffectState = strEffectState

		if (lastUpdateTimeout === null) {
			pastState = currentState ?? effectState
		} else {
			clearTimeout(lastUpdateTimeout)
		}

		currentState = effectState
		lastUpdateTimeout = setTimeout(() => {
			lastUpdateTimeout = null
		}, 2000)
	})

	bindKey(document, "CTRL Z", async () => {
		if (!isActive()) return

		const state = JSON.parse(JSON.stringify(pastState)) ?? []

		for (const [expedientId, orderIndex, orderState, orderDate] of state) {
			const expedient = await readExpedient(expedientId)
			if (!expedient) return
			const order = expedient.orders[orderIndex]
			if (order.state != orderState || order.date.timespan != orderDate.timespan) {
				order.state = orderState
				order.date = orderDate
				updateExpedient(expedientId, expedient)
			}
		}
	})
}