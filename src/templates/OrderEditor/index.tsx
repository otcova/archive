import { Accessor, createEffect } from 'solid-js'
import Checkbox from '../../atoms/Checkbox'
import InputText from '../../atoms/InputText'
import InputTextArea from '../../atoms/InputTextArea'
import { updateExpedient } from '../../database/expedientState'
import { Expedient, ExpedientId, utcDateToString } from '../../database/types'
import style from './OrderEditor.module.sass'

type Props = {
	expedient: Accessor<Expedient>,
	expedientId: ExpedientId,
	orderIndex: number,
}

export function OrderEditor(props: Props) {
	const order = () => props.expedient().orders[props.orderIndex]

	const onTitleChange = newUser => {
		const expedient = props.expedient()
		expedient.orders[props.orderIndex].title = newUser
		updateExpedient(props.expedientId, expedient)
	}

	return <div class={style.container}>
		<div class={style.title_bar}>
			<InputText placeholder='Títol' noStyle value={order().title} onChange={onTitleChange} />
			<div class={style.date}>{utcDateToString(order().date)}</div>
			<Checkbox expedient={props.expedient()} expedientId={props.expedientId} orderIndex={props.orderIndex} />
		</div>
		<InputTextArea placeholder='Comanda' noStyle value={order().description} />
	</div>
}