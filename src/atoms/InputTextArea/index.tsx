import { createEffect, createSignal, on, untrack } from "solid-js"
import style from "./InputTextArea.module.sass"

type Props = {
	value?: string,
	onChange?: (data: string) => void,
	placeholder?: string,
	noStyle?: boolean,
}
export default function InputTextArea(props: Props) {
	let pastValue = props.value

	const onEditorLoad = (editableDiv: HTMLElement) => {
		const format = () => format_textarea(editableDiv, props.placeholder)
		setTimeout(format)

		const updateData = () => {
			format()
			const innerText = readInnerText(editableDiv)
			if (props.onChange && pastValue != innerText) {
				pastValue = innerText
				props.onChange(innerText)
			}
		}

		editableDiv.addEventListener("keydown", _ => setTimeout(updateData))

		editableDiv.addEventListener('paste', (event) => {
			event.preventDefault()
			let paste = paste_format(event.clipboardData.getData("text"))
			const selection = window.getSelection()
			if (!selection.rangeCount) return
			selection.deleteFromDocument()
			selection.getRangeAt(0).insertNode(document.createTextNode(paste))
			selection.collapseToEnd()
		})

		createEffect(() => {
			if (readInnerText(editableDiv) == props.value) return
			editableDiv.innerHTML = props.value
			format()
		})
	}

	const onMouseDown = (event: MouseEvent) => {
		if (event.button == 2) event.preventDefault()
	}

	return <div class={props.noStyle ? style.container_minimal : style.container}>
		<div
			class={style.editor + (props.noStyle ? " " + style.editor_minimal : "")}
			contentEditable={true}
			spellcheck={false}
			data-placeholder={props.placeholder}
			ref={onEditorLoad}
			onMouseDown={onMouseDown}
		>
			{pastValue}
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

	// Convert every element to div
	for (const line of element.children) {
		if (line.tagName != "DIV") {
			let div = document.createElement("div")
			div.appendChild(group_into_fragment(line.childNodes))
			line.replaceWith(div)
		}
	}

	// Separate lines into different divs
	for (const line of element.children) {
		let text_lines = line.textContent.replace("\r", "").split("\n")
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

function readInnerText(element: Element): string {
	return [...element.children].map(line => line.textContent).join("\n")
}