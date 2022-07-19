import { createEffect, createSignal, For } from 'solid-js'
import Checkbox from '../../atoms/Checkbox'
import { Expedient, ExpedientId, expedientUtils } from '../../database'
import { createHook } from '../../database/expedientHook'
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
	createEffect(() => console.log(expedientList()))
	return <div class={style.container}>
		<For each={expedientList()?.map(([id, index]) => JSON.stringify([id, index])) ?? []}>{(_, index) =>
			<Row expedient={expedientList()[index()][2]} expedientId={expedientList()[index()][0]} />
		}</For>
	</div>
}

function Row(props: { expedient: Expedient, expedientId: ExpedientId }) {
	const [showDescription, setShowDescription] = createSignal(false)
	const [height, setHeight] = createSignal(null)
	let content = null

	const toggleDescription = () => {
		setHeight(content.offsetHeight)
		setShowDescription(d => !d)
		setHeight(content.offsetHeight)
	}

	return <div class={style.expedient_box} onClick={toggleDescription} style={{ height: height() + "px" }}>
		<div class={style.expedient_content} ref={content}>
			<div class={style.title_row}>
				<Checkbox expedient={props.expedient} expedientId={props.expedientId} />
				<div class={style.orders_count}>{props.expedient.orders.length}</div>
				<div class={style.grow}>{expedientUtils.strUsers(props.expedient)}</div>
				<div class={style.grow}>{props.expedient.model}</div>
			</div>
			{
				showDescription() &&
				<For each={props.expedient.orders} >{(order, index) =>
					<div class={style.order_row}>
						<Checkbox expedient={props.expedient} expedientId={props.expedientId} orderIndex={index()} />
						<div class={style.order_date}>{expedientUtils.strDate(order.date)}</div>
						<div class={style.grow}>{order.title}</div>
					</div>
				}</For>
			}
		</div>
	</div>
}