import style from "./InputTextArea.module.sass"

type Props = {
	defaultValue?: string,
	placeholder?: string,
	noStyle?: boolean,
}

export default function InputTextArea(props: Props) {
	const onEditorLoad = editableDiv => {

		editableDiv.onkeydown = _ => setTimeout(() =>
			format_textarea(editableDiv, props.placeholder))
		setTimeout(() => format_textarea(editableDiv, props.placeholder))


		editableDiv.addEventListener('paste', (event) => {
			event.preventDefault()
			let paste = paste_format(event.clipboardData.getData("text"))
			const selection = window.getSelection()
			if (!selection.rangeCount) return
			selection.deleteFromDocument()
			selection.getRangeAt(0).insertNode(document.createTextNode(paste))
			selection.collapseToEnd()
		})
	}

	return <div class={props.noStyle? style.container_minimal : style.container}>
		<div
			class={style.editor + (props.noStyle? " " + style.editor_minimal : "")}
			contentEditable={true}
			spellcheck={false}
			data-placeholder={props.placeholder}
			ref={onEditorLoad}
		>
			{props.defaultValue}
		</div>
	</div>
}

function paste_format(text: string): string {
	if (text.includes("\t")) {
		let lines = text.split("\n")
		let convertedCount = 0
		for (let i = 0; i < lines.length - 1; ++i) {
			const line = lines[i]
			if (line.startsWith("# ")) return text
			if (!line.match(/\t|\./) && line.match(/[a-zA-Z]/)) {
				lines[i] = "# " + line
				++convertedCount
			}
		}
		if (lines.length / 2 >= convertedCount)
			return lines.join("\n")
	}
	return text
}

function format_textarea(element: HTMLElement, placeholder: string) {
	merge_inline_elements(element)
	wrap_text_inside_span(element)
	merge_inline_elements(element)
	delete_nested_parent_elements(element)

	// // Convert every element to div
	for (const line of element.children) {
		if (line.tagName != "DIV") {
			let div = document.createElement("div")
			div.appendChild(group_into_fragment(line.childNodes))
			line.replaceWith(div)
		}
	}

	// Separate lines into different divs
	for (const line of element.children) {

		let text_lines = line.textContent.split(/[\r\n]+/)
		if (text_lines.length > 1) {
			let div_list = new DocumentFragment()
			for (const line_text of text_lines) {
				let div = document.createElement("div")
				div_list.appendChild(div)
				div.textContent = line_text
			}
			line.replaceWith(div_list)
		}
	}

	// Clear class
	for (const line of element.children) {
		line.className = ""
	}

	// Set cdd: bold
	for (const line of element.children) {
		if (line.textContent.startsWith("# ")) {
			line.classList.add(style.bold)
		} else {
			line.classList.remove(style.bold)
		}
	}

	// Set css: not_first
	let index = 0
	for (const line of element.children) {
		if (index++) line.classList.add(style.not_first)
		else line.classList.remove(style.not_first)
	}

	// Test placeholder
	if (element.children.length == 0 || (element.children.length == 1 &&
		(element.children[0] as HTMLElement).innerText == "")) {
		placeholder = placeholder.replaceAll("'", "\\'")
		element.innerHTML = `<div class="${style.placeholder}" data-placeholder='${placeholder}'></div>`
		return
	}
}

function merge_inline_elements(element: Element) {
	let elements = [...element.children]
	for (let i = 0; i < elements.length; ++i) {
		const item = elements[i]
		elements = [...elements, ...item.children]
		if (!item.previousSibling) continue
		if (getComputedStyle(item).display == "inline") {
			if (item.previousSibling.nodeName == "#text") {
				// merge item with text
				const parent = item.parentNode
				const fragment = new DocumentFragment()
				item.childNodes.forEach(element => fragment.appendChild(element))
				item.replaceWith(fragment)
				parent.normalize()
			} else if (item.previousSibling.nodeType == 1 &&
				"inline" == getComputedStyle(item.previousSibling as Element).display) {
				// merge item with element
				const parent = item.parentNode
				const sibling = item.previousSibling as Element
				sibling.replaceWith(group_into_fragment(sibling.childNodes))
				item.replaceWith(group_into_fragment(item.childNodes))
				parent.normalize()
			}
		}
	}
}

function wrap_text_inside_span(element: Element) {
	let elements = [element]
	for (let i = 0; i < elements.length; ++i) {
		const item = elements[i]
		elements = [...elements, ...item.children]
		if (item.children.length != item.childNodes.length &&
			(item == element || item.children.length > 0)) {
			const nodes = Array.from(item.childNodes).filter(node =>
				!Array.from(item.children).includes(node as Element)
			)
			for (const text_node of nodes) {
				const span = document.createElement("span")
				text_node.replaceWith(span)
				span.appendChild(text_node)
			}
		}
	}
}

function delete_nested_parent_elements(element: Element) {
	let elements = [...element.children]
	for (let i = 0; i < elements.length; ++i) {
		const item = elements[i]
		elements = [...elements, ...item.children]
		if (item.children.length > 0) {
			item.replaceWith(group_into_fragment(item.childNodes))
		}
	}
}

function group_into_fragment(nodes) {
	let fragment = new DocumentFragment()
	for (const node of [...nodes]) {
		fragment.appendChild(node)
	}
	return fragment
}

const textExample = `# Información general
Marca	BMW
Modelo	Serie 2
Generación	Serie 2 Gran Coupé (F44)
Modificación (motor)	220i (178 CV) xDrive Steptronic
Año de la puesta en producción	Marzo, 2021 años
Arquitectura de la unidad de potencia	Motor de combustión interna
Tipo de carrocería	Coupe
Numero de plazas	5
Numero de puertas	4
# Rendimiento
Consumo de combustible combinado (WLTP)	6.5-7.3 l/100 km
36.19 - 32.22 US mpg
43.46 - 38.7 UK mpg
15.38 - 13.7 km/l
Emisión CO2 Ponderada (WLTP)	149-165 gr/km
Consumo de combustible urbano (NEDC)	7.4-7.8 l/100 km
31.79 - 30.16 US mpg
38.17 - 36.22 UK mpg
13.51 - 12.82 km/l
Consumo de combustible extraurbano (NEDC)	5.2-5.6 l/100 km
45.23 - 42 US mpg
54.32 - 50.44 UK mpg
19.23 - 17.86 km/l
Consumo de combustible combinado (NEDC)	6.1-6.4 l/100 km
38.56 - 36.75 US mpg
46.31 - 44.14 UK mpg
16.39 - 15.63 km/l
Emisión CO2 Ponderada (NEDC)	139-147 gr/km
Combustible	Gasolina
Aceleración 0 - 100 km/h	7.1 s
Aceleración 0 - 62 mph	7.1 s
Aceleración 0 - 60 mph (Calculado por Auto-Data.net)	6.7 s
Velocidad máxima	234 km/h
145.4 mph
Clasificación de los gases de escape	Euro 6d
Relación peso/potencia	8.5 kg/CV, 117.1 CV/tonelada
Relación peso/Par	5.4 kg/Nm, 184.2 Nm/tonelada
2021 BMW Serie 2 Gran Coupé (F44) 220i (178 CV) xDrive Steptronic | Ficha técnica y consumo , Medidas: https://www.auto-data.net/es/bmw-2-series-gran-coupe-f44-220i-178hp-xdrive-steptronic-42769
`