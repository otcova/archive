import { createEffect, createSignal } from 'solid-js'
import InputText from '../../atoms/InputText'
import { utcDateFuture } from '../../database/date'
import { createHook } from '../../database/expedientHook'
import OrderList, { lableOrderListByDate } from '../../templates/OrderList'
import style from "./DoneList.module.sass"

export default function DoneList() {

	const [inputVIN, setInputVin] = createSignal<string>("")
	const [inputUser, setInputUser] = createSignal<string>("")
	const [inputBody, setInputBody] = createSignal<string>("")

	const [orderList, setHookOptions] = createHook("list_orders", {
		sort_by: "Oldest",
		from_date: utcDateFuture(),
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
						car_code: inputVIN().replaceAll(" ", "_"),
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
				<InputText
					placeholder='Matricula / VIN'
					autoFormat={['spaceAfterNumber', 'allCapital']}
					onChange={setInputVin}
				/>
			</div>
		</div>
		<OrderList orderList={() => [...lableOrderListByDate(orderList())]} />
	</>
}