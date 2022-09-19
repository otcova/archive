import { useTab } from "../../templates/TabSystem";
import { bindKey, KeyMap } from "../../utils/bindKey";
import style from "./Button.module.sass";

type Props = {
	text: string,
	red?: boolean,
	keyMap?: KeyMap,
	style?: number,
	action?: () => any,
}

export default function Button(props: Props) {

	const tabCtx = useTab()
	if (props.keyMap) bindKey(document, props.keyMap, () => {
		if (!tabCtx || tabCtx.isActive()) props.action()
		else return "propagate"
	})

	return <div class={props.red ? style.button_red : style.button} onMouseUp={props.action}>
		{props.text}
	</div>
}