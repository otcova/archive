import { invoke } from "@tauri-apps/api"
import { relaunch } from "@tauri-apps/api/process"
import { installUpdate } from "@tauri-apps/api/updater"
import { databaseDir, trySaveDatabase } from "./databaseState"

export async function updateApp() {
	try {
		await trySaveDatabase()
		await installUpdate()
		await relaunch()
	} catch (error) {
		console.error("Error on updateApp", error)
	}
}

export async function installPreviousVersion(reportState?: (state: string) => void) {
	let path = databaseDir + "\\archive.msi"
	try {
		reportState?.("Descarregant")
		await invoke("download_previous_version", {downloadPath: path})
		reportState?.("Instalant")
		await trySaveDatabase()
		await invoke("install_archive_msi", {path})
	} catch (error) {
		console.error("Error on installPreviousVersion ", error)
	}
}