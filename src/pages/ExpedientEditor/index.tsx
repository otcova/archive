import { createEffect, createMemo, For, Show } from 'solid-js'
import Button from '../../atoms/Button'
import IconButton from '../../atoms/IconButton'
import InputText from '../../atoms/InputText'
import InputTextArea from '../../atoms/InputTextArea'
import { createHook } from '../../database/expedientHook'
import { updateExpedient } from '../../database/expedientState'
import { compareDate, ExpedientId, expedientUtils, Order } from '../../database/types'
import { OrderEditor } from '../../templates/OrderEditor'
import { useTab } from '../../templates/TabSystem'
import style from './ExpedientEditor.module.sass'

type Props = {
	expedientId: ExpedientId,
}

export default function ExpedientEditor({ expedientId }: Props) {
	const { closeTab, rename } = useTab()

	const [expedient] = createHook("expedient", expedientId)
	const orders = () => arrangeOrders(expedient().orders)

	const updateTabName = () => {
		const user = expedientUtils.strUsers(expedient())
		const orderTitles = orders()
			.filter(([order]) => order.state != "Done")
			.filter(([order]) => order.title)
			.map(([order]) => order.title)
		const newName = [user.trim(), ...orderTitles].join("  -  ").trim()

		if (newName) rename(newName)
		else rename("Expedient")
	}

	createEffect(() => {
		if (expedient()) updateTabName()
	})

	const onLicensePlateChange = newLicensePlate => {
		const exp = expedient()
		if (exp.license_plate == newLicensePlate) return
		exp.license_plate = newLicensePlate
		updateExpedient(expedientId, exp)
	}

	const onVINChange = newVin => {
		const exp = expedient()
		if (exp.vin == newVin) return
		exp.vin = newVin
		updateExpedient(expedientId, exp)
	}

	const onModelChange = newModel => {
		const exp = expedient()
		if (exp.model == newModel) return
		exp.model = newModel
		updateExpedient(expedientId, exp)
	}

	const onUserChange = newUser => {
		const exp = expedient()
		if (exp.users.length == 0) exp.users = [{ name: "", emails: [], phones: [] }]
		else if (exp.users[0].name == newUser) return
		exp.users[0].name = newUser
		updateExpedient(expedientId, exp)
		updateTabName()
	}

	const onDescriptionChange = newDescription => {
		const exp = expedient()
		if (exp.description == newDescription) return
		exp.description = newDescription
		updateExpedient(expedientId, exp)
	}


	return <Show when={expedient()}>
		<div class={style.expedient_container}>
			<div class={style.expedient_column_left}>
				<InputText placeholder='Usuari' value={expedient().users[0]?.name ?? ""} onChange={onUserChange} />
				<InputText placeholder='Model' value={expedient().model} onChange={onModelChange} />
				<div class={style.input_row}>
					<InputText placeholder='Matricula' value={expedient().license_plate} onChange={onLicensePlateChange} />
					<div class={style.vin_expand_more}>
						<InputText placeholder='VIN' value={expedient().vin} onChange={onVINChange} />
					</div>
				</div>
				<InputTextArea placeholder='Descripció' value={expedient().description} onChange={onDescriptionChange} />
			</div>
			<div class={style.expedient_column_right}>
				<For each={orders().map(([_, orderIndex]) => orderIndex)}>{(orderIndex) => {
					return <OrderEditor expedient={expedient} expedientId={expedientId} orderIndex={orderIndex} />
				}}</For>
			</div>
		</div>
		<div class={style.bottom_bar}>
			<IconButton icon='folder' />
			<div class={style.bottom_bar_folder_space}></div>
			<IconButton icon='image' />
			<IconButton icon='document' />
			<IconButton icon='pdf' />
			<div class={style.bottom_bar_space}></div>
			<Button text="Arxivar" action={closeTab} />
		</div>
	</Show>
}

function arrangeOrders(orders: Order[]): [Order, number][] {
	const arrangedOrders: [Order, number][] = []

	const indexedOrders: [Order, number][] = [...orders].map((order, index) => [order, index])
	const sortedOrders = indexedOrders.sort(([a], [b]) => compareDate(a.date, b.date))

	for (const state of ["Urgent", "Todo", "Pending", "Done"]) {
		for (const order of sortedOrders) {
			if (order[0].state == state)
				arrangedOrders.push(order)
		}
	}

	return arrangedOrders
}