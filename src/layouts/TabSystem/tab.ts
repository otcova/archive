export interface TabTemplate<Props> {
	title: string,
	ContentClass: (props: Props) => JSX.Element,
	props?: Props,
}

export interface Tab<Props> extends TabTemplate<Props> {
	id: number
}

