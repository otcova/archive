import { createEffect, createSignal, Index, JSX, onCleanup, Show } from "solid-js"
import style from "./ContextMenu.module.sass"

type Props = {
	buttons: { text: string, red?: boolean }[],
	onClick: (name: string, index: number) => void,
	children: JSX.Element,
}

let mousePos = [0, 0]

export function ContextMenu(props: Props) {
	const [show, setShow] = createSignal(false)
	let container: HTMLDivElement

	const closeContextMenu = () => setShow(false)
	const openContextMenu = () => {
		setShow(true)
		container.style.left = mousePos[0] + "px"
		container.style.top = mousePos[1] + "px"
		const style = getComputedStyle(container)
		if (parseFloat(style.right) < 0) {
			container.style.removeProperty("left")
			container.style.right = "0px"
		} if (parseFloat(style.bottom) < 0) {
			container.style.removeProperty("top")
			container.style.bottom = "0px"
		}
	}

	const onMouseUp = (event: MouseEvent) => {
		if (event.button == 2) {
			openContextMenu()
			event.stopPropagation()
		}
	}

	addEventListener("mouseup", closeContextMenu)
	document.addEventListener("mouseleave", closeContextMenu)
	onCleanup(() => {
		removeEventListener("mouseup", closeContextMenu)
		document.removeEventListener("mouseleave", closeContextMenu)
	})

	return <div class={style.container} onMouseUp={onMouseUp} onMouseLeave={closeContextMenu}>
		<Show when={show()}>
			<div class={style.contextMenu} ref={container}>
				<Index each={props.buttons}>{(button, index) =>
					<div class={button().red ? style.row_red : style.row}
						onMouseUp={() => props.onClick(button().text, index)}>
						{button().text}
					</div>
				}</Index>
			</div>
		</Show>
		{props.children}
	</div>
}

const updateMousePos = (event: MouseEvent) => {
	mousePos = [event.clientX, event.clientY]
}
addEventListener("mousedown", updateMousePos)
addEventListener("mouseenter", updateMousePos)
addEventListener("mouseleave", updateMousePos)
addEventListener("mousemove", updateMousePos)
addEventListener("mouseout", updateMousePos)
addEventListener("mouseover", updateMousePos)
addEventListener("mouseup", updateMousePos)