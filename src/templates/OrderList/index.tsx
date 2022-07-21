import { createEffect, createSignal, For, JSX as SolidjsJSX } from 'solid-js'
import Checkbox from '../../atoms/Checkbox'
import { Expedient, ExpedientId, expedientUtils } from '../../database'
import { createHook } from '../../database/expedientHook'
import ExpedientEditor from '../../pages/ExpedientEditor'
import { useTab } from '../TabSystem'
import style from './OrderList.module.sass'

export default function OrderList() {

	const [expedientList] = createHook("list_oreders", {
		sort_by: "Newest",
		from_date: 10000000,
		max_list_len: 70,
		show_done: true,
		show_todo: true,
		show_pending: true,
		show_urgent: true,
	})

	return <div class={style.container}>
		<For each={expedientList()?.map(([id, index, exp]) => JSON.stringify([id, index, exp])) ?? []}>{(data, _) => {
			return <Row data={JSON.parse(data as string)} />
		}}</For>
	</div>
}

function Row(props: { data: [ExpedientId, number, Expedient] }) {
	const [expedientId, orderIndex, expedient] = props.data
	const order = expedient.orders[orderIndex]
	const { createTab } = useTab()
	
	const openOrder = () => {
		createTab("", ExpedientEditor, { expedientId, orderIndex })
	}

	return <div class={style.row_container} onClick={openOrder}>
		<Checkbox expedient={expedient} expedientId={expedientId} orderIndex={orderIndex} />
		<div class={style.grow}>{order.title}</div>
		<div class={style.grow}>{expedientUtils.strUsers(expedient)}</div>
		<div class={style.grow}>{expedient.model}</div>
		<div>{expedient.license_plate}</div>
	</div>
}