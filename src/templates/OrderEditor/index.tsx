import { Accessor, createEffect } from 'solid-js'
import Checkbox from '../../atoms/Checkbox'
import InputText from '../../atoms/InputText'
import InputTextArea from '../../atoms/InputTextArea'
import { Expedient, ExpedientId } from '../../database'
import { genTestExpedient } from '../../database/temporal'
import { localDateToString } from '../../database/types'
import style from './OrderEditor.module.sass'

type Props = {
	expedient: Accessor<Expedient>,
	expedientId: ExpedientId,
	orderIndex: number,
}

export function OrderEditor(props: Props) {
	const order = () => props.expedient().orders[props.orderIndex]

	return <div class={style.container}>
		<div class={style.title_bar}>
			<InputText placeholder='Títol' noStyle defaultValue={order().title} />
			<div class={style.date}>{localDateToString(order().date)}</div>
			<Checkbox expedient={props.expedient()} expedientId={props.expedientId} orderIndex={props.orderIndex} />
		</div>
		<InputTextArea placeholder='Comanda' noStyle defaultValue={order().description} />
	</div>
}