import { createEffect, createSignal, onMount } from 'solid-js'
import IconButton from '../../atoms/IconButton'
import InputText from '../../atoms/InputText'
import { utcDateFuture } from '../../database/date'
import { createHook } from '../../database/expedientHook'
import { createExpedient } from '../../database/expedientState'
import { newBlankExpedient } from '../../database/types'
import { verifyVIN } from '../../database/vin/verify'
import OrderList, { lableOrderListByDate } from '../../templates/OrderList'
import { useTab } from '../../templates/TabSystem'
import { bindKey } from '../../utils/bindKey'
import ExpedientEditor from '../ExpedientEditor'
import style from "./FullList.module.sass"

export default function FullList() {

	const { createTab, isActive } = useTab()

	const [inputVIN, setInputVin] = createSignal<string>("")
	const [inputUser, setInputUser] = createSignal<string>("")
	const [inputBody, setInputBody] = createSignal<string>("")

	const [orderList, setHookOptions] = createHook("list_orders", {
		sort_by: "Newest",
		from_date: utcDateFuture(),
		max_list_len: 70,
		show_done: true,
		show_todo: true,
		show_awaiting: true,
		show_instore: true,
		show_urgent: true,
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

	onMount(() => {
		bindKey(document, "Escape", () => {
			console.log(event)
			if (!isActive() || (!inputUser() && !inputBody() && !inputVIN()))
				return "propagate"
			setInputUser("")
			setInputBody("")
			setInputVin("")
		})
	})

	const create_expedient_from_filters = async () => {
		let expedient = newBlankExpedient()
		expedient.user = inputUser()
		if (inputVIN().length == 17 && verifyVIN(inputVIN())) expedient.vin = inputVIN()
		else expedient.license_plate = inputVIN()

		createTab("Nou Expedient", ExpedientEditor, {
			expedientId: await createExpedient(expedient)
		})
	}
	

	const [userSuggestions, setUserFilter] = createHook("list_users", "", { defer: true })
	createEffect(() => setUserFilter(inputUser()))

	const [modelSuggestions, setModelFilter] = createHook("list_models", "", { defer: true })
	createEffect(() => setModelFilter(inputBody()))

	const [licenseSuggestions, setLicenseFilter] = createHook("list_license_plates", "", { defer: true })
	createEffect(() => setLicenseFilter(inputVIN()))

	const [vinSuggestions, setvinFilter] = createHook("list_vins", "", { defer: true })
	createEffect(() => setvinFilter(inputVIN()))

	return <>
		<div class={style.input_row}>
			<div class={style.input_user}>
				<InputText 
					suggestions={userSuggestions()}
					placeholder='Usuari'
					value={inputUser()}
					onChange={setInputUser}
				/>
			</div>
			<div class={style.input_body}>
				<InputText 
					suggestions={modelSuggestions()}
					placeholder='Cos'
					value={inputBody()}
					onChange={setInputBody}
				/>
			</div>
			<div class={style.input_vin}>
				<InputText
					suggestions={[...(licenseSuggestions() ?? []), ...(vinSuggestions() ?? [])].splice(0, 4)}
					placeholder='Matricula / VIN'
					autoFormat={['allCapital']}
					value={inputVIN()}
					onChange={setInputVin}
				/>
			</div>
			<IconButton icon='create from filters' keyMap="Enter" action={create_expedient_from_filters} />
		</div>
		<OrderList orderList={() => [...lableOrderListByDate(orderList())]} />
	</>
}