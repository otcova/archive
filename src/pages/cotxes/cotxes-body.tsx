import { useEffect, useState } from "preact/hooks";
import { ExpedientRow } from "../../components/ExpedientRow";
import { loadExpedients } from "../../utils/database";
import { Expedient } from "../../utils/expedient";
import { Filter } from "./cotxes-bar";
import style from "./cotxes.module.css";

type Props = {
	filter: Filter,
}

export function CotxesBody({ filter }: Props) {

	const [expedients, setExpedients] = useState<Expedient[]>([])

	const showExp = (exp: Expedient) => exp.is_checked() == filter.checked
	const filterOutExp = () => setExpedients(
		exps => exps.filter(exp => showExp(exp))
	)

	useEffect(() => {
		setExpedients([])
		loadExpedients(exp => {
			if (expedients.length > 3) return false
			if (showExp(exp)) setExpedients(exps => [...exps, exp])
			return true
		})
	}, [filter])

	return <div className={style.bodyContainer}>
		{
			expedients.map(exp => <ExpedientRow
				key={exp.id}
				expedient={exp}
				onChange={filterOutExp} />)
		}
	</div>
}
