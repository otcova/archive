import style from "./styles/checkbox.module.css"

type Props = {
	checked: boolean,
	onChange?: (newValue: boolean) => void,
}

export function CheckBox({ checked, onChange }: Props) {
	return <button className={style.checkBox} onClick={() => onChange?.(!checked)}>
		{checked && tick}
	</button >
}

const tick = <svg
	width="9.6139965"
	height="10.767004"
	viewBox="0 0 2.5437031 2.8487699">
	<path style="fill:none;stroke:#0078d4;stroke-width:0.529166;stroke-linecap:butt;stroke-linejoin:round;stroke-miterlimit:4;stroke-dasharray:none;stroke-opacity:1;paint-order:normal"
		d="M 0.17225301,1.8604905 1.016032,2.5842011 2.309496,0.12308918" />
</svg>