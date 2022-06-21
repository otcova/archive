import style from "../styles/buttons.module.css"
import { appWindow } from "@tauri-apps/api/window"
import { closeApp } from "../main"

export function CloseWindow() {
	return <div className={"expY row"}>
		<div className={style.windowClose} onClick={() => closeApp()}></div>
	</div>
}
export function MinimizeWindow() {
	return <div className={"expY row"}>
		<div className={style.windowMinimize} onClick={() => appWindow.minimize()}></div>
	</div>
}