import { createEffect, createSignal, For, on, Show } from 'solid-js'
import Button from '../../atoms/Button'
import InputText from '../../atoms/InputText'
import InputTextArea from '../../atoms/InputTextArea'
import { createHook } from '../../database/expedientHook'
import { deleteExpedient } from '../../database/expedientState'
import { realTimeDatabaseExpedientEditor } from '../../database/realTimeEdit'
import { Expedient, ExpedientId, newBlankOrder, Order, sortOrdersByPriority } from '../../database/types'
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

		const user = expedient().user.split(/\s/)[0].trim()
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

	const [userSuggestions, setUserFilter] = createHook("list_users", "")
	createEffect(() => {
		if (expedient()?.user) setUserFilter(expedient().user)
	})

	return <div class={style.container} ref={setupUndo}>
		<Show when={expedient()}>
			<div class={style.expedient_container}>
				<div class={style.expedient_column_left}>
					<InputText
						placeholder='Usuari'
						value={expedient().user}
						suggestions={userSuggestions()}
						onChange={data => updateExpedient(data, "user")}
						autoFormat={['startWordCapital']}
					/>
					<InputText
						placeholder='Model'
						value={expedient().model}
						onChange={data => updateExpedient(data, "model")}
					/>
					<div class={style.input_row}>
						<InputText
							autoFormat={['allCapital', 'spaceAfterNumber']}
							placeholder='Matricula'
							value={expedient().license_plate}
							onChange={data => updateExpedient(data, "license_plate")}
						/>
						<div class={style.vin_expand_more}>
							<InputText
								autoFormat={['allCapital']}
								placeholder='VIN'
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