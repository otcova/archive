import OrderList, { lableByDate } from '../../templates/OrderList'
import { createHook } from '../../database/expedientHook'
import style from "./DoneList.module.sass"
import InputText from '../../atoms/InputText'
import { createEffect, createSignal } from 'solid-js'

export default function DoneList() {

	const [inputVIN, setInputVin] = createSignal<string>("")
	const [inputUser, setInputUser] = createSignal<string>("")
	const [inputBody, setInputBody] = createSignal<string>("")

	const [orderList, setHookOptions] = createHook("list_orders", {
		sort_by: "Newest",
		from_date: 10000000,
		max_list_len: 70,
		show_done: true,
		show_todo: false,
		show_pending: false,
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
			<InputText placeholder='Usuari' onChange={setInputUser} />
			<InputText placeholder='Cos' onChange={setInputBody} />
			<div class={style.input_vin}>
				<InputText placeholder='Matricula / VIN' onChange={setInputVin} />
			</div>
		</div>
		<OrderList orderList={() => [...lableByDate(orderList())]} />
	</>
}