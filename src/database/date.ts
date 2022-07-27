export type UtcDate = {
	timespan: number
}

export function compareUtcDate(a: UtcDate, b: UtcDate) {
	if (a.timespan > b.timespan) return 1
	if (a.timespan < b.timespan) return -1
	return 0
}

export function utcToJsDate(utcDate: UtcDate): Date {
	return new Date(utcDate.timespan)
}

export function jsDateToUtc(jsDate: Date): UtcDate {
	return { timespan: jsDate.getTime() }
}

export function utcDateToString(utcDate: UtcDate): string {
	const jsDate = utcToJsDate(utcDate)
	const today = utcDateNow()
	if (equalDay(utcDate, today)) {
		if (jsDate.getHours() < 14) return "Matí"
		else return "Tarda"
	} else if (equalDay(utcDate, yesterdayOf(today))) {
		return "Ahir"
	} else if (equalDay(yesterdayOf(utcDate), today)) {
		return "Demà"
	}
	return jsDate.getDate() + " - " + (jsDate.getMonth() + 1) + " - " + jsDate.getFullYear()
}

export function utcDateNow(): UtcDate {
	return jsDateToUtc(new Date())
}

export function utcDateFuture(): UtcDate {
	return { timespan: 1e15 }
}

export function equalDay(utcA: UtcDate, utcB: UtcDate) {
	const a = utcToJsDate(utcA)
	const b = utcToJsDate(utcB)
	return a.toLocaleDateString() == b.toLocaleDateString()
}

export function yesterdayOf(utcDate: UtcDate) {
	return addHours(utcDate, -24)
}

export function addHours(utcDate: UtcDate, hours: number): UtcDate {
	let date = utcToJsDate(utcDate)
	date.setHours(date.getHours() + hours)
	return jsDateToUtc(date)
}