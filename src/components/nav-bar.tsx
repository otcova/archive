import { JSX } from "preact";
import style from "./styles/nav-bar.module.css"
import { appWindow } from '@tauri-apps/api/window'

type Props = {
	children: JSX.Element[]
}

export function NavBar(props: Props) {
	return <div data-tauri-drag-region className={style.container}>
		{props.children}
		<div className={style.minimize} onClick={() => appWindow.minimize()}></div>
		<div className={style.close} onClick={() => appWindow.close()}></div>
	</div>
}