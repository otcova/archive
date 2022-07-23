import OrderList, { lableByDate } from '../../templates/OrderList'
import { createHook } from '../../database/expedientHook'

export default function DoneList() {
	
	const [orderList] = createHook("list_oreders", {
		sort_by: "Newest",
		from_date: 10000000,
		max_list_len: 70,
		show_done: true,
		show_todo: false,
		show_pending: false,
		show_urgent: false,
	})
	
	return <OrderList orderList={() => [...lableByDate(orderList())]} />
}