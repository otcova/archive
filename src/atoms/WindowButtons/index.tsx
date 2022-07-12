import IconButton from "../IconButton"
import style from "./WindowButtons.module.sass"
import { appWindow } from "@tauri-apps/api/window";

export default function WindowButtons() {
	return <div class={style.container}>
		<IconButton icon="minimize" action={() => appWindow.minimize()}/>
		<IconButton icon="close" action={() => window.close()}/>
	</div>
}