import OrderList, { lableByDate } from '../../templates/OrderList'
import { createHook } from '../../database/expedientHook'
import { utcDateNow } from '../../database/types'
import { createEffect } from 'solid-js'
import { useTab } from '../../templates/TabSystem'
import StaticCheckbox from '../../atoms/Checkbox/StaticCheckbox'

export default function OpenList() {
	const { rename } = useTab()

	const [urgentList] = createHook("list_orders", {
		sort_by: "Oldest",
		from_date: utcDateNow(),
		max_list_len: 70,
		show_done: false,
		show_todo: false,
		show_pending: false,
		show_urgent: true,
	})

	const [todoList] = createHook("list_orders", {
		sort_by: "Oldest",
		from_date: utcDateNow(),
		max_list_len: 70,
		show_done: false,
		show_todo: true,
		show_pending: false,
		show_urgent: false,
	})

	// Rename tab
	createEffect(() => {
		if (urgentList() && urgentList().length == 0) {
			console.log("Rename D")
			rename(<StaticCheckbox state={"Todo"} />)
		} else {
			console.log("Rename Ur")
			rename(<StaticCheckbox state={"Urgent"} />)
		}
	})

	return <OrderList orderList={() =>
		[...lableByDate(urgentList()), ...lableByDate(todoList())]
	} />
}