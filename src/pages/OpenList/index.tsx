import { createEffect, Show } from 'solid-js'
import StaticCheckbox from '../../atoms/Checkbox/StaticCheckbox'
import { utcDateFuture } from '../../database/date'
import { createHook } from '../../database/expedientHook'
import OrderList, { lableOrderListByDate } from '../../templates/OrderList'
import { useTab } from '../../templates/TabSystem'
import style from "./OpenList.module.sass"

function SatisfactionFallback() {
	return <div class={style.satisfaction_container}>
		:D
	</div>
}

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
