import { appWindow } from "@tauri-apps/api/window"
import { createEffect, createSignal } from "solid-js"
import DropDownMenu from "../../atoms/DropDown"
import IconButton from "../../atoms/IconButton"
import { databaseDir, saveAndCloseApp } from "../../database/databaseState"
import { countOrders } from "../../database/expedientState"
import { currentVersion, shouldUpdate } from "../../pages/UpdatePanel"
import { ConfirmationPanel } from "../ConfirmationPanel"
import style from "./WindowButtons.module.sass"

export default function WindowButtons() {

	const [panel, setPanel] = createSignal({
		show: false,
		text: "...",
		red_buttons: ["Cancelar"],
		buttons: ["Confirmar"],
		response: () => setPanel({ ...panel(), show: false }),
	})

	function open_version() {
		if (shouldUpdate()) {
			setPanel({
				show: true,
				text: `Vesió actual:  ${currentVersion()}\nNova versió:   ${shouldUpdate()}`,
				red_buttons: ["Cancelar"],
				buttons: ["Actualitzar"],
				response: () => setPanel({ ...panel(), show: false }),
			})
		} else {
			setPanel({
				show: true,
				text: "Versió Actual:  " + currentVersion(),
				red_buttons: [], // ["Instalar versiò anterior"],
				buttons: ["Continuar"],
				response: () => setPanel({ ...panel(), show: false }),
			})
		}
	}


	function open_statistics() {
		const [ordersCount, setOrdersCount] = createSignal<string | number>("...")
		countOrders().then(count => {
			let formated = count + ""
			formated = formated.replace(/(.{1,3})(?=(...)+$)/g, "$1")
			setOrdersCount(formated)
		})
		createEffect(() => {
			setPanel(panel => ({
				show: true,
				text: `Comandes:  ${ordersCount()}`,
				red_buttons: [],
				buttons: ["Continuar"],
				response: () => setPanel({ ...panel, show: false }),
			}))
		})
	}

	function open_security_copies() {
		setPanel({
			show: true,
			text: "Les dades del programa es poden trobar en\n" + databaseDir,
			red_buttons: [],
			buttons: ["Continuar"],
			response: () => setPanel({ ...panel(), show: false }),
		})
	}

	return <div class={style.container}>
		<DropDownMenu options={[
			["Versió", open_version],
			["Estadístiques", open_statistics],
			["Copies de seguretat", open_security_copies]
		]} />
		<IconButton icon="minimize" action={() => appWindow.minimize()} />
		<IconButton icon="close" action={() => saveAndCloseApp()} />
		<ConfirmationPanel {...panel()} />
	</div>
}

