import Button from '../../atoms/Button'
import ExpedientList from '../ExpedientList'
import { useTab } from '../TabSystem'
import style from './Historial.module.sass'

export default function Historial() {
	const { closeTab, rename } = useTab()
	return <>
		<ExpedientList hookType={'all_expedients'} />
		<div class={style.bottom_bar}>
			<Button text="Tancar" action={closeTab} />
		</div>
	</>
}