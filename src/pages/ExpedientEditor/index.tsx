import { createEffect, createMemo, For, Show } from 'solid-js'
import Button from '../../atoms/Button'
import IconButton from '../../atoms/IconButton'
import InputText from '../../atoms/InputText'
import InputTextArea from '../../atoms/InputTextArea'
import { ExpedientId, Order } from '../../database'
import { createHook } from '../../database/expedientHook'
import { compareDate } from '../../database/types'
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

	return <Show when={expedient()}>
		<div class={style.expedient_container}>
			<div class={style.expedient_column_left}>
				<InputText placeholder='Usuari' defaultValue={expedient().users[0]?.name ?? ""} onChange={rename} />
				<InputText placeholder='Model' defaultValue={expedient().model} />
				<div class={style.input_row}>
					<InputText placeholder='Matricula' defaultValue={expedient().license_plate} />
					<div class={style.vin_expand_more}>
						<InputText placeholder='VIN' defaultValue={expedient().vin} />
					</div>
				</div>
				<InputTextArea placeholder='Descripció' defaultValue={expedient().description} />
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