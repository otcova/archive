import { createEffect, createSignal, For, on, Show } from 'solid-js'
import Button from '../../atoms/Button'
import InputText from '../../atoms/InputText'
import InputTextArea from '../../atoms/InputTextArea'
import { createHook } from '../../database/expedientHook'
import { deleteExpedient } from '../../database/expedientState'
import { realTimeDatabaseExpedientEditor } from '../../database/realTimeEdit'
import { Expedient, ExpedientId, newBlankOrder, Order, sortOrdersByPriority, userFirstName } from '../../database/types'
import { verifyVIN } from '../../database/vin/verify'
import { modelName } from '../../database/vin/wmi'
import { ConfirmationPanel } from '../../templates/ConfirmationPanel'
import ExpedientFolderIcons from '../../templates/ExpedientFolderIcons'
import { OrderEditor } from '../../templates/OrderEditor'
import { useTab } from '../../templates/TabSystem'
import { undoSignal } from '../../utils/undo'
import style from './ExpedientEditor.module.sass'

type Props = {
	expedientId: ExpedientId,
}

export default function ExpedientEditor({ expedientId }: Props) {
	const { closeTab, rename } = useTab()

	const [showConfirmationPanel, setShowConfirmationPanel] = createSignal(false)
	const [expedient, setExpedient] = realTimeDatabaseExpedientEditor(expedientId)

	const orders = () => sortOrdersByPriority(expedient().orders)

	const setupUndo = (container) => undoSignal(expedient, setExpedient, container)

	const updateTabName = () => {
		if (!expedient()) return closeTab()

		const user = userFirstName(expedient().user)
		const orderTitles = orders()
			.filter(([order]) => order.state != "Done" && order.title)
			.map(([order]) => order.title.trim())
		const newName = [user, ...orderTitles].join("  -  ")

		rename(newName || "Expedient")
	}
	createEffect(on(expedient, updateTabName, { defer: true }))

	const updateExpedient = (data, path: keyof Expedient) => {
		const exp: Expedient = JSON.parse(JSON.stringify(expedient()))
		if (exp[path] == data) return
		exp[path] = data
		setExpedient(exp)
	}

	const updateOrder = (data, index: number, path: keyof Order) => {
		const exp: Expedient = JSON.parse(JSON.stringify(expedient()))
		if (exp.orders[index][path] == data) return
		exp.orders[index][path] = data
		setExpedient(exp)
	}

	const createOrder = () => {
		const exp: Expedient = JSON.parse(JSON.stringify(expedient()))
		exp.orders.push(newBlankOrder())
		setExpedient(exp)
	}

	const deleteExpedientResponse = (confirmedAction) => {
		setShowConfirmationPanel(false)
		if (confirmedAction) {
			deleteExpedient(expedientId)
		}
	}

	const detect_vin = (event: ClipboardEvent) => {
		if (!expedient()?.vin) {
			let pasted_text = event.clipboardData.getData("text")
			let founded_vins = Array.from(pasted_text.matchAll(
				/(?=(?:^|:|\s)([A-HJ-NPR-Z\d]{17})(?:\s|$))/gi
			), x => x[1])
			let unique_items = new Set(founded_vins)
			if (unique_items.size == 1) {
				setTimeout(() =>
					updateExpedient(founded_vins[0], "vin")
				)
				if (!expedient()?.model && modelName(founded_vins[0])) setTimeout(() =>
					updateExpedient(modelName(founded_vins[0]), "model")
				)
			}
		}
	}

	// const [similarsList, setSimilarsHookOptions] = createHook("list_expedients", {
	// 	filter: expedient(),
	// 	max_list_len: 10,
	// }, { defer: true })
	// createEffect(on(expedient, () => setSimilarsHookOptions(options => (
	// 	{ ...options, filter: expedient() }
	// )), { defer: true }))

	// createEffect(on(similarsList, () => {
	// 	console.log(similarsList())
	// }, { defer: true }))

	const [userSuggestions, setUserFilter] = createHook("list_users", "", { defer: true })
	createEffect(() => setUserFilter(expedient()?.user ?? ""))

	const [modelSuggestions, setModelFilter] = createHook("list_models", "", { defer: true })
	createEffect(() => setModelFilter(expedient()?.model ?? ""))

	const [licenseSuggestions, setLicenseFilter] = createHook("list_license_plates", "", { defer: true })
	createEffect(() => setLicenseFilter(expedient()?.license_plate.replaceAll(" ", "_") ?? ""))

	const [vinSuggestions, setvinFilter] = createHook("list_vins", "", { defer: true })
	createEffect(() => setvinFilter(expedient()?.vin ?? ""))

	return <div class={style.container} ref={setupUndo}>
		<Show when={expedient()}>
			<div class={style.expedient_container}>
				<div class={style.expedient_column_left}
					ref={elem => elem.addEventListener("paste", detect_vin)}>
					<InputText
						placeholder='Usuari'
						value={expedient().user}
						suggestions={userSuggestions()}
						onChange={data => updateExpedient(data, "user")}
						autoFormat={['startWordCapital']}
					/>
					<InputText
						placeholder='Model'
						suggestions={modelSuggestions()}
						value={expedient().model}
						onChange={data => updateExpedient(data, "model")}
					/>
					<div class={style.input_row}>
						<InputText
							autoFormat={['allCapital', 'spaceAfterNumber']}
							suggestions={licenseSuggestions()}
							placeholder='Matricula'
							value={expedient().license_plate}
							onChange={data => updateExpedient(data, "license_plate")}
						/>
						<div class={style.vin_expand_more}>
							<InputText
								autoFormat={['allCapital', 'confusingLettersToNumbers']}
								suggestions={vinSuggestions()}
								placeholder='VIN'
								validate={verifyVIN}
								value={expedient().vin}
								onChange={data => updateExpedient(data, "vin")}
							/>
						</div>
					</div>
					<InputTextArea
						placeholder='Descripció'
						value={expedient().description}
						onChange={data => updateExpedient(data, "description")}
					/>
				</div>
				<div class={style.expedient_column_right}>
					<For each={orders().map(([_, orderIndex]) => orderIndex)}>{(orderIndex) => {
						return <OrderEditor
							expedient={expedient}
							expedientId={expedientId}
							setOrder={(data, path) => updateOrder(data, orderIndex, path)}
							orderIndex={orderIndex}
						/>
					}}</For>
				</div>
			</div>
			<div class={style.bottom_bar}>
				<ExpedientFolderIcons expedient={expedient()} />
				<div class={style.bottom_bar_buttons}>
					<Button text="Eliminar" red action={() => setShowConfirmationPanel(true)} />
					<Button text="Nova Comanda" action={createOrder} />
					<Button text="Arxivar" action={closeTab} />
				</div>
			</div>
			<ConfirmationPanel text="Eliminar Expedient"
				show={showConfirmationPanel()}
				response={deleteExpedientResponse}
			/>
		</Show >
	</div>
}