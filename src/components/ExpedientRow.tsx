import { CheckBox } from "./checkbox"
import style from "./styles/expedientRow.module.css"

type Props = {
}

export function ExpedientRow({ }: Props) {
	return <div className={style.container}>
		<CheckBox checked />
		<div className={style.users}>Some</div>
		<div className={style.model}>Some</div>
		<div>1242 GET</div>
		{openIcon}
	</div>
}

const openIcon = <svg
	width="20.399544"
	height="20.501972"
	viewBox="0 0 5.3973791 5.4244804">
	<rect
		style="fill:none;fill-opacity:1;stroke:#5c5c5c;stroke-width:0.529167;stroke-linecap:round;stroke-linejoin:miter;stroke-miterlimit:4;stroke-dasharray:none;stroke-opacity:1;paint-order:normal;stop-color:#000000"
		id="rect6941"
		width="3.96875"
		height="3.96875"
		x="0.2645835"
		y="1.1911465"
		rx="1.3229166"
		ry="1.3229166" />
	<path
		style="fill:none;stroke:#ffffff;stroke-width:1.32292;stroke-linecap:butt;stroke-linejoin:round;stroke-miterlimit:4;stroke-dasharray:none;stroke-opacity:1"
		d="M 2.283826,3.1135556 4.929656,0.46772257"
		id="path6943" />
	<path
		style="fill:none;stroke:#5c5c5c;stroke-width:0.529167;stroke-linecap:butt;stroke-linejoin:round;stroke-miterlimit:4;stroke-dasharray:none;stroke-opacity:1"
		d="m 2.283826,3.1135556 2.64583,-2.64583303 0.0165,1.95143403"
		id="path6945" />
	<path
		style="fill:none;stroke:#5c5c5c;stroke-width:0.529167;stroke-linecap:butt;stroke-linejoin:round;stroke-miterlimit:4;stroke-dasharray:none;stroke-opacity:1"
		d="m 2.283826,3.1135556 2.6457,-2.64570103 h -1.9513"
		id="path6947" />
</svg>