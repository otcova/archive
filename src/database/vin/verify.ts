export function verifyVIN(vin: string): boolean {
	if (vin.length > 17) return false
	return !vin.match(/(?!$|[A-HJ-NPR-Z\d])/i)
}