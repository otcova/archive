
import { createEffect, createSignal, Show, untrack } from "solid-js"
import { jsDateToUtc, UtcDate, utcToJsDate } from "../../database/types"
import Button from "../../atoms/Button"
import SelectionButtons from "../../atoms/SelectionButtons"
import style from "./AwaitDateSelectorPanel.module.sass"
import DateEditor from "../DateEditor"

type Props = {
	show: boolean
	date: UtcDate
	response: (date: null | UtcDate) => void
}

export function AwaitDateSelectorPanel(props: Props) {
	let [date, setDate] = createSignal(utcToJsDate(props.date))

	createEffect(() => {
		if (props.show) {
			setDate(utcToJsDate(untrack(() => props.date)))
		}
	})

	const updateTime = (time) => {
		if (time == "Matí") date().setHours(8)
		if (time == "Tarda") date().setHours(15)
	}

	const stopPropagation = event => event.stopPropagation()

	const defaultTimeIndex = date().getHours() < 10 ? 0 : 1

	return <Show when={props.show}>
		<div class={style.container} onClick={stopPropagation} data-tauri-drag-region>
			<div class={style.panel}>
				<div class={style.date_container}>
					<div class={style.row}>
						<DateEditor date={date} setDate={setDate} />
					</div>
					<SelectionButtons buttons={["Matí", "Tarda"]} default={defaultTimeIndex} onSelect={updateTime} />
				</div>
				<div class={style.buttons_row}>
					<Button text={"Cancelar"} red
						action={() => props.response(null)}
					/>
					<Button text={"Confirmar"}
						action={() => props.response(jsDateToUtc(date()))}
					/>
				</div>
			</div>
		</div>
	</Show >
}