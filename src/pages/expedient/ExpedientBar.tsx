import { NavBar } from "../../components";
import style from "./expedient.module.css";

export function ExpedientBar() {
	const usuarisClick = () => {
	}
	return <NavBar>
		<button onClick={usuarisClick}>3 Similars</button>
		<div className={style.title}></div>
		<button onClick={usuarisClick}>Desfer Canvis</button>
		<button className="primary-button">Obrir Expedient</button>
	</NavBar>
}