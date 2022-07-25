import { createEffect, createMemo, For, Show } from 'solid-js'
import Button from '../../atoms/Button'
import IconButton from '../../atoms/IconButton'
import InputText from '../../atoms/InputText'
import InputTextArea from '../../atoms/InputTextArea'
import { realTimeDatabaseExpedientEditor } from '../../database/realTimeEdit'
import { compareDate, ExpedientId, expedientUtils, Order } from '../../database/types'
import { OrderEditor } from '../../templates/OrderEditor'
import { useTab } from '../../templates/TabSystem'
import style from './ExpedientEditor.module.sass'

type Props = {
	expedientId: ExpedientId,
}

export default function ExpedientEditor({ expedientId }: Props) {
	const { closeTab, rename } = useTab()

	const [expedient, setExpedient] = realTimeDatabaseExpedientEditor(expedientId)

	const orders = () => arrangeOrders(expedient().orders)

	const updateTabName = () => {
		const user = expedient().user.split(/\s/)[0]
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

	const updateExpedient = (data, ...path: (string | number)[]) => {
		const exp = expedient()
		if (readDeepPath(exp, path) == data) return
		const newExpedient = { ...exp }
		setDeepPath(newExpedient, path, data)
		setExpedient(newExpedient)
	}


	return <Show when={expedient()}>
		<div class={style.expedient_container}>
			<div class={style.expedient_column_left}>
				<InputText placeholder='Usuari' value={expedient().user} onChange={data => updateExpedient(data, "user")} />
				<InputText placeholder='Model' value={expedient().model} onChange={data => updateExpedient(data, "model")} />
				<div class={style.input_row}>
					<InputText placeholder='Matricula' value={expedient().license_plate} onChange={data => updateExpedient(data, "license_plate")} />
					<div class={style.vin_expand_more}>
						<InputText placeholder='VIN' value={expedient().vin} onChange={data => updateExpedient(data, "vin")} />
					</div>
				</div>
				<InputTextArea placeholder='Descripció' value={expedient().description} onChange={data => updateExpedient(data, "description")} />
			</div>
			<div class={style.expedient_column_right}>
				<For each={orders().map(([_, orderIndex]) => orderIndex)}>{(orderIndex) => {
					return <OrderEditor
						expedient={expedient}
						expedientId={expedientId}
						setOrder={(order, path) => updateExpedient(order, "orders", orderIndex, path)}
						orderIndex={orderIndex}
					/>
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

function readDeepPath(obj: object, path: (string | number)[], _index = 0) {
	const child = obj[path[_index]]
	if (path.length - _index == 1) return child
	return readDeepPath(child, path, _index + 1);
}

function setDeepPath(obj: object, path: (string | number)[], value: any, _index = 0) {
	if (path.length - 1 == _index) obj[path[_index]] = value
	else setDeepPath(obj[path[_index]], path, value, _index + 1);
}