import { invoke } from '@tauri-apps/api/tauri'
import Button from '../../atoms/Button'
import { Expedient, Order } from '../../database'
import { createExpedient } from '../../database/expedientState'
import OrderList from '../../templates/OrderList'
import Historial, { openHistorial } from '../Historial'
import { useTab } from '../../templates/TabSystem'
import style from './Agenda.module.sass'

export default function Agenda() {
	const { createTab } = useTab()
	
	return <>
		<OrderList />
		{/* <div class={style.bottom_bar}>
			<Button text="Historial" action={() => openHistorial(createTab)} />
			<div class={style.bottom_bar_space}></div>
			<Button text="Buscar Expedient" action={calculate}/>
			<Button text="Crear Expedient" action={testCreateExpedients} />
		</div> */}
	</>
}