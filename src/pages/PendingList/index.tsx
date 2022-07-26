import OrderList, { lableByDate } from '../../templates/OrderList'
import { createHook } from '../../database/expedientHook'
import { utcDateFuture, utcDateNow } from '../../database/types'
import { createEffect } from 'solid-js'
import { useTab } from '../../templates/TabSystem'
import StaticCheckbox from '../../atoms/Checkbox/StaticCheckbox'

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
		from_date: utcDateNow(),
		max_list_len: 70,
		show_done: false,
		show_todo: false,
		show_awaiting: false,
		show_instore: true,
		show_urgent: false,
	})

	// Rename tab
	createEffect(() => {
		if (awaitingList() && awaitingList().length != 0) {
			rename(<StaticCheckbox state={"Awaiting"} />)
		} else if (instoreList() && instoreList().length == 0) {
			rename(<StaticCheckbox state={"Done"} />)
		} else {
			rename(<StaticCheckbox state={"InStore"} />)
		}
	})

	return <OrderList orderList={() => [...lableByDate(instoreList()), ...lableByDate(awaitingList())]} />
}