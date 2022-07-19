import { documentDir } from "@tauri-apps/api/path";
import { invoke } from "@tauri-apps/api/tauri";
import { appWindow } from "@tauri-apps/api/window";
import { databaseError, setDatabaseError } from ".";

setTimeout(async () => {
	await openDatabase()
	setInterval(storeDatabase, 1000 * 60)
}, 0)

const databaseDir = async () => ({ path: await documentDir() + "Archive" })

const criticalError = (error, msg?) => setDatabaseError({
	error,
	msg,
	button: "Tancar",
	action: closeApp,
})

const unclassifiedError = (error) => criticalError("Error", error)
const lockError = () => criticalError("L'aplicació només pot estar oberta un cop")

const requestCreateDatabase = () => setDatabaseError({
	msg: "No s'ha trobat cap base de dades",
	button: "Crear Base de Dades",
	action: createDatabase
})

async function openDatabase() {
	try {
		await invoke('open_database', await databaseDir())
		setDatabaseError(null)
	} catch (error) {
		switch (error) {
			case "NotFound": return requestCreateDatabase()
			case "Collision": return lockError()
			case "DataIsCorrupted": return loadRollbackInfo()
			case "AlreadyOpen":
				await invoke('release_all_hooks')
				return setDatabaseError(null)
		}
		unclassifiedError(error)
	}
}

async function createDatabase() {
	try {
		await invoke('create_database', await databaseDir())
		setDatabaseError(null)
	} catch (error) {
		switch (error) {
			case "AlreadyExists": return criticalError(
				"Error en crear base de dades",
				`La carpeta '${(await databaseDir()).path}' no està buida`
			)
			case "Collision": return lockError()
		}
		unclassifiedError(error)
	}
}

async function loadRollbackInfo() {
	setDatabaseError({
		error: "S'ha trobat informació corrupte a la base de dates",
		msg: "Buscant la copia de seguretat més recent no corrupte ...",
	})
	try {
		const info = await invoke("database_rollback_info", await databaseDir()) as any
		const { newestInstant, rollbackInstant } = info
		setDatabaseError({
			error: "S'ha trobat informació corrupte a la base de dades",
			msg: `Dades corruptes:   ${newestInstant}\nCopia de seguretat:   ${rollbackInstant}`,
			button: "Continuar a partir de la copia de seguretat",
			action: loadRollback,
		})
	} catch (error) {
		if (error == "NotFound") {
			return criticalError(
				"S'ha trobat informació corrupte a la base de dates",
				"No s'ha pogut recuperar cap copia de seguretat",
			)
		}
		unclassifiedError(error)
	}
}

async function loadRollback() {
	try {
		await invoke('rollback_database', { path: await databaseDir() })
		setDatabaseError(null)
	} catch (error) {
		switch (error) {
			case "NotFound": return criticalError("No s'ha pogut recuperar cap copia de seguretat")
			case "Collision": return lockError()
		}
		unclassifiedError(error)
	}
}

async function storeDatabase() {
	if (!databaseError()) {
		try { await invoke('store_database') }
		catch (error) {
			try { await invoke('store_database') }
			catch (error) { criticalError("Error en guardar les dades!!!", error) }
		}
	}
}

export async function closeApp() {
	await storeDatabase();
	appWindow.close();
}