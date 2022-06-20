import { createContext } from "preact"
import { useContext, useRef, useState } from "preact/hooks"
import { Button } from "../components/button"
import { CheckBox } from "../components/checkBox"
import { ImgButton } from "../components/imgButton"
import style from "../styles/expedientList.module.css"

type ButtonTemplate = {
	txt: string,
	action: () => any,
}

type Props = {
	button?: ButtonTemplate,
}

export const ExpedientListContext = createContext<Props>(undefined!);

export function ExpedientList(props: Props) {
	return <div className={style.container}>
		<ExpedientListContext.Provider value={props}>
			<ExpedientRow />
			<ExpedientRow />
			<ExpedientRow />
			<ExpedientRow />
			<ExpedientRow />
			<ExpedientRow />
		</ExpedientListContext.Provider>
	</div>
}

export function ExpedientRow() {
	let [expanded, setExpanded] = useState(false)

	return <div className={style.row_container} onClick={() => setExpanded(e => !e)}>
		<ExpedientRowTitle />
		<ExpedientRowContent expanded={expanded} />
	</div>
}

export function ExpedientRowContent({ expanded }: { expanded: boolean }) {
	let rendered = useRef(false)
	if (expanded) rendered.current = true

	return <>{
		rendered.current && <div className={style.row_content} style={{ height: "0px" }}>
			<div ref={child => requestAnimationFrame(() => {
				let parent = child?.parentElement
				if (parent && child)
					parent.style.height = expanded ? child.offsetHeight + "px" : "0px"
			})}>
				<div className={style.row_description}>{test}</div>
				<OrderRow />
			</div>
		</div >
	}</>
}

export function ExpedientRowTitle() {

	let { button } = useContext(ExpedientListContext)

	return <div className={style.row}>
		<CheckBox />
		<div className="">1</div>
		<div className="expX">Maria Delgada</div>
		<div className="expX">Chevrolette</div>
		<div className="">8481 GRA</div>
		<ImgButton img={0} action={() => { }} />
		{button && <Button txt={button.txt} style={2} action={button.action} />}
		<ImgButton img={1} action={() => { }} />
	</div>
}

export function OrderRow() {
	return <div className={style.row_order}>
		<CheckBox />
		<div>Some text</div>
	</div>
}

let test = "Industry's standard dummy text ever since the 1500s. When an unkandard dummy text ever since the 1500s. When an unkandard dummy text ever since the 1500s. When an unknown printer a galley of type and scrambled, an unknown."