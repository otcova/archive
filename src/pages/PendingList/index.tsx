import { createEffect } from 'solid-js'
import StaticCheckbox from '../../atoms/Checkbox/StaticCheckbox'
import { utcDateFuture } from '../../database/date'
import { createHook } from '../../database/expedientHook'
import OrderList, { lableOrderListByDate } from '../../templates/OrderList'
import { useTab } from '../../templates/TabSystem'

export default function PendingList() {

	const { rename } = useTab()

	const [awaitingList] = createHook("list_orders", {
		sort_by: "Oldest",
		from_date: utcDateFuture(),
		max_list_len: 70,
		show_done: false,
		show_todo: false,
		show_awaiting: true,
		show_instore: false,
		show_urgent: false,
	})

	const [instoreList] = createHook("list_orders", {
		sort_by: "Oldest",
		from_date: utcDateFuture(),
		max_list_len: 70,
		show_done: false,
		show_todo: false,
		show_awaiting: false,
		show_instore: true,
		show_urgent: false,
	})

	// Rename tab
	createEffect(() => {
		if (instoreList() != null && instoreList().length != 0) {
			rename(<StaticCheckbox state={"InStore"} />)
		} else if (awaitingList() != null && awaitingList().length != 0) {
			rename(<StaticCheckbox state={"Awaiting"} />)
		} else {
			rename(<StaticCheckbox state={"Done"} />)
		}
	})

	return <OrderList orderList={() => [...lableOrderListByDate(instoreList()), ...lableOrderListByDate(awaitingList())]} />
}