import { useEffect, useState } from "preact/compat";
import { CheckBox, NavBar } from "../../components";
import { ExpedientRow } from "../../components/ExpedientRow";
import style from "./cotxes.module.css";

export function CotxesBody() {
	
	return <div className={style.bodyContainer}>
		<ExpedientRow />
		<ExpedientRow />
		<ExpedientRow />
	</div>
}
