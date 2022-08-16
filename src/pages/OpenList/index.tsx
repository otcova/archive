import { createEffect, createSignal, Show } from 'solid-js'
import StaticCheckbox from '../../atoms/Checkbox/StaticCheckbox'
import { utcDateFuture } from '../../database/date'
import { createHook } from '../../database/expedientHook'
import { countExpedients, countOrders } from '../../database/expedientState'
import OrderList, { lableOrderListByDate } from '../../templates/OrderList'
import { useTab } from '../../templates/TabSystem'
import style from "./OpenList.module.sass"

export default function OpenList() {
	const { rename } = useTab()

	const [urgentList] = createHook("list_orders", {
		sort_by: "Oldest",
		from_date: utcDateFuture(),
		max_list_len: 70,
		show_done: false,
		show_todo: false,
		show_awaiting: false,
		show_instore: false,
		show_urgent: true,
	})

	const [todoList] = createHook("list_orders", {
		sort_by: "Oldest",
		from_date: utcDateFuture(),
		max_list_len: 70,
		show_done: false,
		show_todo: true,
		show_awaiting: false,
		show_instore: false,
		show_urgent: false,
	})

	// Rename tab
	createEffect(() => {
		if (urgentList() && urgentList().length != 0) {
			rename(<StaticCheckbox state={"Urgent"} />)
		} else if (todoList() && todoList().length == 0) {
			rename(<StaticCheckbox state={"Done"} />)
		} else {
			rename(<StaticCheckbox state={"Todo"} />)
		}
	})

	return <Show when={urgentList() && todoList()}>
		<Show when={urgentList()?.length || todoList()?.length} fallback={SatisfactionFallback}>
			<OrderList orderList={() =>
				[...lableOrderListByDate(urgentList()), ...lableOrderListByDate(todoList())]
			} />
		</ Show>
	</Show>
}

function SatisfactionFallback() {
	const [expedientsCount, setExpedientsCount] = createSignal<string | number>("...")
	const [ordersCount, setOrdersCount] = createSignal<string | number>("...")

	countExpedients().then(n => setExpedientsCount(n))
	countOrders().then(n => setOrdersCount(n))

	return <div class={style.satisfaction_container}>
		:D
		<div class={style.row}>
			<div class={style.left}>
				<div>Expedients:</div>
				<div>Commandas:</div>
			</div>
			<div class={style.right}>
				<div>{expedientsCount()}</div>
				<div>{ordersCount()}</div>
			</div>
		</div>
	</div>
}