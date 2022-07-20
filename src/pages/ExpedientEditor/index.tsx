import Button from '../../atoms/Button'
import IconButton from '../../atoms/IconButton'
import InputText from '../../atoms/InputText'
import InputTextArea from '../../atoms/InputTextArea'
import { OrderEditor } from '../../templates/OrderEditor'
import { useTab } from '../../templates/TabSystem'
import style from './ExpedientEditor.module.sass'

export default function ExpedientEditor() {
	const { closeTab } = useTab()

	return <>
		<div class={style.expedient_container}>
			<div class={style.expedient_column_left}>
				<InputText placeholder='Usuari' />
				<InputText placeholder='Model' />
				<div class={style.input_row}>
					<InputText placeholder='Matricula' />
					<div class={style.vin_expand_more}>
						<InputText placeholder='VIN' />
					</div>
				</div>
				<InputTextArea placeholder='Descripció' />
			</div>
			<div class={style.expedient_column_right}>
				<OrderEditor />
			</div>
		</div>
		<div class={style.bottom_bar}>
			<IconButton icon='folder' />
			<div class={style.bottom_bar_folder_space}></div>
			<IconButton icon='image' />
			<IconButton icon='document' />
			<IconButton icon='pdf' />
			<div class={style.bottom_bar_space}></div>
			<Button text="Arxivar" action={closeTab} />
		</div>
	</>
}