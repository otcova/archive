import { useState } from "preact/compat"
import { Compra, Expedient } from "../logic/expedient"
import { CheckBox } from "./checkbox"
import style from "./styles/expedientRow.module.css"
import openIcon from "./img/openIcon"
import folderIcon from "./img/folderIcon"

type Props = {
	expedient: Expedient,
	onChange: (expedient: Expedient) => void
}

export function ExpedientRow({ expedient, onChange }: Props) {

	const [open, setOpen] = useState(false)

	const checkbox_changed = (checked: boolean) => {
		expedient.check(checked)
		onChange(expedient)
	}
	return <div className={style.container}>
		<div className={style.header} onClick={() => setOpen(!open)}>
			<CheckBox checked={expedient.is_checked()} onChange={checkbox_changed} />
			<div>{expedient.compres.length}</div>
			<div className={style.users}>{expedient.users}</div>
			<div className={style.model}>{expedient.model}</div>
			<div>{expedient.matricula}</div>
			{
				expedient.hasFolder() ?
					<div onClick={e => { e.stopPropagation(); expedient.openFolder() }} className="hand">{folderIcon}</div>
					: <div style={{opacity: 0}}>{folderIcon}</div>
			}
			<div onClick={e => { e.stopPropagation(); }} className="hand">{openIcon}</div>
		</div>
		{
			open &&
			<>
				<div className={style.description}>{expedient.description}</div>
				<div className={style.compresContainer}>
					{
						expedient.compres.map((compra, i) =>
							<CompraRow
								key={i}
								compra={compra}
								onChange={() => onChange(expedient)} />)
					}
				</div>
			</>
		}
	</div>
}

type CompraProps = {
	compra: Compra,
	onChange: (compra: Compra) => void
}

function CompraRow({ compra, onChange }: CompraProps) {

	const checkbox_changed = (checked: boolean) => {
		compra.check(checked)
		onChange(compra)
	}

	return <div className={style.compraRow}>
		<CheckBox checked={compra.checked} onChange={checkbox_changed} />
		<div className={style.compraDescription}>{compra.description}</div>
	</div>
}