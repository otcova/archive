import { Accessor, createEffect } from 'solid-js'
import Checkbox from '../../atoms/Checkbox'
import InputText from '../../atoms/InputText'
import InputTextArea from '../../atoms/InputTextArea'
import { Expedient, ExpedientId, expedientUtils } from '../../database'
import { genTestExpedient } from '../../database/temporal'
import style from './OrderEditor.module.sass'

type Props = {
	expedient: Accessor<Expedient>,
	expedientId: ExpedientId,
	orderIndex: number,
	defaultOpen?: boolean
}

export function OrderEditor(props: Props) {
	const order = () => props.expedient().orders[props.orderIndex]
	createEffect(() => console.log(props.expedient()))

	return <div class={style.container}>
		<div class={style.title_bar}>
			<InputText placeholder='Títol' noStyle defaultValue={order().title} />
			<div class={style.date}>{expedientUtils.strDate(order().date)}</div>
			<Checkbox expedient={props.expedient()} expedientId={props.expedientId} orderIndex={props.orderIndex} t/>
		</div>
		<InputTextArea placeholder='Comanda' noStyle defaultValue={order().description} />
	</div>
}