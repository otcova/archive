import { error } from "./error";

export class Compra {
	readonly description: string = "";
	readonly checked: boolean = false;

	static safeCast(compra: any): Compra {
		const empty_compra = new Compra() as any
		for (const key in compra) {
			if (typeof empty_compra[key] != typeof compra[key])
				error("casting", "Unable to cast to Compra " + JSON.stringify(compra))
		}
		return Object.assign(new Compra(), compra)
	}
	
	check(check: boolean) {
		(this as any).checked = check
		this.updateDataBase()
	}
	
	updateDataBase() {
		console.log("TODO");
	}
}

export class Expedient {
	readonly id: number = 0;
	readonly users: string = "";
	readonly model: string = "";
	readonly matricula: string = "";
	readonly vin: string = "";
	readonly description: string = "";
	readonly compres: Compra[] = [];

	is_checked() {
		for (const compra of this.compres)
			if (!compra.checked) return false
		return true
	}

	static safeCast(exp: any): Expedient {
		const empty_exp = new Expedient() as any
		for (const key in exp) {
			if (typeof empty_exp[key] != typeof exp[key])
				error("casting", "Unable to cast to expedient " + JSON.stringify(exp))
		}
		
		const compres = []
		for (const compra of exp.compres)
			compres.push(Compra.safeCast(compra))
		exp.compres = compres

		return Object.assign(new Expedient(), exp)
	}
	
	hasFolder() {
		console.log("TODO")
		return false
	}
	openFolder() {
		console.log("TODO")
	}
	
	/** 
	 * If expedient is allready equal to check no changes are applied.
	 * Else all compres are set to true or first is set to false.
	 */
	check(check: boolean) {
		if (this.is_checked() == check) return
		if (check) {
			for (const compra of this.compres)
				(compra as any).checked = true
		} else if (this.compres.length > 0) {
			(this.compres[0] as any).checked = false
		}
		this.updateDataBase()
	}
	
	updateDataBase() {
		console.log("TODO");
	}
}