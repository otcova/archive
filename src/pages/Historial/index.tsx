import { onCleanup } from 'solid-js'
import Button from '../../atoms/Button'
import OrderList from '../../templates/OrderList'
import { useTab } from '../../templates/TabSystem'
import style from './Historial.module.sass'

let focusHistorial = null

export default Historial
function Historial() {
	const { closeTab, focusTab } = useTab()
	focusHistorial = focusTab
	onCleanup(() => focusHistorial = null)
	
	return <>
		<OrderList hookType={'all_expedients'} />
		<div class={style.bottom_bar}>
			<Button text="Tancar" action={closeTab} />
		</div>
	</>
}

export function openHistorial(createTab) {
	if (focusHistorial) focusHistorial()
	else createTab("Historial", Historial)
}

