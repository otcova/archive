
import style from "../styles/buttons.module.css"
import { useEvent } from "../utils/useEvent"

type Props = {
	txt: string,
	shortCut?: string, // ctrl a, alt r, ctrl alt n
	style?: number,
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
		className={[style.normal, style.important, style.repetitive][props.style || 0]}
		onClick={props.action}>
		{props.txt}
	</div>
}