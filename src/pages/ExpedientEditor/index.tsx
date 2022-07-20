import Button from '../../atoms/Button'
import IconButton from '../../atoms/IconButton'
import InputText from '../../atoms/InputText'
import InputTextArea from '../../atoms/InputTextArea'
import { Expedient, ExpedientId } from '../../database'
import { OrderEditor } from '../../templates/OrderEditor'
import { useTab } from '../../templates/TabSystem'
import style from './ExpedientEditor.module.sass'

type Props = {
	expedient: Expedient,
	expedientId: ExpedientId,
	orderIndex?: number,
}

export default function ExpedientEditor(props: Props) {
	const { closeTab, rename } = useTab()

	return <>
		<div class={style.expedient_container}>
			<div class={style.expedient_column_left}>
				<InputText placeholder='Usuari' defaultValue={props.expedient.users[0].name} onChange={rename} />
				<InputText placeholder='Model' defaultValue={props.expedient.model} />
				<div class={style.input_row}>
					<InputText placeholder='Matricula' defaultValue={props.expedient.license_plate} />
					<div class={style.vin_expand_more}>
						<InputText placeholder='VIN' defaultValue={props.expedient.vin} />
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