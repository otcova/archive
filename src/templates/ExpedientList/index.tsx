import { createSignal, For } from 'solid-js'
import Checkbox from '../../atoms/Checkbox'
import { Expedient, ExpedientId, expedientUtils } from '../../database'
import { createHook } from '../../database/expedientHook'
import style from './ExpedientList.module.sass'

export default function ExpedientList(props: { hookType: "all_expedients" | "all_open_expedients" }) {

	const [expedientList] = createHook(props.hookType,
		{ from: expedientUtils.futureDate(), limit: 1000 })

	return <div class={style.container}>
		<For each={expedientList()?.map(([id]) => JSON.stringify(id)) ?? []}>{(_, index) =>
			<Row expedient={expedientList()[index()][1]} expedientId={expedientList()[index()][0]} />
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