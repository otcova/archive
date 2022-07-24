import OrderList, { lableByDate } from '../../templates/OrderList'
import { createHook } from '../../database/expedientHook'
import { utcDateNow } from '../../database/types'

export default function PendingList() {

	const [orderList] = createHook("list_orders", {
		sort_by: "Oldest",
		from_date: utcDateNow(),
		max_list_len: 70,
		show_done: false,
		show_todo: false,
		show_pending: true,
		show_urgent: false,
	})

	return <OrderList orderList={() => [...lableByDate(orderList())]} />
}