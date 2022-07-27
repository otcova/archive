import IconButton from "../../atoms/IconButton"
import style from "./WindowButtons.module.sass"
import { appWindow } from "@tauri-apps/api/window"
import { saveAndCloseApp } from "../../database/databaseState"

export default function WindowButtons() {
	return <div class={style.container}>
		<IconButton icon="minimize" action={() => appWindow.minimize()} />
		<IconButton icon="close" action={() => saveAndCloseApp()} />
	</div>
}