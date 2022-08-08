import { createEffect, createSignal, onMount } from 'solid-js'
import InputText from '../../atoms/InputText'
import { utcDateFuture } from '../../database/date'
import { createHook } from '../../database/expedientHook'
import OrderList, { lableOrderListByDate } from '../../templates/OrderList'
import { bindKey } from '../../utils/bindKey'
import style from "./DoneList.module.sass"

export default function DoneList() {

	const [inputVIN, setInputVin] = createSignal<string>("")
	const [inputUser, setInputUser] = createSignal<string>("")
	const [inputBody, setInputBody] = createSignal<string>("")

	const [orderList, setHookOptions] = createHook("list_orders", {
		sort_by: "Newest",
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

	let top_row
	onMount(() =>
		bindKey(top_row, "Escape", () => {
			if (!inputUser() && !inputBody() && !inputVIN())
				return "propagate"
			setInputUser("")
			setInputBody("")
			setInputVin("")
		})
	)

	return <>
		<div class={style.input_row} ref={top_row}>
			<div class={style.input_user}>
				<InputText placeholder='Usuari' value={inputUser()} onChange={setInputUser} />
			</div>
			<div class={style.input_body}>
				<InputText placeholder='Cos' value={inputBody()} onChange={setInputBody} />
			</div>
			<div class={style.input_vin}>
				<InputText
					placeholder='Matricula / VIN'
					autoFormat={['spaceAfterNumber', 'allCapital']}
					value={inputVIN()}
					onChange={setInputVin}
				/>
			</div>
		</div>
		<OrderList orderList={() => [...lableOrderListByDate(orderList())]} />
	</>
}