import { CheckBox, NavBar } from "../../components";
import style from "./cotxes.module.css";

export type Filter = {
	checked: boolean,
}

type Props = {
	filter: Filter,
	onFilterChange: (newFilter: Filter) => void,
}

export function CotxesBar({ filter, onFilterChange }: Props) {
	return <NavBar>
		<CheckBox checked={filter.checked} onChange={checked => onFilterChange({checked})} />
		<Title />
		<button>Usuaris</button>
		<button className="primary-button">Obrir Expedient</button>
	</NavBar>
}

function Title() {
	return <div
		data-tauri-drag-region
		className={style.title}>
		Cotxes
	</div>
}