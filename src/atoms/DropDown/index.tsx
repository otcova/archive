import { createEffect, createSignal, For } from "solid-js";
import style from "./DropDownMenu.module.sass";

type Props = {
	title?: string,
	options: [string, () => void][],
}

export default function DropDownMenu(props: Props) {

	const [active, setActive] = createSignal(false);
	document.onclick = () => setActive(false)

	let options_container: HTMLDivElement, options_sized_container: HTMLDivElement
	createEffect(() => {
		if (active()) {
			options_container.style.height = options_sized_container.offsetHeight + "px"
			options_container.style.borderWidth = "1px"
		} else {
			options_container.style.removeProperty("height")
			options_container.style.removeProperty("border-width")
		}
	})

	return <div class={style.container}>
		<div class={style.button + (active() ? " " + style.button_active : "")}
			onClick={e => {
				setActive(true)
				e.stopImmediatePropagation()
			}}>
			{props.title ?? ""}
		</div>
		<div ref={options_container}
			class={style.option_container + (active() ? " " + style.option_active : "")}>
			<div ref={options_sized_container}>
				<For each={props.options}>{option =>
					<div class={style.option} onClick={option[1]}>{option[0]}</div>
				}</For>
			</div>
		</div>
	</div>
}