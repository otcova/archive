import style from "./styles/checkbox.module.css"
import tick from "./img/tick"

type Props = {
	checked: boolean,
	onChange?: (newValue: boolean) => void,
}

export function CheckBox({ checked, onChange }: Props) {
	return <button className={style.checkBox} onClick={() => onChange?.(!checked)}>
		{checked && tick}
	</button >
}