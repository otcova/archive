
import { Component, For, Show } from "solid-js"
import Button from "../../atoms/Button"
import style from "./ConfirmationPanel.module.sass"

export type ConfirmationPanelProps = {
	show: boolean
	text: Component | string,
	redButtons?: string[],
	buttons: string[],
	response: (date: string) => void
}

export function ConfirmationPanel(props: ConfirmationPanelProps) {

	const stopPropagation = event => event.stopPropagation()

	return <Show when={props.show}>
		<div class={style.container} onClick={stopPropagation} data-tauri-drag-region>
			<div class={style.panel}>
				<div class={style.text}>
					{typeof props.text == "string"? props.text : <props.text />}
				</div>
				<div class={style.buttons_row}>
					<For each={props.redButtons ?? []}>{text =>
						<Button text={text} red
							action={() => props.response(text)}
						/>
					}</For>
					<For each={props.buttons}>{text =>
						<Button text={text}
							{...(props.buttons.length == 1 ? { keyMap: "Enter" } : {})}
							action={() => props.response(text)}
						/>
					}</For>
				</div>
			</div>
		</div>
	</Show >
}