import Checkbox from '../../atoms/Checkbox'
import InputText from '../../atoms/InputText'
import InputTextArea from '../../atoms/InputTextArea'
import { genTestExpedient } from '../../database/temporal'
import style from './OrderEditor.module.sass'

export function OrderEditor() {
	return <div class={style.container}>
		<div class={style.title_bar}>
			<InputText placeholder='Títol' noStyle />
			<div class={style.date}>17 - 06 - 2022</div>
			<Checkbox expedient={genTestExpedient()} expedientId={{DYNAMIC:0}}/>
		</div>
		<InputTextArea placeholder='Comanda' noStyle/>
	</div>
}