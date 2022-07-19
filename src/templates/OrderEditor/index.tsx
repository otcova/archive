import Checkbox from '../../atoms/Checkbox'
import InputText from '../../atoms/InputText'
import InputTextArea from '../../atoms/InputTextArea'
import style from './OrderEditor.module.sass'

export function OrderEditor() {
	return <div class={style.container}>
		<div class={style.title_bar}>
			<InputText placeholder='Títol' noStyle />
		</div>
		<InputTextArea placeholder='Comanda' noStyle/>
	</div>
}