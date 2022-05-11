import { error } from "./error";
import { Expedient } from "./expedient";
// import { documentDir } from "@tauri-apps/api/path"

let id = 0;
export function nextId(): number {
	return id++;
}

export async function loadExpedients(next: (exp: Expedient) => boolean) {
	const data = await load(["expedients", { from: 0, to: 10 }])
	for (const exp of data)
		if (!next(Expedient.safeCast(exp))) return
}


type range = { from: number, to: number }
type direction = (string | number | range)
function openPath(obj: any, path: direction[]): any {
	const direction = path.shift()
	if (!direction) return obj
	if (obj == undefined) return openPath(obj, path)
	if (typeof direction == "object") {
		let bundle = []
		for (let i = direction.from; i <= direction.to; ++i) {
			if (obj[i]) bundle.push(openPath(obj[i], path))
		}
		return bundle
	}
	return openPath(obj[direction], path)
}


async function load(path: direction[]): Promise<any> {
	return openPath(database, path)
}

const database = {
	expedients: [
		{
			id: id++, users: "Pedro | Maria 631 829 123",
			model: "Mercedes SL 300 Gullwing",
			matricula: "3156 EOS", vin: "",
			description: "The industry's standard dummy text ever since the", compres: [
				{ description: "012 Ipsum has been the industry's standard dummy text ever since the 1500s, when an unknown printer took a galley of type and scrambled it to make.", checked: false },
			]
		},
		{
			id: id++, users: "Jose",
			model: "Chevrolet Corvette",
			matricula: "3451 RGY", vin: "",
			description: "", compres: [
				{ description: "Pastilles de Fre\n1 Lorem Ipsum has been the industry's standard dummy text ever since the.\nGalley of type and scrambled.", checked: false },
				{ description: "Lorem Ipsum has been the industry's standard dummy text ever since the 1500s, when an unknown printer took a galley of type and scrambled it to make.", checked: false },
				{ description: "Omg Ipsum has been the industry's standard dummy text ever since the 1500s.\nWhen an unknown printer took a galley of type and scrambled, an unknown printer took.\nAn unknown printer took it to make.", checked: false },
			]
		},
		{
			id: id++, users: "799 152 991",
			model: "Rolls-Royce Dawn Drophead",
			matricula: "6426 JUE", vin: "",
			description: "", compres: [
				{ description: "Galley Ipsum has been the industry's standard dummy text ever since the.\nGalley of type and scrambled.", checked: false },
			]
		},
	]
}