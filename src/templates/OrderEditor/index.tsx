import { Accessor, createSignal } from 'solid-js'
import Checkbox from '../../atoms/Checkbox'
import InputText from '../../atoms/InputText'
import InputTextArea from '../../atoms/InputTextArea'
import { Expedient, ExpedientId, refactorExpedientOrders } from '../../database/types'
import { utcDateToString } from '../../database/date'
import style from './OrderEditor.module.sass'
import { ContextMenu } from '../../atoms/ContextMenu'
import { updateExpedient } from '../../database/expedientState'

type Props = {
	expedient: Accessor<Expedient>,
	expedientId: ExpedientId,
	setOrder: (Order, path: string) => void,
	orderIndex: number,
}

export function OrderEditor(props: Props) {

	const order = () => props.expedient().orders[props.orderIndex]

	const onContextMenuClick = () => {
		const expedient = refactorExpedientOrders({
			...props.expedient(),
			orders: props.expedient().orders.filter((_, index) => index != props.orderIndex)
		})
		updateExpedient(props.expedientId, expedient)
	}

	return <ContextMenu
		buttons={[{ text: "Eliminar Commanda", red: true }]}
		onClick={onContextMenuClick}
	>
		<div class={style.container}>
			<div class={style.title_bar}>
				<InputText noStyle
					placeholder='Títol'
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