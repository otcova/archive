export interface TabTemplate {
	title: string,
	ContentClass: () => JSX.Element,
}

export interface Tab extends TabTemplate {
	id: number
}

