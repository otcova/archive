
import { Show } from "solid-js"
import Button from "../../atoms/Button"
import style from "./ConfirmationPanel.module.sass"

type Props = {
	show: boolean
	text: string,
	response: (date: boolean) => void
}

export function ConfirmationPanel(props: Props) {

	const stopPropagation = event => event.stopPropagation()

	return <Show when={props.show}>
		<div class={style.container} onClick={stopPropagation} data-tauri-drag-region>
			<div class={style.panel}>
				<div class={style.text}>
					{props.text}
				</div>
				<div class={style.buttons_row}>
					<Button text={"Cancelar"} red
						action={() => props.response(false)}
					/>
					<Button text={"Confirmar"}
						action={() => props.response(true)}
					/>
				</div>
			</div>
		</div>
	</Show >
}