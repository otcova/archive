import { Accessor, Setter } from "solid-js"
import InputText from "../../atoms/InputText"
import style from "./DateEditor.module.sass"

type Props = {
	date: Accessor<Date>,
	setDate: Setter<Date>
}

export default function DateEditor(props: Props) {
	return <div class={style.container}>
		<div class={style.day}>
			<InputText noStyle selectOnFocus
				maxLen={2}
				charRegex={/\d/}
				onChange={day => props.setDate(new Date(props.date().setDate(Number(day) || 1)))}
				value={"" + props.date().getDate()}
			/>
		</div>
		{"  -  "}
		<div class={style.month}>
			<InputText noStyle selectOnFocus
				maxLen={2}
				charRegex={/\d/}
				onChange={month => props.setDate(new Date(
					props.date().setMonth(
						Math.min(12, (Number(month) || 1)) - 1
					)
				))}
				value={"" + (props.date().getMonth() + 1)}
			/>
		</div >
		{"  -  "}
		< div class={style.year}>
			<InputText noStyle selectOnFocus
				maxLen={4}
				charRegex={/\d/}
				onChange={year => props.setDate(new Date(props.date().setFullYear(Number(year))))}
				value={"" + props.date().getFullYear()}
			/>
		</div >
	</div >
}