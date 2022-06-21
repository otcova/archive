import { documentDir } from '@tauri-apps/api/path'
import { invoke } from '@tauri-apps/api/tauri'
import { updateScene } from './app';
import { closeApp } from './main';

let databaseState: String = ""
let rollbackInfo: null | {
	newest_instant: string,
	rollback_instant: string,
} = null;
let errorDescription = "";
openDatabase();
setInterval(storeDatabase, 60 * 1000);

async function databaseDir() {
	return (await documentDir()) + "Archive"
}


export async function openDatabase() {
	if (databaseState == "Database Open") throw Error("Database is already open!!!")
	try {
		await invoke('open_database', { path: await databaseDir() })
		databaseState = "Database Open"
		await invoke('store_database')
	} catch (error) {
		if (error == "NotFound") databaseState = "Database Not Found"
		else if (error == "Collision") databaseState = "Database Lock Collision"
		else if (error == "Corrupted") databaseState = "Database Is Corrupted"
		else databaseState = "" + error
	}
	updateScene()

	if (databaseState == "Database Is Corrupted (with rollback info)") {
		rollbackInfo = await invoke("database_rollback_info", { path: await databaseDir() })
		updateScene()
	}
}

export async function createDatabase() {
	if (databaseState == "Database Open") throw Error("Database is already open!!!")

	try {
		await invoke('create_database', { path: await databaseDir() })
		databaseState = "Database Open"
	} catch (error) {
		if (error == "AlreadyExist") databaseState = "Database Already Exists"
		else if (error == "Collision") databaseState = "Database Lock Collision"
		else databaseState = "" + error
	}
	updateScene()
}

export async function rollbackDatabase() {
	if (databaseState == "Database Open") throw Error("Database is already open!!!")

	try {
		await invoke('rollback_database', { path: await databaseDir() })
		databaseState = "Database Open"
	} catch (error) {
		if (error == "NotFound") databaseState = "Backup Not Found"
		else if (error == "Collision") databaseState = "Database Lock Collision"
		else databaseState = "" + error
	}
	updateScene()
}

export async function storeDatabase() {
	if (databaseState == "Database Open") {
		try {
			await invoke('store_database')
		} catch (error) {
			try {
				await invoke('store_database')
			} catch (error) {
				databaseState = "Database store has failed"
				errorDescription = "" + error
				updateScene()
			}
		}
	}
}

///////////////////// ERROR MANAGE ///////////////////////
export type ErrorLog = {
	errorMsg: string,
	msg: string,
	button: string,
	action: () => any,
}

export function currentErrorLog(): ErrorLog | null {
	switch (databaseState) {
		case "Database Open": return null
		case "": return {
			errorMsg: "",
			msg: "Opening Database ...",
			button: "",
			action: () => { },
		}
		case "Database Is Corrupted": return {
			errorMsg: "Hem trobat dades danyades  !!!",
			msg: "Estem buscant la copia de seguratat més recent no danyada.",
			button: "",
			action: () => { },
		}
		case "Database Is Corrupted (with rollback info)": return {
			errorMsg: "Hem trobat dades danyades  !!!",
			msg: "Ultim cop guardat:   {date}\nCopia de seguretat:   {date}",
			button: "Continuar a partir de la copia de seguretat",
			action: () => rollbackDatabase(),
		}
		case "Database Lock Collision": return {
			errorMsg: "L'aplicació només pot estar oberta un cop",
			msg: "",
			button: "Tancar Aplicació",
			action: () => closeApp(),
		}
		case "Database Not Found": return {
			errorMsg: "",
			msg: "No s'ha trobat cap basse de dades.",
			button: "Crear basse de dades",
			action: () => createDatabase(),
		}
		case "Database Already Exists": return {
			errorMsg: "ERROR",
			msg: "No s'ha pogut crear la base de dades perquè ja hi havien fitxers.",
			button: "Tancar Aplicació",
			action: () => closeApp(),
		}
		case "Buckup Not Found": return {
			errorMsg: "ERROR",
			msg: "No s'han pogut recuperar les dades perquè totes les còpies de seguretat estan danyades.",
			button: "Tancar Aplicació",
			action: () => closeApp(),
		}
		case "Database store has failed": return {
			errorMsg: "Error en guardar les dades!!!",
			msg: errorDescription,
			button: "Tancar Aplicació",
			action: () => closeApp(),
		}
	}
	return {
		errorMsg: "ERROR",
		msg: "" + databaseState,
		button: "",
		action: () => { },
	}
}