import { appWindow } from "@tauri-apps/api/window"
import { Component, createEffect, createSignal, Show } from "solid-js"
import DropDownMenu from "../../atoms/DropDown"
import IconButton from "../../atoms/IconButton"
import { databaseDir, saveAndCloseApp } from "../../database/databaseState"
import { deleteRepeated } from "../../database/expedientState"
import { canLoadApp, setShowUpdatePanel } from "../../pages/UpdatePanel"
import { ConfirmationPanel } from "../ConfirmationPanel"
import Statistics from "./Statistics"
import style from "./WindowButtons.module.sass"


type PanelConfig = {
	show: boolean;
	text: Component | string;
	red_buttons: string[];
	buttons: string[];
	response: (button: string) => void;
};

export default function WindowButtons() {
	const [panel, setPanel] = createSignal<PanelConfig>({
		show: false,
		text: "...",
		red_buttons: ["Cancelar"],
		buttons: ["Confirmar"],
		response: () => setPanel({ ...panel(), show: false }),
	})

	function open_statistics() {
		createEffect(() => {
			setPanel(panel => ({
				show: true,
				text: Statistics,
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

	function delete_repetits() {
		deleteRepeated().then(count => {
			setPanel({
				show: true,
				text: "Repetits: " + count + "\n",
				red_buttons: [],
				buttons: ["Continuar"],
				response: () => setPanel({ ...panel(), show: false }),
			})
		})
	}

	return <div class={style.container}>
		<Show when={canLoadApp()}>
			<DropDownMenu options={[
				["Versió", () => setShowUpdatePanel(true)],
				["Estadístiques", open_statistics],
				["Copies de seguretat", open_security_copies],
				["Elimina Repetits", delete_repetits],
			]} />
		</Show>
		<IconButton icon="minimize" action={() => appWindow.minimize()} />
		<IconButton icon="close" action={() => saveAndCloseApp()} />
		<ConfirmationPanel {...panel()} />
	</div>
}
