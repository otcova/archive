import { getVersion } from '@tauri-apps/api/app'
import { checkUpdate } from '@tauri-apps/api/updater'
import { createEffect, createSignal } from 'solid-js'
import { installPreviousVersion, updateApp } from '../../database/update'
import { ConfirmationPanel, ConfirmationPanelProps } from '../../templates/ConfirmationPanel'


const [_canLoadApp, setCanLoadApp] = createSignal(false)
export const canLoadApp = _canLoadApp

export const [currentVersion, setCurrentVersion] = createSignal("...")

getVersion().then(setCurrentVersion);

export const [shouldUpdate, setShouldUpdate] = createSignal<boolean | string>(null)
export const [showUpdatePanel, _setShowUpdatePanel] = createSignal(true)

export const setShowUpdatePanel = (show: boolean) => {
	if (show) _setShowUpdatePanel(show)
	else {
		setCanLoadApp(true)
		_setShowUpdatePanel(false)
	}
}

checkUpdate().then(({ shouldUpdate, manifest }) => {
	if (shouldUpdate) setShouldUpdate(manifest.version)
	else setShouldUpdate(false)
	setShowUpdatePanel(shouldUpdate)
})

export function UpdatePanel() {
	const [panel, setPanel] = createSignal<ConfirmationPanelProps>({
		show: true,
		text: "Comprobant versió actual...",
		buttons: [],
		response: () => { },
	})
	const [installing, setInstalling] = createSignal<false | string>(false)

	createEffect(() => {
		let instalLog = installing()
		if (instalLog) {
			setPanel({
				show: true,
				text: instalLog,
				buttons: [],
				response: () => { },
			})
		} else if (shouldUpdate()) {
			setPanel({
				show: true,
				text: `Vesió actual:  ${currentVersion()}\nNova versió:   ${shouldUpdate()}`,
				redButtons: ["Cancelar"],
				buttons: ["Actualitzar"],
				response: button => {
					if (button == "Cancelar") setShowUpdatePanel(false)
					else updateApp()
				}
			})
		} else if (shouldUpdate() === false) {
			setPanel({
				show: true,
				text: `Vesió actual:  ${currentVersion()}`,
				redButtons: ["Instalar versio anterior"],
				buttons: ["Continuar"],
				response: button => {
					if (button == "Continuar") setShowUpdatePanel(false)
					else {
						installPreviousVersion(setInstalling)
							.then(() => setShowUpdatePanel(false))
					}
				}
			})
		}
	})
	return <ConfirmationPanel {...panel()} />
}