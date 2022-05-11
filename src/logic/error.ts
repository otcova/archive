export function error(where: string, msg: string) {
	throw Error(`[${where}] ${msg}`)
}