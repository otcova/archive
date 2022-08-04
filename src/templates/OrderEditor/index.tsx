import { Accessor, createEffect } from 'solid-js'
import Checkbox from '../../atoms/Checkbox'
import { ContextMenu } from '../../atoms/ContextMenu'
import InputText from '../../atoms/InputText'
import InputTextArea from '../../atoms/InputTextArea'
import { utcDateToString } from '../../database/date'
import { createHook } from '../../database/expedientHook'
import { updateExpedient } from '../../database/expedientState'
import { Expedient, ExpedientId, newBlankOrder, Order } from '../../database/types'
import style from './OrderEditor.module.sass'

type Props = {
	expedient: Accessor<Expedient>,
	expedientId: ExpedientId,
	setOrder: (data, path: keyof Order) => void,
	orderIndex: number,
}

export function OrderEditor(props: Props) {

	const order = () => props.expedient().orders[props.orderIndex]

	const deleteOrder = () => {
		const expedient = props.expedient()
		expedient.orders.splice(props.orderIndex, 1)
		if (expedient.orders.length == 0) expedient.orders.push(newBlankOrder())
		updateExpedient(props.expedientId, { ...expedient })
	}

	const [titleSuggestions, setTitleFilter] = createHook("list_order_titles", "", { defer: true })
	createEffect(() => setTitleFilter(order()?.title ?? ""))

	return <ContextMenu
		buttons={[{ text: "Eliminar Commanda", red: true }]}
		onClick={deleteOrder}
	>
		<div class={style.container}>
			<div class={style.title_bar}>
				<InputText noStyle
					placeholder='Títol'
					suggestions={titleSuggestions()}
					autoFormat={['firstCapital']}
					value={order().title}
					onChange={data => props.setOrder(data, "title")}
				/>
				<div class={style.date}>{utcDateToString(order().date)}</div>
				<Checkbox
					expedient={props.expedient()}
					expedientId={props.expedientId}
					orderIndex={props.orderIndex}
				/>
			</div>
			<InputTextArea noStyle
				placeholder='Comanda'
				value={order().description}
				onChange={data => props.setOrder(data, "description")}
			/>
		</div>
	</ContextMenu>
}