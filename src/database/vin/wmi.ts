import raw_wmi from "./wmi.txt?raw"

const wmi = new Map()

for (const model of raw_wmi.replace("\r", "").split('\n')) {
	const [code, name] = model.split("\t")
	wmi.set(code, name)
}

export function modelName(vin: string): string {
	return wmi.get(vin.substring(0, 3)) ?? wmi.get(vin.substring(0, 2))
}