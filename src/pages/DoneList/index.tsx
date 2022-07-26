import OrderList, { lableByDate } from '../../templates/OrderList'
import { createHook } from '../../database/expedientHook'
import style from "./DoneList.module.sass"
import InputText from '../../atoms/InputText'
import { createEffect, createSignal } from 'solid-js'
import { utcDateNow } from '../../database/types'

export default function DoneList() {

	const [inputVIN, setInputVin] = createSignal<string>("")
	const [inputUser, setInputUser] = createSignal<string>("")
	const [inputBody, setInputBody] = createSignal<string>("")

	const [orderList, setHookOptions] = createHook("list_orders", {
		sort_by: "Oldest",
		from_date: utcDateNow(),
		max_list_len: 70,
		show_done: true,
		show_todo: false,
		show_awaiting: false,
		show_instore: false,
		show_urgent: false,
	})

	createEffect(() => {
		const filter = inputVIN() + inputUser() + inputBody() != ""
		if (!filter) {
			setHookOptions(options => {
				delete options.filter
				return { ...options }
			})
		} else {
			setHookOptions(options => {
				return {
					...options, filter: {
						car_code: inputVIN(),
						user: inputUser(),
						body: inputBody(),
					}
				}
			})
		}
	})

	return <>
		<div class={style.input_row}>
			<div class={style.input_user}>
				<InputText placeholder='Usuari' onChange={setInputUser} />
			</div>
			<div class={style.input_body}>
				<InputText placeholder='Cos' onChange={setInputBody} />
			</div>
			<div class={style.input_vin}>
				<InputText placeholder='Matricula / VIN' onChange={setInputVin} />
			</div>
		</div>
		<OrderList orderList={() => [...lableByDate(orderList())]} />
	</>
}