import { JSX } from "preact";

import style from "./styles/nav-bar.module.css"

type Props = {
	children: JSX.Element[]
}

export function NavBar(props: Props) {
	return <div data-tauri-drag-region className={style.container}>
		{props.children}
	</div>
}