import { Button } from "../components/button";
import style from "../styles/expedientTab.module.css"

type ButtonTemplate = {
	txt: string,
	action: () => any,
}

type Props = {
	buttons: ButtonTemplate[],
}

export function BottomButtons(props: Props) {
	let mainButton = props.buttons.shift();
	return <div className={style.body}>
		{
			props.buttons.map(btn =>
				<Button key={btn.txt} txt={btn.txt} action={btn.action} />
			)
		}
		<div className="expX"></div>
		{mainButton && <Button txt={mainButton.txt} style={1} action={mainButton.action} />}
	</div>
}