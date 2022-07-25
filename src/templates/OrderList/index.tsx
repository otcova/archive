import { Accessor, createEffect, createSignal, For, JSX as SolidjsJSX } from 'solid-js'
import Checkbox from '../../atoms/Checkbox'
import { createHook } from '../../database/expedientHook'
import { equalDay, Expedient, ExpedientId, expedientUtils, UtcDate, utcDateToString } from '../../database/types'
import ExpedientEditor from '../../pages/ExpedientEditor'
import { useTab } from '../TabSystem'
import style from './OrderList.module.sass'

type Props = {
	orderList: Accessor<([ExpedientId, number, Expedient] | string)[]>
}

export default function OrderList(props: Props) {
	return <div class={style.container}>
		<For each={props.orderList()?.map(data => JSON.stringify(data))}>{(data, _) => {
			const order = JSON.parse(data)
			if (typeof order == "string") return <Lable text={order} />
			return <Row data={order} />
		}}</For>
	</div>
}

function Row(props: { data: [ExpedientId, number, Expedient] }) {
	const [expedientId, orderIndex, expedient] = props.data
	const order = expedient.orders[orderIndex]
	const { createTab } = useTab()

	const openOrder = () => {
		createTab("", ExpedientEditor, { expedientId })
	}

	return <div class={style.row_container} onClick={openOrder}>
		<Checkbox expedient={expedient} expedientId={expedientId} orderIndex={orderIndex} />
		<div class={style.grow}>{order.title}</div>
		<div class={style.grow}>{expedient.user}</div>
		<div class={style.grow}>{expedient.model}</div>
		<div>{expedient.license_plate}</div>
	</div>
}

function Lable({ text }: { text: string }) {
	return <div class={style.lable}>{text}</div>
}

export function lableByDate(list?: [ExpedientId, number, Expedient][]): ([ExpedientId, number, Expedient] | string)[] {
	if (!list) return []
	let pastDate = {} as UtcDate
	let labledList = []

	for (const data of list) {
		const order = data[2].orders[data[1]]
		if (!equalDay(order.date, pastDate)) {
			pastDate = order.date
			labledList.push(utcDateToString(pastDate))
		}

		labledList.push(data)
	}

	return labledList
}