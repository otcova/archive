import Checkbox from "../../atoms/Checkbox"
import { Expedient, ExpedientId } from "../../database/types"
import ExpedientEditor from "../../pages/ExpedientEditor"
import { useTab } from "../TabSystem"
import style from './OrderList.module.sass'

export function Row(props: { data: [ExpedientId, number, Expedient] }) {
	const [expedientId, orderIndex, expedient] = props.data
	const order = expedient.orders[orderIndex]
	const { createTab } = useTab()

	const openOrder = () => {
		createTab("", ExpedientEditor, { expedientId })
	}

	return <div class={style.row_container} onClick={openOrder}>
		<Checkbox expedient={expedient} expedientId={expedientId} orderIndex={orderIndex} />
		<div class={style.grow}>{expedient.user}</div>
		<div class={style.grow}>{order.title}</div>
		<div class={style.grow}>{expedient.model}</div>
		<div class={style.license_plate}>{expedient.license_plate}</div>
	</div>
}

export function RowLable({ text }: { text: string }) {
	return <div class={style.lable}>{text}</div>
}