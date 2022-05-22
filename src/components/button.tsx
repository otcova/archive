
import style from "../styles/buttons.module.css"
import { useEvent } from "../utils/useEvent"

type Props = {
	txt: string,
	shortCut?: string, // ctrl a, alt r, ctrl alt n
	important?: boolean,
	action: () => any,
}

export function Button(props: Props) {
	
	useEvent("keydown", event => {
		if (!props.shortCut) return
		let cmd = props.shortCut.toLocaleUpperCase().trim()
		let cmdKey = cmd.split(/ +/g).pop();
		if (cmd.includes("CTRL") && !event.ctrlKey) return
		if (cmd.includes("ALT") && !event.altKey) return
		if (event.code == "Key" + cmdKey) props.action()
	})
	
	return <div
		className={props.important ? style.important : style.normal}
		onClick={props.action}>
		{props.txt}
	</div>
}