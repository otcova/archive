import Button from '../../atoms/Button'
import IconButton from '../../atoms/IconButton'
import InputText from '../../atoms/InputText'
import style from './ExpedientEditor.module.sass'

export default function ExpedientEditor() {

	return <>
		<div class={style.expedient_container}>
			<div class={style.expedient_column_left}>
				<InputText placeholder='Usuari'/>
				<InputText placeholder='Model'/>
				<div class={style.input_row}>
					<InputText placeholder='Matricula' charRegex={/[0-9]+/} maxLen={7}/>
					<InputText placeholder='VIN'/>
				</div>
			</div>
			<div class={style.expedient_column_right}>
				<Button text="Hello!" />
				<Button text="Hello!" />
				<Button text="Hello!" />
				<Button text="Hello!" />
				<Button text="Hello!" />
				<Button text="Hello!" />
				<Button text="Hello!" />
				<Button text="Hello!" />
				<Button text="Hello!" />
				<Button text="Hello!" />
				<Button text="Hello!" />
				<Button text="Hello!" />
				<Button text="Hello!" />
				<Button text="Hello!" />
				<Button text="Hello!" />
				<Button text="Hello!" />
				<Button text="Hello!" />
				<Button text="Hello!" />
			</div>
		</div>
		<div class={style.bottom_bar}>
			<IconButton icon='folder' />
			<div class={style.bottom_bar_space}></div>
			<Button text="Arxivar" />
		</div>
	</>
}