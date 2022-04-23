import { useState } from "preact/compat"
import { ExpedientBar } from "./ExpedientBar"
import { ExpedientCompres } from "./ExpedientCompres"
import { ExpedientData } from "./ExpedientData"
import style from "./expedient.module.css"

export function PageExpedient() {
	
	return <>
		<ExpedientBar />
		<div className={style.body}>
			<ExpedientData />
			<ExpedientCompres />
		</div>
	</>
}
