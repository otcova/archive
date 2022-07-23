import OrderList, { lableByDate } from '../../templates/OrderList'
import { createHook } from '../../database/expedientHook'

export default function OpenList() {

	const [urgentList] = createHook("list_oreders", {
		sort_by: "Oldest",
		from_date: 10000000,
		max_list_len: 70,
		show_done: false,
		show_todo: false,
		show_pending: false,
		show_urgent: true,
	})

	const [todoList] = createHook("list_oreders", {
		sort_by: "Oldest",
		from_date: 10000000,
		max_list_len: 70,
		show_done: false,
		show_todo: true,
		show_pending: false,
		show_urgent: false,
	})

	return <OrderList orderList={() =>
		[...lableByDate(urgentList()), ...lableByDate(todoList())]
	} />
}