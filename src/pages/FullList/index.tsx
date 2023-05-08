import { Show, createEffect, createSignal, onMount } from 'solid-js'
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

	const [userPopularity, setUserPopularity] = createSignal<number>(0)
	const [inputUser, setInputUser] = createSignal<string>("")
	const [inputBody, setInputBody] = createSignal<string>("")
	const [inputVIN, setInputVin] = createSignal<string>("")


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
		const filter = inputVIN() + inputUser() + inputBody() + userPopularity() != ""
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
						popularity: userPopularity(),
					}
				}
			})
		}
	})

	onMount(() => {
		bindKey(document, "Escape", () => {
			if (!isActive()) return "propagate"
			if (userPopularity()) setUserPopularity(0)
			else {
				if (!inputUser() && !inputBody() && !inputVIN()) return "propagate"
				setInputUser("")
				setInputBody("")
				setInputVin("")
			}
		})
		bindKey(document, "+", () => {
			if (!isActive()) return "propagate"
			setUserPopularity(p => {
				if (p == -5) return 0;
				if (p < 0) return p / 2;
				if (p == 0) return 5;
				return Math.min(80, 2 * p);
			})
		})
		bindKey(document, "-", () => {
			if (!isActive()) return "propagate"
			setUserPopularity(p => {
				if (p == 5) return 0;
				if (p > 0) return p / 2;
				if (p == 0) return -5;
				return Math.max(-80, 2 * p);
			})
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

		setInputVin("")
		setInputBody("")
		setInputUser("")
	}


	const [userSuggestions, setUserFilter] = createHook("list_users", "", { defer: true })
	createEffect(() => setUserFilter(inputUser()))

	const [modelSuggestions, setModelFilter] = createHook("list_models", "", { defer: true })
	createEffect(() => setModelFilter(inputBody()))

	const [licenseSuggestions, setLicenseFilter] = createHook("list_license_plates", "", { defer: true })
	createEffect(() => setLicenseFilter(inputVIN()))

	const [vinSuggestions, setvinFilter] = createHook("list_vins", "", { defer: true })
	createEffect(() => setvinFilter(inputVIN()))

	const displayPopularity = () => {
		let n = userPopularity()
		if (n == 0) return ""
		return (n > 0 ? "+ " : "- ") + Math.abs(userPopularity())
	}

	return <>
		<div class={style.input_row}>
			<div class={style.input_user}>
				<Show when={userPopularity() != 0}>
					<div class={style.popularity}>{displayPopularity()}</div>
				</Show>
				<InputText
					suggestions={userSuggestions()}
					placeholder='Usuari'
					value={inputUser()}
					onChange={setInputUser}
					escape="clear"
				/>
			</div>
			<div class={style.input_body}>
				<InputText
					suggestions={modelSuggestions()}
					placeholder='Cos'
					value={inputBody()}
					onChange={setInputBody}
					escape="clear"
				/>
			</div>
			<div class={style.input_vin}>
				<InputText
					suggestions={[...(licenseSuggestions() ?? []), ...(vinSuggestions() ?? [])].splice(0, 4)}
					placeholder='Matricula / VIN'
					autoFormat={['allCapital']}
					value={inputVIN()}
					onChange={setInputVin}
					escape="clear"
				/>
			</div>
			<IconButton icon='create from filters' keyMap="Enter" action={create_expedient_from_filters} />
		</div>
		<OrderList orderList={() => [...lableOrderListByDate(orderList())]} />
	</>
}