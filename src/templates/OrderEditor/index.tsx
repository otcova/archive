import { Accessor, createEffect } from 'solid-js'
import Checkbox from '../../atoms/Checkbox'
import InputText from '../../atoms/InputText'
import InputTextArea from '../../atoms/InputTextArea'
import { Expedient, ExpedientId, utcDateToString } from '../../database/types'
import style from './OrderEditor.module.sass'

type Props = {
	expedient: Accessor<Expedient>,
	expedientId: ExpedientId,
	setOrder: (Order, path: string) => void,
	orderIndex: number,
}

export function OrderEditor(props: Props) {
	const order = () => props.expedient().orders[props.orderIndex]

	return <div class={style.container}>
		<div class={style.title_bar}>
			<InputText noStyle
				placeholder='Títol'
				autoFormat={['firstCapital']}
				value={order().title}
				onChange={data => props.setOrder(data, "title")}
			/>
			<div class={style.date}>{utcDateToString(order().date)}</div>
			<Checkbox expedient={props.expedient()} expedientId={props.expedientId} orderIndex={props.orderIndex} />
		</div>
		<InputTextArea noStyle
			placeholder='Comanda'
			value={order().description}
			onChange={data => props.setOrder(data, "description")}
		/>
	</div>
}