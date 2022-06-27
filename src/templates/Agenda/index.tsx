import { onCleanup, onMount } from 'solid-js'
import Button from '../../atoms/Button'
import ExpedientList from '../ExpedientList'
import Historial from '../Historial'
import { useTab } from '../TabSystem'
import style from './Agenda.module.sass'

export default function Agenda() {
	const { createTab } = useTab()
	const openHistorial = () => {
		createTab("Historial", Historial)
	}

	return <>
		<ExpedientList hookType={'all_open_expedients'} />
		<div class={style.bottom_bar}>
			<Button text="Historial" action={openHistorial} />
			<div class={style.bottom_bar_space}></div>
			<Button text="Buscar Expedient" />
			<Button text="Crear Expedient" />
		</div>
	</>
}